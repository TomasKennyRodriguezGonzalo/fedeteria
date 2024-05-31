use chrono::{Local, NaiveDate, TimeZone};
use datos_comunes::{CrearUsuarioError, QueryRegistrarUsuario, ResponseGetOffices, ResponseRegistrarUsuario, Sucursal};
use reqwasm::http::Request;
use web_sys::{FormData, HtmlFormElement, HtmlInputElement};
use wasm_bindgen::JsCast;
use yew::{platform::spawn_local, prelude::*};
use yew_hooks::use_effect_once;
use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

use crate::information_store::InformationStore;
use crate::router::Route;
use crate::components::checked_input_field::CheckedInputField;

#[function_component(RegisterMolecule)]
pub fn register_molecule()-> Html {
    let navigator = use_navigator().unwrap();

    let name_state = use_state(|| {"".to_string()});
    let dni_state = use_state(|| {0});
    let mail_state = use_state(|| {"".to_string()});
    let password_state = use_state(|| {"".to_string()});
    let date_state = use_state(|| {"".to_string()});

    let name_state_cloned = name_state.clone();
    let name_changed = Callback::from(move |name| {
        name_state_cloned.set(name);
    });

    let date_state_cloned = date_state.clone();
    let date_changed = Callback::from(move |event : Event| {
        let target = event.target().unwrap();
        let input:HtmlInputElement = target.unchecked_into();
        let input_value = input.value();
        date_state_cloned.set(input_value);
    });
    
    let mail_state_cloned = mail_state.clone();
    let mail_changed = Callback::from(move |mail| {
        mail_state_cloned.set(mail);
    });
    
    let password_state_cloned = password_state.clone();
    let password_changed = Callback::from(move |password| {
        password_state_cloned.set(password)
    });
    
    let dni_state_cloned = dni_state.clone();
    let dni_changed = Callback::from(move |event : Event| {
        let target = event.target().unwrap();
        let input: HtmlInputElement = target.unchecked_into();
        dni_state_cloned.set(input.value().parse::<u64>().expect("error dni parse"));
    });
    
 
    let loading_state = use_state(|| false);

    let error_state = use_state(|| {"".to_string()});
    let cloned_error_state = error_state.clone();
    
    let (_information_store, information_dispatch) = use_store::<InformationStore>();
    let information_dispatch = information_dispatch.clone();
    let loading_state_cloned = loading_state.clone();
    let onsubmit = Callback::from(move |event:SubmitEvent|{
        let loading_state = loading_state_cloned.clone();
        loading_state.set(true);
        log::info!("Loading started");
        let information_dispatch = information_dispatch.clone();
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
        let loading_state = loading_state.clone();
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
                                Ok(_)=>{
                                    navigator.push(&Route::LogInPage);
                                    information_dispatch.reduce_mut(|store| store.messages.push("Registraste tu usuario correctamente.".to_string()))
                                }
                                Err(CrearUsuarioError::DNIExistente)=>{
                                    let cloned_error_state = cloned_error_state.clone();
                                    cloned_error_state.set("El DNI ingresado ya pertenece a una cuenta.".to_string());
                                }
                                Err(CrearUsuarioError::EmailExistente)=>{
                                    let cloned_error_state = cloned_error_state.clone();
                                    cloned_error_state.set("El correo electrónico ingresado ya pertenece a una cuenta.".to_string());
                                    
                                }
                                Err(CrearUsuarioError::ErrorIndeterminado)=>{
                                    let cloned_error_state = cloned_error_state.clone();
                                    cloned_error_state.set("ERROR INDETERMINADO".to_string());
                                    
                                }
                                Err(CrearUsuarioError::MenorA18)=>{
                                    let cloned_error_state = cloned_error_state.clone();
                                    cloned_error_state.set("Para registrarte debes ser mayor de 18 años.".to_string());
                                }
                            }
                        }
                        Err(error) => {
                            log::error!("error en deserializacion{:?}",error);
                        },
                    }
                },
                Err(_) =>{
                    log::error!("error en llamada al backend");
                } 
            }; 
            loading_state.set(false);
            log::info!("Loading finished");
        });
    });


    let state_office_list:UseStateHandle<Vec<Sucursal>> = use_state(||Vec::new());
    let state_office_list_clone = state_office_list.clone();

    use_effect_once(move || {
        let state_office_list_clone = state_office_list_clone.clone();
        spawn_local(async move {
            let state_office_list_clone = state_office_list_clone.clone();
            log::info!("entre al spawn local");
            let respuesta = Request::get("/api/obtener_sucursales")
            .header("Content-Type", "application/json")
            .send()
            .await;
        match respuesta{
            Ok(respuesta) =>{
                let response:Result<ResponseGetOffices, reqwasm::Error> = respuesta.json().await;
                log::info!("deserailice la respuesta {:?}",response);
                match response{
                    Ok(respuesta) => {           
                        state_office_list_clone.set(respuesta.office_list);
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
        });   
    

        ||{}


    });


    html! {
        <>
        <div class = "login-box">
            <h1> {"Registrarse"} </h1>
            <form {onsubmit}>
                <CheckedInputField name = "nombre" label="Nombre:" tipo = "text" on_change = {name_changed} />
                
                <div>
                <label> {"DNI:"} </label>
                </div>
                <input type="number" name="dni" min="0" onchange={dni_changed}/>
                
                <CheckedInputField name = "email" label="Correo electrónico:" tipo = "email" on_change = {mail_changed} />

                <CheckedInputField name = "contraseña" label="Contraseña:" tipo = "password" on_change = {password_changed} />

                <div>
                    <label> {"Fecha de nacimiento:"} </label>
                </div>
                <input type="date" name="nacimiento" onchange={date_changed}/>
                <br/>
                if !(name_state.is_empty()) && !(password_state.is_empty()) && (*dni_state != 0) && !(mail_state.is_empty()) && !(&*date_state).is_empty() {
                    <input type="submit" value="Confirmar"/>
                } else {
                    <a class="disabled-dyn-element">{"Confirmar"}</a>
                }
            </form>
            if !error_state.is_empty(){
            <h2 class="error-text">
                {&*error_state}
            </h2>
            }
            if (&*loading_state).clone() {
                <div class="loading">
                </div>
            }
        </div>
        </>
    }
}
