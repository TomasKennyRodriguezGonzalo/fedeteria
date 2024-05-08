
use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::extract::{Query, State};
use axum::routing::post;
use axum::Json;
use axum::{response::IntoResponse, routing::get, Router};
use clap::Parser;
//use axum::debug_handler;
use database::usuario::EstadoCuenta;
use database::Database;
use datos_comunes::*;
use tokio::fs;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use std::hash::{Hash, Hasher, DefaultHasher};
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use serde::Deserialize;

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
                return Json(Ok(ResponseStatus{status : true}))
            } else{
                let res =state.db.decrementar_intentos(index);
                match res{
                    Ok(intentos) =>{
                        return Json(Err(LogInError::IncorrectPassword {intentos: intentos}))
                    }
                    Err(_) => {
                        return Json(Err(LogInError::BlockedUser ))
                    }
                }
            }
        }
        None => { 
            return Json(Err(LogInError::UserNotFound))
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

async fn retornar_usuario(
    State(state): State<SharedState>,
    Json(query): Json<QueryObtenerUsuario>
) -> Json<Option<ResponseObtenerUsuario>> {
    let mut state = state.write().await;
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

fn hash_str(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}


