
use axum::body::{Body};
use axum::http::{Response, StatusCode};
use axum::extract::{Query, State};
use axum::routing::post;
use axum::Json;
use axum::{response::IntoResponse, routing::get, Router};
use chrono::{DateTime, Local, TimeZone};
use clap::Parser;
use database::Database;
use datos_comunes::*;
use tokio::fs;
use tokio::net::TcpListener;
use tokio::sync::{Mutex, RwLock};
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use serde::{Serialize, Deserialize};
use crate::database::usuario::Usuario;

use crate::state::ServerState;
mod database;
mod state;

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
        .route("/api/check_login", get(check_login))
        .route("/api/usuario_existe", get(usuario_existe))
        .route("/api/registrar_usuario", post(registrar_usuario))
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

#[derive(Deserialize)]
struct QueryLogin {
    dni: u32,
    password: String,
}

//  fedeteria.com/api/check_login?username=algo&password=otracosa

async fn check_login(datos_login: Query<QueryLogin>) -> impl IntoResponse {
    let datos_login = datos_login.0;
    let username = datos_login.dni;
    let password = datos_login.password;
    if username == 44933855 && &password == &"beiser" {
        log::info!("TRUE!!!!!!!!!!!!!!!!!");

        return "true"
    }
    log::info!("FALSE!");
    return "false"
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
        log::info!("Usuario creado: {:?}", state.db.get_ultimo_usuario());
        log::error!("FALTA ENVIAR MAIL");
    }
    
    let res = Json(res);
    log::info!("{res:?}");
    res
}