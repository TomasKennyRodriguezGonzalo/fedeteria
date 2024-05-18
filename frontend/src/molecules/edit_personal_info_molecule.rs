use std::str::FromStr;

use gloo::console::log;
use yew::prelude::*;
use yew_router::{hooks::use_navigator, navigator};
use yewdux::prelude::*;
extern crate chrono;
use chrono::prelude::*;
use crate::components::generic_button::_Props::onclick_event;
use crate::{router::Route, store::UserStore};
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::Request;
use datos_comunes::{self, QueryCambiarDatosUsuario, QueryGetUserInfo, ResponseCambiarDatosUsuario, ResponseGetUserInfo};
use yew_router::prelude::Link;
use crate::pages::profile_page::User;
use crate::components::generic_input_field::GenericInputField;
use crate::molecules::confirm_prompt_button_molecule::ConfirmPromptButtonMolecule;
use crate::components::generic_button::GenericButton;





#[function_component(EditPersonalInfoMolecule)]
pub fn edit_personal_info_molecule() -> Html {

    //comprobaciones de first render
    let default_local_date: DateTime<Local> = Local.with_ymd_and_hms(1,1,1,1,1,1).unwrap();
    let user_state = use_state(|| User::new("default".to_string(), "default".to_string(), default_local_date));
    let cloned_user_state = user_state.clone();
    let first_render_state = use_state(|| true);
    let cloned_first_render_state = first_render_state.clone();
    
    let (store, _dispatch) = use_store::<UserStore>();
    let dni = store.dni;

    
    let show_button_state = use_state(|| false);
    
    let navigator = use_navigator().unwrap();   
    use_effect(move ||{
        let cloned_first_render_state = cloned_first_render_state.clone();
        if (&*cloned_first_render_state).clone(){
            let cloned_dni = dni.clone();
            let cloned_dni = cloned_dni.clone();
            let cloned_user_state = cloned_user_state.clone(); {
                let cloned_dni = cloned_dni.clone();
                spawn_local(async move {
                    let cloned_dni = cloned_dni.clone();
                    let cloned_user_state = cloned_user_state.clone();
                    let query = QueryGetUserInfo { dni : cloned_dni.unwrap() };
                    log::info!("entre al spawn local");
                    let respuesta = Request::post("/api/get_user_info").header("Content-Type", "application/json").body(serde_json::to_string(&query).unwrap()).send().await;
                    match respuesta{
                        Ok(respuesta) =>{
                            let response:Result<Option<ResponseGetUserInfo>, reqwasm::Error> = respuesta.json().await;
                            log::info!("deserailice la respuesta {:?}",response);
                            match response{
                                Ok(respuesta) => {  
                                    if respuesta.is_some(){
                                        let user_info = User::new(respuesta.clone().unwrap().nombre_y_ap, respuesta.clone().unwrap().email, respuesta.clone().unwrap().nacimiento);
                                        cloned_user_state.set(user_info);
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


    //usuario preparado para recibir los cambios al final
    let user_state_before_confirm = use_state(|| User::new("".to_string(), "".to_string(), default_local_date));
    let cloned_user_state_before_confirm = user_state_before_confirm.clone();



    //cambio de nombre
    let name_state:UseStateHandle<String> = use_state(|| (cloned_user_state_before_confirm.full_name).clone());    
    let cloned_name_state = name_state.clone(); 

    let full_name_changed = Callback::from(move |new_name|{
        let cloned_name_state = cloned_name_state.clone();
        cloned_name_state.set(new_name);
    });

    //cambio de email
    let email_state = use_state(|| (cloned_user_state_before_confirm.email).clone());    
    let cloned_email_state = email_state.clone(); 
    
    let full_email_changed:Callback<String> = Callback::from(move |new_email|{
        let cloned_email_state = cloned_email_state.clone();
        cloned_email_state.set(new_email);
    });
    
    
    //cambio de fecha HACER
    
    let born_date_state = use_state(|| (cloned_user_state_before_confirm.born_date).clone());    
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


    //traigo los nuevos valores
    let cloned_email_state = email_state.clone(); 
    let cloned_name_state = name_state.clone(); 
    let cloned_born_date_state = born_date_state.clone(); 
    let (store, dispatch) = use_store::<UserStore>();
    let dni = store.dni;
    let cloned_dni = dni.clone();
    let cloned_show_button_state = show_button_state.clone();
    let change_user = Callback::from(move |e:MouseEvent|{
        log::info!("el nombre que se va a enviar es: {:?}", (&*cloned_name_state).clone());
        let cloned_born_date_state = cloned_born_date_state.clone(); 
        let cloned_email_state = cloned_email_state.clone(); 
        let cloned_name_state = cloned_name_state.clone(); 
        let cloned_show_button_state = cloned_show_button_state.clone();
        let cloned_user_state_before_confirm = user_state_before_confirm.clone();
        let cloned_user_state = cloned_user_state.clone();
        let new_user = User::new((*cloned_name_state).clone(), (*cloned_email_state).clone(), (cloned_user_state_before_confirm.born_date).clone());
        cloned_user_state.set(new_user);
        cloned_show_button_state.set(false);
            spawn_local(async move {
                let cloned_born_date_state = cloned_born_date_state.clone(); 
                let cloned_user_state = cloned_user_state.clone();
                let cloned_dni = cloned_dni.clone();
                log::info!("El dni es: {:?}", cloned_dni);
                log::info!("la nueva local date es: {:?}", &*cloned_born_date_state);
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

    html! {
        <>
            <h2 class="information-text">{"nombre y apellido: "} {&*user_state.full_name}</h2>
            <GenericInputField name = "full_name_change" label="Ingresa tu nuevo nombre" tipo = "text" handle_on_change = {full_name_changed} />
            if (&*show_button_state).clone(){
                <ConfirmPromptButtonMolecule text = "Seguro de que quiere cambiar su nombre?" confirm_func = {change_user} reject_func = {reject_changes}  />
            }
            <h2 class="information-text">{"email: "} {&*user_state.email}</h2>
            <GenericInputField name = "full_name_change" label="Ingresa tu nuevo email" tipo = "email" handle_on_change = {full_email_changed} />
            <h2 class="information-text">{"fecha de nacimiento: "} {(&user_state.born_date).to_string().clone()}</h2>
            <GenericInputField name = "full_date_change" label="Ingresa tu nueva fecha" tipo = "date" handle_on_change = {full_born_date_changed} />
            <GenericButton text = "cambiar datos" onclick_event = {change_user_button} />
        </>
    
    }
}
