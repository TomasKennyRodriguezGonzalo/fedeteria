use std::str::FromStr;

use gloo::console::log;
use web_sys::window;
use yew::prelude::*;
use yew_router::{hooks::use_navigator, navigator};
use yewdux::{dispatch, prelude::*};
extern crate chrono;
use chrono::prelude::*;
use crate::components::generic_button::_Props::onclick_event;
use crate::{router::Route, store::UserStore};
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::Request;
use datos_comunes::{self, QueryCambiarDatosUsuario, QueryGetUserInfo, ResponseCambiarDatosUsuario, ResponseGetUserInfo};
use yew_router::prelude::Link;
use crate::pages::profile_page::User;
use crate::components::checked_input_field::CheckedInputField;
use crate::molecules::confirm_prompt_button_molecule::ConfirmPromptButtonMolecule;
use crate::components::generic_button::GenericButton;


#[derive(PartialEq,Clone,Copy)]
pub enum CambiarDatosUsuarioError{
    SinError,
    EmailInvalido,
    NombreConNumeros,
}

#[function_component(EditPersonalInfoMolecule)]
pub fn edit_personal_info_molecule() -> Html {

    //comprobaciones de first render
    let default_local_date: DateTime<Local> = Local.with_ymd_and_hms(1,1,1,1,1,1).unwrap();
    let first_render_state = use_state(|| true);
    let cloned_first_render_state = first_render_state.clone();
    
    let (store, _dispatch) = use_store::<UserStore>();
    let dni = store.dni;
    
    let show_button_state = use_state(|| false);
    
    let navigator = use_navigator().unwrap();   
    
    let name_state:UseStateHandle<String> = use_state(|| "default".to_string());   
    let cloned_name_state = name_state.clone();  
    
    let email_state = use_state(|| "default".to_string());    
    let cloned_email_state = email_state.clone(); 
   
    let cloned_dni = dni.clone();
   
    let born_date_state = use_state(|| default_local_date);    
    let cloned_born_date_state = born_date_state.clone(); 
    
    let user_state = use_state(|| User::new("default".to_string(), "default".to_string(), default_local_date));
    let cloned_user_state = user_state.clone();
    use_effect(move ||{
        let cloned_first_render_state = cloned_first_render_state.clone();
        if (&*cloned_first_render_state).clone(){
            let cloned_dni = cloned_dni.clone();
            let cloned_born_date_state = cloned_born_date_state.clone();
            let cloned_name_state = cloned_name_state.clone();
            let cloned_email_state = cloned_email_state.clone();
            let cloned_user_state = cloned_user_state.clone(); {
                let cloned_dni = cloned_dni.clone();
                spawn_local(async move {
                    let cloned_dni = cloned_dni.clone();
                    let cloned_user_state = cloned_user_state.clone();
                    let cloned_name_state = cloned_name_state.clone();
                    let cloned_email_state = cloned_email_state.clone();
                    let cloned_born_date_state = cloned_born_date_state.clone();
                    let query = QueryGetUserInfo { dni : cloned_dni.unwrap() };
                    let respuesta = Request::post("/api/get_user_info").header("Content-Type", "application/json").body(serde_json::to_string(&query).unwrap()).send().await;
                    match respuesta{
                        Ok(respuesta) =>{
                            let response:Result<Option<ResponseGetUserInfo>, reqwasm::Error> = respuesta.json().await;
                            match response{
                                Ok(respuesta) => {  
                                    if respuesta.is_some(){
                                        let cloned_name_state = cloned_name_state.clone();
                                        let cloned_email_state = cloned_email_state.clone();
                                        let cloned_user_state = cloned_user_state.clone();
                                        let cloned_born_date_state = cloned_born_date_state.clone();
                                        let nombre_traido = respuesta.clone().unwrap().nombre_y_ap;
                                        cloned_user_state.set(User::new(nombre_traido, respuesta.clone().unwrap().email, respuesta.clone().unwrap().nacimiento));
                                        cloned_name_state.set(respuesta.clone().unwrap().nombre_y_ap);
                                        cloned_email_state.set(respuesta.clone().unwrap().email);
                                        cloned_born_date_state.set(respuesta.unwrap().nacimiento);
                                    }    else{
                                        log::error!("user not found (frontend)");
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
                    
                });
            }
            cloned_first_render_state.set(false);
        }
        
        
        ||{}
        
    });
    
    let navigator = navigator.clone();
    let dni = dni.clone();
    use_effect(move || {
        if dni.is_none() {
            navigator.push(&Route::LogInPage)
        }
    });
    
    
    
    //logica de botones y cambio de datos

    
    //cloned_user_state tiene los datos de la persona que esta en la pagina
    let cloned_user_state = user_state.clone();
    let cloned_name_state = name_state.clone();

    //usuario preparado para recibir los cambios al final



    //cambio de nombre

    let cloned_user_state = user_state.clone();
    let full_name_changed = Callback::from(move |new_name:String|{
        let cloned_user_state = cloned_user_state.clone();
        let cloned_name_state = cloned_name_state.clone();
        if new_name != cloned_user_state.full_name {
            cloned_name_state.set(new_name);
        } else{
            cloned_name_state.set((&*cloned_user_state).clone().full_name);
        }
    });

    //cambio de email

    let cloned_email_state = email_state.clone();
    let cloned_user_state = user_state.clone();
    let cloned_user_state = user_state.clone();
    let full_email_changed = Callback::from(move |new_email: Option<String>| {
        //let cloned_user_state = cloned_user_state.clone();
        let cloned_email_state = cloned_email_state.clone();
       //     cloned_email_state.set((&*cloned_user_state).clone().email);
            cloned_email_state.set(new_email.unwrap());
        
    });
    
    
    //cambio de fecha HACER
    
   
   let cloned_born_date_state = born_date_state.clone();

    let full_born_date_changed:Callback<String> = Callback::from(move |new_date:String|{
        let parsed_date = NaiveDate::parse_from_str(&new_date, "%Y-%m-%d");
        let new_date = parsed_date.unwrap();
        let time = NaiveTime::from_hms_opt(0, 0, 0);
        let naive_datetime = new_date.and_hms_opt(0, 0, 0).unwrap();
        let cloned_born_date_state = cloned_born_date_state.clone();
        let new_date: DateTime<Local> = Local.from_local_datetime(&naive_datetime)
        .single()
        .expect("Error al convertir NaiveDateTime a DateTime<Local>");
        cloned_born_date_state.set(new_date);

    });

    // creo el state de error
    let my_error_state = use_state(|| CambiarDatosUsuarioError::SinError);
    let cloned_my_error_state = my_error_state.clone();
    
    //traigo los nuevos valores
    let cloned_user_state = user_state.clone();
    let cloned_email_state = email_state.clone(); 
    let cloned_name_state = name_state.clone(); 
    let cloned_born_date_state = born_date_state.clone(); 
    let (store, dispatch) = use_store::<UserStore>();
    let dni = store.dni;
    let cloned_dni = dni.clone();
    let cloned_show_button_state = show_button_state.clone();
    let (store, dispatch) = use_store::<UserStore>();
    let cloned_dispatch = dispatch.clone();
    let change_user = Callback::from(move |e:MouseEvent|{
        let cloned_my_error_state = cloned_my_error_state.clone();
        let cloned_dispatch = cloned_dispatch.clone();
        let cloned_born_date_state = cloned_born_date_state.clone(); 
        let cloned_email_state = cloned_email_state.clone(); 
        let cloned_name_state = cloned_name_state.clone(); 
        let cloned_show_button_state = cloned_show_button_state.clone();
        let cloned_user_state_before_confirm = cloned_user_state.clone();
        if !(*cloned_email_state).contains("@") || !(*cloned_email_state).contains(".com") {
            cloned_my_error_state.set(CambiarDatosUsuarioError::EmailInvalido);
        } else if (*cloned_name_state).contains("1") || (*cloned_name_state).contains("2") || (*cloned_name_state).contains("3") || (*cloned_name_state).contains("4") || (*cloned_name_state).contains("5") || (*cloned_name_state).contains("6") || (*cloned_name_state).contains("7") || (*cloned_name_state).contains("8") || (*cloned_name_state).contains("9") || (*cloned_name_state).contains("0"){
            cloned_my_error_state.set(CambiarDatosUsuarioError::NombreConNumeros);
        } else {
                let cloned_user_state = cloned_user_state.clone();
                let new_user = User::new((&*cloned_name_state).clone(), (&*cloned_email_state).clone(), (cloned_user_state_before_confirm.born_date).clone());
                cloned_user_state.set(new_user);
                cloned_show_button_state.set(false);
                spawn_local(async move {
                    let cloned_dispatch = cloned_dispatch.clone();
                    let cloned_born_date_state = cloned_born_date_state.clone(); 
                    let cloned_user_state = cloned_user_state.clone();
                    let cloned_dni = cloned_dni.clone();
                    let query = QueryCambiarDatosUsuario { dni : cloned_dni.unwrap(), full_name : (&*cloned_name_state).clone(), email : (&*cloned_email_state).clone(), born_date : (&*cloned_born_date_state).clone() };
                    let response = Request::post("/api/cambiar_usuario").header("Content-Type", "application/json").body(serde_json::to_string(&query).unwrap()).send().await;
                    match response{
                        Ok(response) => {
                            let response:Result<ResponseCambiarDatosUsuario, reqwasm::Error> = response.json().await;
                            log::info!("deserialice la respuesta {:?}", response);
                            match response{
                                Ok(response) => {  
                                    if response.datos_cambiados{
                                        log::info!("datos cambiados con exito");
                                        let cloned_dispatch = cloned_dispatch.clone();
                                        cloned_dispatch.reduce_mut(|store|{
                                            store.nombre = (&*cloned_name_state).clone();
                                        });
                                        
                                        if let Some(window) = window() {
                                            window.location().reload().unwrap();
                                        }
                                    }else{
                                        log::error!("ERROR EN EL CAMBIO DE USUARIO");
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
                });
            }
            
        });
        
        let cloned_show_button_state = show_button_state.clone();
        let reject_changes = Callback::from(move |e:MouseEvent|{
            let cloned_show_button_state = cloned_show_button_state.clone();
            cloned_show_button_state.set(false);
        });
        
        let cloned_show_button_state = show_button_state.clone();
        let change_user_button = Callback::from(move |()|{
            let cloned_show_button_state = cloned_show_button_state.clone();
            cloned_show_button_state.set(true);
        });

    let mut error_text="".to_string();
    match (*my_error_state){
        CambiarDatosUsuarioError::EmailInvalido =>{
            error_text = "email invalido".to_string();
        }
        CambiarDatosUsuarioError::NombreConNumeros =>{
            error_text = "el nombre y apellido no pueden contener numeros!".to_string();
        }
        CambiarDatosUsuarioError::SinError => (),
    }

    html! {
        <>
            <div class="edit-personal-info-box">
                if (*my_error_state) != CambiarDatosUsuarioError::SinError{
                    <div> {error_text.clone()} </div>
                }
                <h2 class="information-text">{"Nombre y apellido: "} {&*user_state.full_name}</h2>
                <CheckedInputField name = "full_name_change" label="Ingresa tu nuevo nombre" tipo = "text" on_change = {full_name_changed} />
                <h2 class="information-text">{"Email: "} {&*user_state.email}</h2>
                <CheckedInputField name = "email" label="Ingresa tu nuevo email" tipo = "email" on_change_checked = {full_email_changed} />
                <h2 class="information-text">{"Fecha de nacimiento: "} {(&user_state.born_date).clone().format("%Y-%m-%d")}</h2>
                <CheckedInputField name = "full_date_change" label="Ingresa tu nueva fecha" tipo = "date" on_change = {full_born_date_changed} />
                <GenericButton text = "cambiar datos" onclick_event = {change_user_button} />
                if (&*show_button_state).clone(){
                    <div class="confirm-prompt">
                        <ConfirmPromptButtonMolecule text = "Seguro de que quiere cambiar su nombre?" confirm_func = {change_user} reject_func = {reject_changes}  />
                    </div>
                }
            </div>
        </>
    
    }
}
