use datos_comunes::{LogInError, QueryLogin, ResponseLogIn};
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use crate::store::UserStore;
use crate::Components::generic_button::GenericButton;
use crate::Components::generic_input_field::GenericInputField;
use crate::Components::dni_input_field::DniInputField;
use wasm_bindgen_futures::spawn_local;
use crate::router::Route;
use yew_router::prelude::*;
use yewdux::prelude::*;


#[derive(Default)]
pub struct State{
    pub username:String,
    pub password:String,
}

#[derive(Serialize,Deserialize)]
pub struct AuthResponse{
    pub data:User
}

#[derive(Serialize,Deserialize)]
pub struct User{
    pub id:u32,
    pub username:String,
    pub token:String,
}

#[function_component(LogInMolecule)]
pub fn log_in_molecule()-> Html{


    let dni_state:UseStateHandle<u64> = use_state(|| 0);
    let cloned_dni_state = dni_state.clone();
    let dni_changed = Callback::from(move |dni:String|{
            cloned_dni_state.set(dni.parse::<u64>().unwrap());
    });
    

    let error_state = use_state(|| "".to_string());
    let cloned_error_state = error_state.clone();


    let password_state = use_state(|| "no password yet".to_owned());
    let cloned_password_state = password_state.clone();
    let password_changed = Callback::from(move |password|{
        cloned_password_state.set(password);
    });

    let login_response = use_state(|| "false".to_string());
    let login_response_c = login_response.clone();
    let cloned_dni_state = dni_state.clone();
    let cloned_password_state = password_state.clone();
    let navigator = use_navigator().unwrap();


    let (_store, dispatch) = use_store::<UserStore>();


    let dispatch_cloned = dispatch.clone();


    let submit_clicked_example = Callback::from(move |()| {

        let cloned_error_state = cloned_error_state.clone();

        let dispatch_cloned = dispatch_cloned.clone();
        let login_response_c = login_response_c.clone();
        {
            let cloned_error_state = cloned_error_state.clone();
            let dni = &*cloned_dni_state;
            let password = &*cloned_password_state;
            {
                let cloned_error_state = cloned_error_state.clone();
                let dni = dni.clone();
                let password = password.clone();
                let navigator = navigator.clone();
                let dispatch_cloned = dispatch_cloned.clone();
                spawn_local(async move {
                        log::info!("entre al spawn local");
                        let cloned_error_state = cloned_error_state.clone();
                        let query = QueryLogin{dni:dni.clone(), password: password.clone()};
                        let respuesta = Request::post("/api/check_login").header("Content-Type", "application/json").body(serde_json::to_string(&query).unwrap()).send().await;
                        let cloned_dni = dni.clone();
                        match respuesta{
                            Ok(respuesta) =>{
                                let response:Result<ResponseLogIn, reqwasm::Error> = respuesta.json().await;
                                log::info!("deserailice la respuesta {:?}",response);
                                match response{
                                    Ok(respuesta) => {
                                        match respuesta{
                                            Ok(respuesta) => {
                                                let status = respuesta.status;
                                                if status == true{
                                                    dispatch_cloned.reduce_mut(|store|{
                                                        store.dni = Some(dni);
                                                        store.login_fail = false;
                                                    });
                                                    navigator.push(&Route::Home);

                                                } else{
                                                    log::error!("UNREACHABLE CODE");
                                                }
                                                
                                            }
                                            Err(error) => {
                                                match error{
                                                    LogInError::UserNotFound => {
                                                        let cloned_error_state = cloned_error_state.clone();
                                                        cloned_error_state.set("El usuario ingresado no existe, si no tienes cuenta registrate.".to_string());
                                                        
                                                    }
                                                    LogInError::BlockedUser => {
                                                        let cloned_error_state = cloned_error_state.clone();
                                                        cloned_error_state.set("Usuario bloqueado, comunicarse con personal.".to_string());
                                                    }
                                                    LogInError::IncorrectPassword{intentos} => {
                                                        let cloned_error_state  = cloned_error_state.clone();
                                                        let intentos_restantes = intentos;
                                                        cloned_error_state.set(format!("Contraseña incorrecta, intentos restantes: {}", intentos_restantes));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(error)=>{
                                        log::error!("Error en deserializacion: {}", error);
                                    }
                                }
                            }
                            Err(error)=>{
                                log::error!("Error en llamada al backend: {}", error);

                            }
                        }
  
                    })     
            }
        }
    });


    let onsubmit = Callback::from(move |event:SubmitEvent|{
        event.prevent_default();
    });


    let (store, _dispatch) = use_store::<UserStore>();


    html! {
        <div class="login-box">
            <h1>{"Login"}</h1>
            <section>
                <div>
                    <form {onsubmit}>
                        <DniInputField dni = "dni" label="Dni" tipo = "number" handle_on_change = {dni_changed} />
                        <GenericInputField name = "password" label="Password" tipo = "password" handle_on_change = {password_changed} />
                        <GenericButton text = "submit" onclick_event = {submit_clicked_example} />
                        if !(&*error_state).is_empty(){
                            <div class="error-text">
                                <h2>{&*error_state}</h2>
                            </div>
                        }
                    </form>
                </div>
            </section>
        </div>
    }

}

