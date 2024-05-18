
use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::extract::{Multipart, Query, Request, State};
use axum::routing::post;
use axum::Json;
use axum::{body::Bytes, BoxError};
use axum::{response::IntoResponse, routing::get, Router};
use futures::{Stream, TryStreamExt};
use clap::Parser;
//use axum::debug_handler;
use database::usuario::EstadoCuenta;
use database::Database;
use datos_comunes::*;
use tokio::fs::{self, File};
use tokio::io::BufWriter;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use std::hash::{Hash, Hasher, DefaultHasher};
use std::{io, path};
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use serde::Deserialize;
use tokio_util::io::StreamReader;

use crate::database::publicacion::{self, Publicacion};
use crate::mail::send_email;
use crate::state::ServerState;
mod database;
mod state;
mod mail;

// Setup the command line interface with clap.
#[derive(Parser, Debug)]
#[clap(name = "server", about = "A server for our wasm project!")]
struct Opt {
    /// set the log level
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,

    /// set the listen addr
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    addr: String,

    /// set the listen port
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,

    /// set the directory where static files are to be found
    #[clap(long = "static-dir", default_value = "./dist")]
    static_dir: String,
}

type SharedState = Arc<RwLock<ServerState>>;

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    // Setup logging & RUST_LOG from args
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }
    // enable console logging
    tracing_subscriber::fmt::init();

    let db = Database::cargar();
    let state = Arc::new(RwLock::new(ServerState::new(db)));
    let router = Router::new()
        .route("/api/hello", get(hello))
        .route("/api/check_login", post(check_login))
        .route("/api/usuario_existe", get(usuario_existe))
        .route("/api/registrar_usuario", post(registrar_usuario))
        .route("/api/retornar_usuario", post(retornar_usuario))
        .route("/api/eliminar_sucursal", post(eliminar_sucursal))
        .route("/api/obtener_sucursales", get(obtener_sucursales))
        .route("/api/obtener_rol", post(obtener_rol))
        .route("/api/crear_publicacion", post(crear_publicacion))
        .route("/api/get_user_info", post(get_user_info))
        .fallback(get(|req| async move {
            let res = ServeDir::new(&opt.static_dir).oneshot(req).await;
            match res {
                Ok(res) => {
                    let status = res.status();
                    match status {
                        StatusCode::NOT_FOUND => {
                            let index_path = PathBuf::from(&opt.static_dir).join("index.html");
                            let index_content = match fs::read_to_string(index_path).await {
                                Err(_) => {
                                    return Response::builder()
                                        .status(StatusCode::NOT_FOUND)
                                        .body(Body::from("index file not found"))
                                        .unwrap()
                                }
                                Ok(index_content) => index_content,
                            };
    
                            Response::builder()
                                .status(StatusCode::OK)
                                .body(Body::from(index_content))
                                .unwrap()
                        }
                        _ => {
                            res.into_response()
                        },
                    }
                },
                Err(_) => panic!(),
            }
        }))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));


    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));
    let listener = TcpListener::bind(sock_addr).await.unwrap();

    log::info!("listening on http://{}", sock_addr);

    axum::serve(listener, router.with_state(state).into_make_service())
        .await
        .expect("Unable to start server");
}

async fn hello() -> impl IntoResponse {
    "hello from server!"
}

//  fedeteria.com/api/check_login?username=algo&password=otracosa
async fn check_login(
    State(state): State<SharedState>,
    Json(query): Json<QueryLogin>,
    ) -> Json<ResponseLogIn> {
    let dni = query.dni;
    let password = query.password;
    let mut state = state.write().await;
    let index = state.db.encontrar_dni(dni);
    match index{
        Some(index) =>{
            let user = state.db.obtener_datos_usuario(index);
            if user.estado == EstadoCuenta::Bloqueada{
                return Json(Err(LogInError::BlockedUser ))
            }
            if user.contraseña == hash_str(&password){
                state.db.resetear_intentos(index);
                Json(Ok(ResponseStatus{status : true}))
            } else{
                let res =state.db.decrementar_intentos(index);
                match res{
                    Ok(intentos) =>{
                        Json(Err(LogInError::IncorrectPassword {intentos}))
                    }
                    Err(_) => {
                        Json(Err(LogInError::BlockedUser ))
                    }
                }
            }
        }
        None => { 
            Json(Err(LogInError::UserNotFound))
        },
    }
}

#[derive(Deserialize)]
struct QueryDNI {
    dni: u64,
}
async fn usuario_existe(
    State(state): State<SharedState>,
    query: Query<QueryDNI>
) -> impl IntoResponse {
    let state = state.read().await;
    let dni = query.0.dni;
    let res = state.db.encontrar_dni(dni);
    if res.is_some() {
        "El usuario existe"
    } else {
        "El usuario no existe"
    }
}

async fn obtener_rol(
    State(state): State<SharedState>,
    Json(query): Json<QueryGetUserRole>
) -> Json<Option<ResponseGetUserRole>> {
    log::info!("Entré a obtener rol!");
    let state = state.read().await;
    log::info!("El dni recibido es: {}", query.dni);
    let indice = state.db.encontrar_dni(query.dni);
    log::info!("El indice es: {:?}", indice);
    if indice.is_some() {
        let rol_obtenido = state.db.obtener_rol_usuario(indice.unwrap());
        log::info!("El rol es: {:?}", rol_obtenido.clone());
        let respuesta = ResponseGetUserRole{rol:rol_obtenido.clone()};
        log::info!("La respuesta es: {:?}", respuesta.clone());
        return Json(Some(respuesta));
    }
    Json(None)
}

async fn retornar_usuario(
    State(state): State<SharedState>,
    Json(query): Json<QueryObtenerUsuario>
) -> Json<Option<ResponseObtenerUsuario>> {
    let state = state.write().await;
    log::info!("we are checking with {} dni ",query.dni.clone()); 
    let res = state.db.encontrar_dni(query.dni);
    if let Some(res) = res {
       let usuario = state.db.obtener_datos_usuario(res);
       let response = ResponseObtenerUsuario{nombre:usuario.nombre_y_apellido.clone()};
       log::info!("username found "); 
       Json(Some(response))
    } else{
        log::info!("username not found "); 
        Json(None)
    }
}

async fn registrar_usuario(
    State(state): State<SharedState>,
    Json(query): Json<QueryRegistrarUsuario>
) -> Json<ResponseRegistrarUsuario> {
    let mut state = state.write().await;
    let res = state.db.agregar_usuario(query);
    if res.is_ok() {
        let usuario = state.db.get_ultimo_usuario();
        log::info!("Usuario creado: {:?}", usuario);
        match send_email(usuario.nombre_y_apellido.to_string(), usuario.email.to_string(),
            "Registro en Fedeteria".to_string(),
            format!("Hola {}!\nUsted ha creado una cuenta en la página https://fedeteria.com, con el DNI {}.\n
Si cree que esto es un error, por favor contacte a un administrador.", usuario.nombre_y_apellido, usuario.dni)
        ).await {
            Ok(_) => log::info!("Mail enviado al usuario."),
            Err(_) => log::error!("Error al enviar mail."),
        }
    }
    
    let res = Json(res);
    log::info!("{res:?}");
    res
}


fn hash_str(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}


async fn eliminar_sucursal (
    State(state): State<SharedState>,
    Json(query): Json<QueryDeleteOffice>
) -> Json<ResponseDeleteOffice> {
    let mut state = state.write().await;
    let respuesta = ResponseDeleteOffice { respuesta: state.db.eliminar_sucursal(query) };
    Json(respuesta)
}

async fn obtener_sucursales (
    State(state): State<SharedState>,
) -> Json<ResponseGetOffices> {
    let state = state.read().await;
    let sucursales=state.db.obtener_sucursales();
    let respuesta = ResponseGetOffices{office_list : sucursales.clone()};
    Json(respuesta)
 
}

async fn crear_publicacion (
    State(state): State<SharedState>,
    // req: Request,
    mut multipart: Multipart,
) -> Result<String, ()> {
    // log::info!("La request es: {req:?}");
    log::info!("Recibido mensaje de crear publicacion!");
    let titulo = multipart.next_field().await.unwrap().unwrap();
    assert_eq!(titulo.name().unwrap(), "Titulo");
    let titulo = titulo.text().await.unwrap();
    let descripcion = multipart.next_field().await.unwrap().unwrap();
    assert_eq!(descripcion.name().unwrap(), "Descripción");
    let descripcion = descripcion.text().await.unwrap();
    multipart.next_field().await.unwrap().unwrap();
    let dni = multipart.next_field().await.unwrap().unwrap();
    assert_eq!(dni.name().unwrap(), "dni");
    let dni_str = dni.text().await.unwrap();
    let dni: u64 = dni_str.parse().unwrap();

    log::info!("Dni: {dni}, titulo: {titulo}, descripcion: {descripcion}");
    let mut imagenes = vec![];
    while let Ok(Some(field)) = multipart.next_field().await {
        let file_name = if let Some(file_name) = field.file_name() {
            file_name.to_owned()
        } else {
            log::info!("Recibido field de otro tipo, nombre: {:?}", field.name());
            continue;
        };
        log::info!("Recibido archivo: {file_name}");
        let path = Path::new(database::IMGS_DIR).join(&dni_str);
        std::fs::create_dir_all(&path).unwrap();
        let path = path.join(file_name);
        imagenes.push(path.to_str().unwrap().to_string());
        stream_to_file(path, field).await.unwrap();
    }
    let publicacion = Publicacion::new(titulo, descripcion, imagenes, dni);
    let mut state = state.write().await;
    state.db.agregar_publicacion(publicacion);
    Ok("HOLO".to_string())
}

async fn stream_to_file<S, E>(path: PathBuf, stream: S) -> Result<(), (StatusCode, String)>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<BoxError>,
{
    async {
        // Convert the stream into an `AsyncRead`.
        let body_with_io_error = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        let body_reader = StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);

        // Create the file. `File` implements `AsyncWrite`.
        log::info!("Guardando archivo en {:?}", path);
        let mut file = BufWriter::new(File::create(path).await?);

        // Copy the body into the file.
        tokio::io::copy(&mut body_reader, &mut file).await?;

        Ok::<_, io::Error>(())
    }
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}



async fn get_user_info( State(state): State<SharedState>,
Json(query): Json<QueryGetUserInfo>
) -> Json<Option<ResponseGetUserInfo>>{
    let state = state.read().await;
    let res = state.db.encontrar_dni(query.dni);
    if let Some(res) = res {
       let usuario = state.db.obtener_datos_usuario(res);
       let response = ResponseGetUserInfo{nombre_y_ap:usuario.nombre_y_apellido.clone(), email:usuario.email.clone(), nacimiento:usuario.nacimiento.clone() };
       log::info!("username found "); 
       Json(Some(response))
    } else{
        log::info!("username not found "); 
        Json(None)
    }


}



