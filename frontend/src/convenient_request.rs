use reqwasm::http::Request;
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen_futures::spawn_local;

// Request de tipo POST, con una query en json y respuesta en json.
pub fn request_post<Q, R>(
    url: &'static str,
    query: Q,
    on_success: impl FnOnce(R) + 'static,
) where 
    Q: Serialize + 'static,
    R: DeserializeOwned,
{
    spawn_local(async move {
        let respuesta = Request::post(url)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&query).unwrap())
            .send().await;

        match respuesta {
            Ok(respuesta) => {
                log::info!("Convenient Request recibió {respuesta:?}");
                let respuesta: Result<R, reqwasm::Error> = respuesta.json().await;
                match respuesta{
                    Ok(respuesta) => {
                            on_success(respuesta);
                    }
                    Err(error)=>{
                        log::error!("Error en deserializacion: {}", error);
                    }
                }
            }
            Err(error)=> {
                log::error!("Error en llamada al backend: {}", error);
            }
        }
    });
}

// Request de tipo GET, sin query de ningun tipo y respuesta en tipo json
pub fn request_get<R>(
    url: &'static str,
    on_success: impl FnOnce(R) + 'static,
) where
    R: DeserializeOwned
{
    spawn_local(async move {
        let respuesta = Request::get(url).send().await;

        match respuesta {
            Ok(respuesta) => {
                log::info!("Convenient Request recibió {respuesta:?}");
                let respuesta: Result<R, reqwasm::Error> = respuesta.json().await;
                match respuesta{
                    Ok(respuesta) => {
                            on_success(respuesta);
                    }
                    Err(error)=>{
                        log::error!("Error en deserializacion: {}", error);
                    }
                }
            }
            Err(error)=> {
                log::error!("Error en llamada al backend: {}", error);
            }
        }
    });
}