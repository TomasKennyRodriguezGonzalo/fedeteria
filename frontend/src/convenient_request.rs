use datos_comunes::{QueryEnviarNotificacion, ResponseEnviarNotificacion};
use reqwasm::http::Request;
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen_futures::spawn_local;

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


pub fn send_notification(titulo:String, detalle:String, url:String, dni:u64){
    
    let query= QueryEnviarNotificacion {
        dni: dni,
        titulo: titulo,
        detalle: detalle,
        url: url,
    };

    request_post("/api/enviar_notificacion", query, |respuesta:ResponseEnviarNotificacion|{

    });



}
