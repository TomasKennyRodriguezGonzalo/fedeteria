
use axum::body::{Body};
use axum::http::{Response, StatusCode};
use axum::extract::Query;
use axum::{response::IntoResponse, routing::get, Router};
use clap::Parser;
use tokio::fs;
use tokio::net::TcpListener;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use serde::{Serialize, Deserialize};

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

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    // Setup logging & RUST_LOG from args
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }
    // enable console logging
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/api/hello", get(hello))
        .route("/api/check_login", get(check_login))
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
        // .route("/", get(|req| async move {
        //     ServeDir::new(opt.static_dir).oneshot(req).await.unwrap()
        // }))
        // .fallback_service(ServeDir::new("public").not_found_service(ServeFile::new("public/index.html")),
        // )
        // .fallback_service(get(|req| async move {
        //     match ServeDir::new(opt.static_dir).oneshot(req).await {
        //         Ok(res) => res.map(boxed),
        //         Err(err) => Response::builder()
        //             .status(StatusCode::INTERNAL_SERVER_ERROR)
        //             .body(Body::from(format!("error: {err}")))
        //             .expect("error response"),
        //     }
        // }))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));


    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));
    let listener = TcpListener::bind(sock_addr).await.unwrap();

    log::info!("listening on http://{}", sock_addr);

    axum::serve(listener, app.into_make_service())
        .await
        .expect("Unable to start server");
}

async fn hello() -> impl IntoResponse {
    "hello from server!"
}

#[derive(Deserialize)]
struct DatosLogin {
    username: String,
    password: String,
}

//  fedeteria.com/api/check_login?username=algo&password=otracosa

async fn check_login(datos_login: Query<DatosLogin>) -> impl IntoResponse {
    let datos_login = datos_login.0;
    let username = datos_login.username;
    let password = datos_login.password;
    if &username == "nico" && &password == &"beiser" {
        log::info!("TRUE!!!!!!!!!!!!!!!!!");

        return "true"
    }
    log::info!("FALSE!");
    return "false"
}