use chrono::{Local, NaiveDate, TimeZone};
use datos_comunes::{CrearUsuarioError, QueryRegistrarUsuario, ResponseRegistrarUsuario};
use reqwasm::http::Request;
use web_sys::{FormData, HtmlFormElement};
use wasm_bindgen::JsCast;
use yew::{platform::spawn_local, prelude::*};
use serde_json::json;
use yew_router::components::Link;
use yew_router::prelude::use_navigator;

use crate::router::Route;


#[function_component(RegisterMolecule)]
pub fn register_molecule()-> Html {
    let navigator = use_navigator().unwrap();
    let error_state = use_state(|| {"".to_string()});
    let cloned_error_state = error_state.clone();
    
    let onsubmit = Callback::from(move |event:SubmitEvent|{
        let navigator = navigator.clone();
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
        let cloned_error_state = cloned_error_state.clone();
        spawn_local(async move {
            log::info!("query de registro: {query:?}");
            let respuesta = Request::post("/api/registrar_usuario")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&query).unwrap())
                .send().await;
        
            match respuesta {
                Ok(resp) => {
                    let resp:Result<ResponseRegistrarUsuario, reqwasm::Error> = resp.json().await;
                    match resp {
                        Ok(resp) => {
                            match resp {
                                Ok(resp)=>{
                                    navigator.push(&Route::LogInPage)
                                }
                                Err(CrearUsuarioError::DNIExistente)=>{
                                    let cloned_error_state = cloned_error_state.clone();
                                    cloned_error_state.set("El DNI ingresado ya se encuentra registrado.".to_string());
                                }
                                Err(CrearUsuarioError::EmailExistente)=>{
                                    let cloned_error_state = cloned_error_state.clone();
                                    cloned_error_state.set("El correo electrónico ingresado ya se encuentra registrado.".to_string());
                                    
                                }
                                Err(CrearUsuarioError::ErrorIndeterminado)=>{
                                    let cloned_error_state = cloned_error_state.clone();
                                    cloned_error_state.set("ERROR INDETERMINADO".to_string());
                                    
                                }
                                Err(CrearUsuarioError::MenorA18)=>{
                                    let cloned_error_state = cloned_error_state.clone();
                                    cloned_error_state.set("Para registrarte debes ser mayor de edad.".to_string());
                                }
                            }
                        }
                        Err(error) => {
                            log::error!("error en deserializacion{:?}",error);
                            ()
                        },
                    }
                },
                Err(_) =>{
                    log::error!("error en llamada al backend");
                    ()
                } 
            };

            
        });



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
            if !(&*error_state).is_empty(){
                <h2 class="error-text">
                    {&*error_state}
                </h2>
            }
            
            <span> {"¿Ya tienes usuario? "} </span>
            <Link<Route> to={Route::LogInPage}>{"Iniciar Sesion"}</Link<Route>>
        </div>
        </>
    }
}
