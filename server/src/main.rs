
use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::extract::{Multipart, Query, State};
use axum::routing::post;
use axum::Json;
use axum::{body::Bytes, BoxError};
use axum::{response::IntoResponse, routing::get, Router};
use futures::{Stream, TryStreamExt};
use clap::Parser;
use axum::debug_handler;
use database::usuario::EstadoCuenta;
use database::Database;
use datos_comunes::*;
use tokio::fs::{self, File};
use tokio::io::BufWriter;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use std::hash::{Hash, Hasher, DefaultHasher};
use std::io;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use serde::Deserialize;
use tokio_util::io::StreamReader;

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
        .route("/api/agregar_sucursal", post(agregar_sucursal))
        .route("/api/eliminar_sucursal", post(eliminar_sucursal))
        .route("/api/obtener_sucursales", get(obtener_sucursales))
        .route("/api/obtener_rol", post(obtener_rol))
        .route("/api/crear_publicacion", post(crear_publicacion))
        .route("/api/get_user_info", post(get_user_info))
        .route("/api/obtener_cuentas_bloqueadas", get(obtener_cuentas_bloqueadas))
        .route("/api/desbloquear_cuenta", post(desbloquear_cuenta))
        .route("/api/cambiar_rol_usuario", post(cambiar_rol_usuario))
        .route("/api/datos_publicacion", get(get_datos_publicacion))
        .nest_service("/publication_images", ServeDir::new("db/imgs"))
        .route("/api/cambiar_usuario", post(cambiar_usuario))
        .route("/api/alternar_pausa_publicacion", post(alternar_pausa_publicacion))
        .route("/api/obtener_publicaciones", post(obtener_publicaciones))
        .route("/api/eliminar_publicacion", post(eliminar_publicacion))
        .route("/api/obtener_notificaciones", post(obtener_notificaciones))
        .route("/api/datos_notificacion", post(get_notificacion))
        .route("/api/eliminar_notificacion", post(eliminar_notificacion))
        .route("/api/tasar_publicacion", post(tasar_publicacion))
        .route("/api/obtener_publicaciones_sin_tasar", post(obtener_publicaciones_sin_tasar))
        .route("/api/crear_oferta", post(crear_oferta))
        .route("/api/obtener_trueques", post(obtener_trueques))
        .route("/api/obtener_trueque", post(obtener_trueque))
        .route("/api/aceptar_oferta", post(aceptar_oferta))
        .route("/api/rechazar_oferta", post(rechazar_oferta))
        .route("/api/cancelar_oferta", post(cancelar_oferta))
        .route("/api/cambiar_trueque_a_definido", post(cambiar_trueque_a_definido))
        .route("/api/obtener_sucursal_por_dni", post(obtener_sucursal))
        .route("/api/obtener_trueque_por_codigos", post(obtener_trueque_por_codigos))
        .route("/api/finalizar_trueque", post(finalizar_trueque))
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
        ) {
            Ok(_) => log::info!("Mail enviado al usuario."),
            Err(_) => log::error!("Error al enviar mail."),
        }
    }
    
    let res = Json(res);
    log::info!("{res:?}");
    res
}

async fn agregar_sucursal (State(state): State<SharedState>,
Json(query): Json<QueryAddOffice>) ->  Json<ResponseAddOffice> {
    let mut state = state.write().await;
    let agrego = state.db.agregar_sucursal(query);
    let respuesta = ResponseAddOffice { respuesta: state.db.obtener_sucursales(), agrego };
    Json(respuesta) 
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
    
    let mut dni = multipart.next_field().await.unwrap().unwrap();
    if dni.name().unwrap() != "dni" {
        drop(dni);
        dni = multipart.next_field().await.unwrap().unwrap();
    }
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
        let path = path.join(&file_name);
        let relative_path = Path::new(&dni_str).join(&file_name);
        imagenes.push(relative_path.to_str().unwrap().to_string());
        stream_to_file(path, field).await.unwrap();
    }
    let publicacion = Publicacion::new(titulo, descripcion, imagenes, dni);
    let mut state = state.write().await;
    state.db.agregar_publicacion(publicacion);
    Ok("OK".to_string())
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

async fn get_datos_publicacion (
    State(state): State<SharedState>,
    Query(query): Query<QueryPublicacion>
) -> Json<ResponsePublicacion> {
    let id = query.id;
    let state = state.read().await;
    if let Some(publicacion) = state.db.get_publicacion(id) {
        Json(Ok(publicacion.clone()))
    } else {
        Json(Err(ErrorPublicacion::PublicacionInexistente))
    }
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

async fn cambiar_usuario( State(state): State<SharedState>,
Json(query): Json<QueryCambiarDatosUsuario>
) -> Json<ResponseCambiarDatosUsuario>{
    let mut state = state.write().await;
    let index = state.db.encontrar_dni(query.dni);
    if let Some(index) = index{
        let response = state.db.cambiar_usuario(
            index, query.full_name.clone(), query.email.clone(), query.born_date.clone()); 
        Json(response)
    } else {
        log::error!("Usuario no encontrado!");
        Json(Err(ErrorCambiarDatosUsuario::ErrorIndeterminado))
    }
}

async fn alternar_pausa_publicacion( State(state): State<SharedState>,
Json(query): Json<QueryTogglePublicationPause>
) -> Json<ResponseTogglePublicationPause>{
    let mut state = state.write().await;
    let id = query.id;
    state.db.alternar_pausa_publicacion(&id);
    Json(ResponseTogglePublicationPause{changed : true})
}


async fn obtener_publicaciones( 
    State(state): State<SharedState>,
    Json(query): Json<QueryPublicacionesFiltradas>
) -> Json<ResponsePublicacionesFiltradas>{
    let state = state.read().await;
    let response = state.db.obtener_publicaciones(query);
    Json(response)
}

async fn obtener_cuentas_bloqueadas (State(state): State<SharedState>
) -> Json<ResponseGetBlockedAccounts> {
    let state = state.read().await;
    let usuarios_bloqueados = state.db.obtener_usuarios_bloqueados();
    let respuesta = ResponseGetBlockedAccounts{ blocked_users: usuarios_bloqueados};
    Json(respuesta)
}

async fn desbloquear_cuenta (State(state): State<SharedState>, 
Json(query): Json<QueryUnlockAccount>) -> Json<ResponseUnlockAccount> {
    let mut state = state.write().await;
    let respuesta = state.db.desbloquear_cuenta(query);

    Json(respuesta)
}

async fn cambiar_rol_usuario (State(state): State<SharedState>,
Json(query): Json<QueryChangeUserRole>) -> Json<ResponseChangeUserRole>{
    let mut state = state.write().await;
    let respuesta = ResponseChangeUserRole { changed: state.db.cambiar_rol_usuario(query) };
    Json(respuesta)
}


async fn eliminar_publicacion (State(state): State<SharedState>,
Json(query): Json<QueryEliminarPublicacion>) -> Json<ResponseEliminarPublicacion>{
    let mut state = state.write().await;
    let respuesta = ResponseEliminarPublicacion { ok: state.db.eliminar_publicacion(query.id) };
    Json(respuesta)
}


async fn obtener_notificaciones( State(state): State<SharedState>,
Json(query): Json<QueryGetNotificaciones>
) -> Json<ResponseNotificaciones>{
    let mut state = state.write().await;
    let respuesta = state.db.obtener_notificaciones(&query);
    Json(ResponseNotificaciones{notificaciones: respuesta})
}

async fn get_notificacion( State(state): State<SharedState>,
Json(query): Json<QueryNotificacion>
) -> Json<ResponseNotificacion>{
    let state = state.write().await;
    let respuesta = state.db.get_notificacion(&query);
    Json(ResponseNotificacion{notificacion: respuesta})
}

async fn eliminar_notificacion( State(state): State<SharedState>,
Json(query): Json<QueryEliminarNotificacion>
) -> Json<ResponseEliminarNotificacion>{
    let mut state = state.write().await;
    let respuesta = state.db.eliminar_notificacion(&query);
    Json(ResponseEliminarNotificacion{notificaciones: respuesta})
}

async fn obtener_publicaciones_sin_tasar( State(state): State<SharedState>,
Json(query): Json<QueryPublicacionesSinTasar>
) -> Json<ResponsePublicacionesSinTasar>{
    let state = state.write().await;
    let respuesta = state.db.obtener_publicaciones_sin_tasar();
    Json(ResponsePublicacionesSinTasar{publicaciones: respuesta})
}

async fn tasar_publicacion( State(state): State<SharedState>,
Json(query): Json<QueryTasarPublicacion>
) -> Json<ResponseTasarPublicacion>{
    let mut state = state.write().await;
    let respuesta = state.db.tasar_publicacion(&query);
    if (respuesta) {
        let titulo = "Publicación tasada!".to_string();
        let publicacion = state.db.get_publicacion(query.id).unwrap();
        let dni_usuario_receptor = publicacion.dni_usuario;
        let indice_usuario_receptor = state.db.encontrar_dni(dni_usuario_receptor).unwrap();
        let detalle = format!(
            "Tu publicación ha sido tasada en un valor de {} pesos!, entrá al link para despausarla y empezar a recibir ofertas de trueque!",
            query.precio.unwrap()
        );
        let url = format!("/publicacion/{}", query.id);
        state.db.enviar_notificacion(indice_usuario_receptor, titulo, detalle, url);
    }
    Json(ResponseTasarPublicacion{tasado: respuesta})
}


async fn crear_oferta( State(state): State<SharedState>,
Json(query): Json<QueryCrearOferta>
) -> Json<ResponseCrearOferta>{
    let mut state = state.write().await;
    let respuesta = state.db.crear_oferta(query);

    if let Some(id) = respuesta {
        let oferta = state.db.get_trueque(id).unwrap();
        let publicacion_receptora = oferta.receptor.1;
        let publicacion_receptora = state.db.get_publicacion(publicacion_receptora).unwrap();
        let dni_receptor = oferta.receptor.0;
        let dni_ofertante = oferta.oferta.0;
        let indice_receptor = state.db.encontrar_dni(dni_receptor).unwrap();
        let titulo = "Nueva Oferta de Trueque!".to_string();
        let detalle = format!("Has recibido una oferta de trueque en tu {} presiona aquí para verla!", publicacion_receptora.titulo);
        let url = format!("/trueque/{id}");

        state.db.enviar_notificacion(indice_receptor, titulo, detalle, url);
    }
    Json(ResponseCrearOferta{estado: respuesta.is_some()})
}

async fn obtener_trueques ( State(state): State<SharedState>,
Json(query): Json<QueryTruequesFiltrados>
) -> Json<ResponseTruequesFiltrados> {
    let state = state.read().await;
    let respuesta = state.db.obtener_trueques(query);
    Json(respuesta)
}

async fn obtener_trueque ( State(state): State<SharedState>,
Json(query): Json<QueryObtenerTrueque>
) -> Json<ResponseObtenerTrueque> {
    let id = query.id;
    let state = state.read().await;
    if let Some(trueque) = state.db.get_trueque(id) {
        Json(Ok(trueque.clone()))
    } else {
        Json(Err(ErrorObtenerTrueque::TruequeInexistente))
    }
}



async fn aceptar_oferta( State(state): State<SharedState>,
Json(query): Json<QueryAceptarOferta>
) -> Json<ResponseAceptarOferta>{
    let id = query.id;
    let mut state = state.write().await;
    
    let respuesta = state.db.aceptar_oferta(id);
    let oferta = state.db.get_trueque(id).unwrap();
    let dni_receptor = oferta.receptor.0;
    let indice_receptor = state.db.encontrar_dni(dni_receptor).unwrap();
    let dni_ofertante = oferta.oferta.0;
    let indice_ofertante = state.db.encontrar_dni(dni_ofertante).unwrap();
    let receptor = state.db.obtener_datos_usuario(indice_receptor);
    let titulo = "Oferta Aceptada".to_string();
    let detalle = format!("{} ha aceptado tu oferta! presiona aquí para ver los detalles!", receptor.nombre_y_apellido);
    let url = format!("/trueque/{id}");

    state.db.enviar_notificacion(indice_ofertante, titulo, detalle, url);
    
    Json(ResponseAceptarOferta{aceptada : respuesta})

}


async fn rechazar_oferta( State(state): State<SharedState>,
Json(query): Json<QueryRechazarOferta>
) -> Json<ResponseRechazarOferta>{
    let id = query.id;
    let mut state = state.write().await;
    let oferta = state.db.get_trueque(id).unwrap();
    let dni_receptor = oferta.receptor.0;
    let indice_receptor = state.db.encontrar_dni(dni_receptor).unwrap();
    let dni_ofertante = oferta.oferta.0;
    let indice_ofertante = state.db.encontrar_dni(dni_ofertante).unwrap();
    let receptor = state.db.obtener_datos_usuario(indice_receptor);
    let titulo = "Oferta Rechazada".to_string();
    let detalle = format!("{} ha rechazado tu oferta :(",receptor.nombre_y_apellido);
    let url = format!("/trueque/{id}");
    let respuesta = state.db.rechazar_oferta(id);

    state.db.enviar_notificacion(indice_ofertante, titulo, detalle, url);
    log::info!("oferta rechazada notifiacion enviada a {}",dni_ofertante);
    Json(ResponseRechazarOferta{rechazada : respuesta})
}

async fn cancelar_oferta( State(state): State<SharedState>,
Json(query): Json<QueryRechazarOferta>
) -> Json<ResponseRechazarOferta>{
    let id = query.id;
    let mut state = state.write().await;
    let respuesta = state.db.rechazar_oferta(id);
    Json(ResponseRechazarOferta{rechazada : respuesta})
}

async fn cambiar_trueque_a_definido( State(state): State<SharedState>,
Json(query): Json<QueryCambiarTruequeADefinido>
) -> Json<ResponseCambiarTruequeADefinido>{

    let mut state = state.write().await;
    let respuesta = state.db.cambiar_trueque_a_definido(query);
    /* Contenido del Vec:
    0 --> Nombre Receptor
    1 --> Mail Receptor
    2 --> Mensaje Receptor
    3 --> Nombre Ofertante
    4 --> Mail Ofertante
    5 --> Mensaje Ofertante
    */
    if let Some(vec_string) = respuesta.1 {
        //envio mail a receptor
        match send_email(vec_string.get(0).unwrap().clone(), vec_string.get(1).unwrap().clone(),
                "Registro en Fedeteria".to_string(),
                vec_string.get(2).unwrap().clone()) {
                Ok(_) => log::info!("Mail enviado al receptor."),
                Err(_) => log::error!("Error al enviar mail al receptor."),
            }
        
        //envio mail al ofertante
        match send_email(vec_string.get(3).unwrap().clone(), vec_string.get(4).unwrap().clone(),
                "Registro en Fedeteria".to_string(),
                vec_string.get(5).unwrap().clone()) {
                Ok(_) => log::info!("Mail enviado al ofertante."),
                Err(_) => log::error!("Error al enviar mail al receptor."),
            }
    }
    Json(ResponseCambiarTruequeADefinido{cambiado : respuesta.0})

}

//devuelve solo el indice de uno, el deseado, obtenerlo en el front
async fn obtener_trueque_por_codigos ( State(state): State<SharedState>,
Json(query): Json<QueryTruequesFiltrados>
) -> Json<ResponseTruequePorCodigos> {
    let state = state.read().await;
    let respuesta = state.db.obtener_trueque_por_codigos(query);
    log::info!("VECTOR DE ID DE TRUEQUE: {:?}", respuesta);
    if respuesta.len() == 1 {
        return Json(ResponseTruequePorCodigos {trueque_encontrado: Some(respuesta)})
    }
    Json(ResponseTruequePorCodigos {trueque_encontrado: None})
}

//forma rara de obtener sucursal, mala mia (Franco), implemente mal el trueque, guardo en el trueque el string de sucursal
//en lugar del id, para la tercera demo si hago tiempo reacondiciono el tema ese
async fn obtener_sucursal (
    State(state): State<SharedState>,
    Json(query): Json<QueryGetOffice>
) -> Json<ResponseGetOffice> {
    let state = state.read().await;
    let indice_usuario = state.db.encontrar_dni(query.dni).unwrap();
    let rol = state.db.obtener_rol_usuario(indice_usuario);
    if let RolDeUsuario::Empleado { sucursal } = rol {
        let sucursal_empleado = state.db.obtener_sucursal(sucursal);
        return Json(ResponseGetOffice {sucursal: Some(sucursal_empleado)});
    }
    Json(ResponseGetOffice {sucursal: None})  
}

async fn finalizar_trueque (
    State(state): State<SharedState>,
    Json(query): Json<QueryFinishTrade>
) -> Json<ResponseFinishTrade> {
    let mut state = state.write().await;
    state.db.finalizar_trueque(query);
    Json(ResponseFinishTrade {respuesta: true})
}