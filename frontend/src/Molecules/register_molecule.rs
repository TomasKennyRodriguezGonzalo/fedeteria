use chrono::{Local, TimeZone};
use datos_comunes::{CrearUsuarioError, QueryRegistrarUsuario, ResponseRegistrarUsuario};
use reqwasm::http::Request;
use yew::{platform::spawn_local, prelude::*};
use serde_json::json;


#[function_component(RegisterMolecule)]
pub fn register_molecule()-> Html {
    let respuesta = use_state(|| "false".to_string());
    let respuesta_c = respuesta.clone();
    let submit_clicked_example = Callback::from(move |_| {
        let respuesta_c = respuesta_c.clone();
        {
            let query = QueryRegistrarUsuario {
                nombre_y_apellido: "Juan Pérez".to_string(),
                dni: 44933856,
                email: "JuanPerez@mail.com".to_string(),
                contraseña: "barraespaciadOOra".to_string(),
                nacimiento: Local.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap(),
            };
            spawn_local(async move {
                let respuesta = Request::post("/api/registrar_usuario")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&query).unwrap())
                    .send().await;

                let respuesta: ResponseRegistrarUsuario = match respuesta {
                    Ok(resp) => {
                        let resp = serde_json::from_str(&resp.text().await.unwrap());
                        match resp {
                            Ok(resp) => resp,
                            Err(_) => {
                                log::error!("Error al deserializar");
                                Err(CrearUsuarioError::ErrorIndeterminado)
                            },
                        }
                    },
                    Err(_) => Err(CrearUsuarioError::ErrorIndeterminado),
                };
                log::info!("Respuesta: {respuesta:?}");
                
            })
        }
    });
    html! {
        <>
            <h1> {"Registrarse"} </h1>
            <button onclick={move |_| submit_clicked_example.emit(())}> {"Submit"} </button>
            <span> {"¿Ya tienes usuario? "} </span> <a href="/login" value="Redirect"> {"Ingresa a tu cuenta."} </a>
        </>
    }
}