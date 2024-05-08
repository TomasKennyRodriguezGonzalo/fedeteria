use chrono::{Local, NaiveDate, TimeZone};
use datos_comunes::{CrearUsuarioError, QueryRegistrarUsuario, ResponseRegistrarUsuario};
use reqwasm::http::Request;
use web_sys::{FormData, HtmlFormElement};
use wasm_bindgen::JsCast;
use yew::{platform::spawn_local, prelude::*};
use serde_json::json;
use yew_router::components::Link;

use crate::router::Route;

#[function_component(RegisterMolecule)]
pub fn register_molecule()-> Html {

    let onsubmit = Callback::from(move |event:SubmitEvent|{
        event.prevent_default();
        let target = event.target();
        let form = target.and_then(|t| t.dyn_into::<HtmlFormElement>().ok()).unwrap();

        let form_data = FormData::new_with_form(&form).unwrap();

        let dni: f64 = form_data.get("dni").try_into().unwrap();
        let dni = dni as u64;

        let str_nacimiento: String = form_data.get("nacimiento").try_into().unwrap();
        let fecha = NaiveDate::parse_from_str(&str_nacimiento, "%Y-%m-%d").unwrap();
        let nacimiento = Local.from_local_datetime(&fecha.into()).unwrap();
        let query = QueryRegistrarUsuario {
            nombre_y_apellido: form_data.get("nombre").try_into().unwrap(),
            dni,
            email: form_data.get("email").try_into().unwrap(),
            contraseña: form_data.get("contraseña").try_into().unwrap(),
            nacimiento,
        };

        let respuesta = get_respuesta(query);

    });
    html! {
        <>
        <div class = "login-box">
            <h1> {"Registrarse"} </h1>
            <form {onsubmit}>
                <label> {"Nombre completo:"} </label>
                <input type="text" name="nombre" />
                <br />
                
                <label> {"DNI:"} </label>
                <input type="number" name="dni" min="0"/>
                <br />
                
                <label> {"Correo:"} </label>
                <input type="email" name="email" />
                <br />
                
                <label> {"Contraseña:"} </label>
                <input type="password" name="contraseña" />
                <br />
                
                <label> {"Fecha de nacimiento:"} </label>
                <input type="date" name="nacimiento" />
                <br />
            
                <input type="submit" value="Confirmar" />
            </form>
            // <div>
            
            <span> {"¿Ya tienes usuario? "} </span>
            <Link<Route> to={Route::LogInPage}>{"Iniciar Sesion"}</Link<Route>>
            </div>
        </>
    }
}

async fn get_respuesta(query: QueryRegistrarUsuario) -> ResponseRegistrarUsuario {
    log::info!("query de registro: {query:?}");
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
                    Err(CrearUsuarioError::ErrorIndeterminado)
                },
            }
        },
        Err(_) => Err(CrearUsuarioError::ErrorIndeterminado),
    };
    respuesta   
}