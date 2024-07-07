use web_sys::window;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yew_router::components::Link;
use yew_router::hooks::use_navigator;
use yewdux::prelude::*;
extern crate chrono;
use chrono::prelude::*;
use crate::information_store::InformationStore;
use crate::request_post;
use crate::{router::Route, store::UserStore};
use datos_comunes::{self, QueryCambiarDatosUsuario, QueryGetUserInfo, ResponseCambiarDatosUsuario, ResponseGetUserInfo};
use crate::components::checked_input_field::CheckedInputField;
use crate::molecules::confirm_prompt_button_molecule::ConfirmPromptButtonMolecule;
use crate::components::generic_button::GenericButton;


#[derive(PartialEq,Clone,Copy)]
pub enum CambiarDatosUsuarioError {
    SinError,
    EmailInvalido,
    NombreConNumeros,
    EmailExistente,
    MenorA18,
    ErrorIndeterminado,
}

#[function_component(EditPersonalInfoMolecule)]
pub fn edit_personal_info_molecule() -> Html {
    
    let (store, store_dispatch) = use_store::<UserStore>();
    let dni = store.dni;
    
    let show_button_state = use_state(|| false);
    
    let navigator = use_navigator().unwrap();   
    
    let name_state = use_state(|| "".to_string());   
    let email_state = use_state(|| Some("".to_string()));    
    let birth_date_state = use_state(|| None);    
    let datos_viejos: UseStateHandle<Option<(String, String, String)>> = use_state(|| None);
    let (_information_store, information_dispatch) = use_store::<InformationStore>();
   
    let datos_viejos_c = datos_viejos.clone();
    // Me traigo los datos actuales del usuario
    use_effect_once(move || {
        let datos_viejos = datos_viejos_c;
        let query = QueryGetUserInfo { dni: dni.unwrap() };
        request_post("/api/get_user_info", query, move |response: ResponseGetUserInfo| {
            datos_viejos.set(Some((
                response.nombre_y_ap.clone(),
                response.email.clone(),
                response.nacimiento.clone().format("%Y-%m-%d").to_string()
            )));
        });
        
        || {}
    });
    
    let navigator = navigator.clone();
    use_effect(move || {
        if dni.is_none() {
            navigator.push(&Route::LogInPage)
        }
    });
    
    let name_state_c = name_state.clone();
    let email_state_c = email_state.clone();
    let birth_date_state_c = birth_date_state.clone();
    let name_changed = Callback::from(move |new_name: String| {
        name_state_c.set(new_name);
    });

    let email_changed: Callback<Result<String, String>> = Callback::from(move |new_email: Result<String, String>| {
        match new_email {
            Ok(new_email) => email_state_c.set(Some(new_email)),
            Err(new_email_malo) => {
                if new_email_malo == "" {
                    email_state_c.set(Some("".to_string()))
                } else {
                    email_state_c.set(None);
                }
            },
        }
    });

    let full_born_date_changed = Callback::from(move |new_date: String| {
        let parsed_date = NaiveDate::parse_from_str(&new_date, "%Y-%m-%d");
        let new_date = parsed_date.unwrap();
        // let time = NaiveTime::from_hms_opt(0, 0, 0);
        let naive_datetime = new_date.and_hms_opt(0, 0, 0).unwrap();
        let new_date: DateTime<Local> = Local.from_local_datetime(&naive_datetime)
            .single()
            .expect("Error al convertir NaiveDateTime a DateTime<Local>");
        birth_date_state_c.set(Some(new_date));
    });

    // creo el state de error
    let my_error_state = use_state(|| CambiarDatosUsuarioError::SinError);
    let cloned_my_error_state = my_error_state.clone();

    let name_state_c = name_state.clone();
    let email_state_c = email_state.clone();
    let birth_date_state_c = birth_date_state.clone();
    let show_button_state_c = show_button_state.clone();
    let information_dispatch_c = information_dispatch.clone();
    let change_user = Callback::from(move |_: MouseEvent| {
        let my_error_state = cloned_my_error_state.clone();
        let store_dispatch = store_dispatch.clone();
        let name_state = name_state_c.clone();
        let birth_date_state = birth_date_state_c.clone();
        let email_state = email_state_c.clone();
        let show_button_state = show_button_state_c.clone();
        let information_dispatch = information_dispatch_c.clone();
        show_button_state.set(false);
        if let Some(email) = email_state.as_deref() {
            let name = (*name_state).clone();
            let query = QueryCambiarDatosUsuario {
                dni : dni.unwrap(),
                full_name : if name.is_empty() {None} else {Some(name)},
                email : if email.is_empty() {None} else {Some(email.to_string())},
                born_date : *birth_date_state
            };
            request_post("/api/cambiar_usuario", query, move |response: ResponseCambiarDatosUsuario| {
                match response {
                    Ok(()) => {
                        let mensaje = "Tus datos personales se han cambiado con éxito.".to_string();
                        information_dispatch.reduce_mut(|store| store.messages.push(mensaje));
                        store_dispatch.reduce_mut(|store|{
                            if !(&*name_state).clone().is_empty(){
                                store.nombre.clone_from(&(*name_state));
                            }
                        });
                        
                        if let Some(window) = window() {
                            window.location().reload().unwrap();
                        }
                    },
                    Err(err) => {
                        my_error_state.set(match err {
                            datos_comunes::ErrorCambiarDatosUsuario::ErrorIndeterminado => CambiarDatosUsuarioError::ErrorIndeterminado,
                            datos_comunes::ErrorCambiarDatosUsuario::EmailExistente => CambiarDatosUsuarioError::EmailExistente,
                            datos_comunes::ErrorCambiarDatosUsuario::MenorA18 => CambiarDatosUsuarioError::MenorA18,
                        });
                    },
                }
            });
        } else {
            my_error_state.set(CambiarDatosUsuarioError::EmailInvalido);
        }
            
    });
        
    let cloned_show_button_state = show_button_state.clone();
    let reject_changes = Callback::from(move |_|{
        let cloned_show_button_state = cloned_show_button_state.clone();
        cloned_show_button_state.set(false);
    });
    
    let cloned_show_button_state = show_button_state.clone();
    let change_user_button = Callback::from(move |()|{
        let cloned_show_button_state = cloned_show_button_state.clone();
        cloned_show_button_state.set(true);
    });

    let mut error_text="".to_string();
    match *my_error_state {
        CambiarDatosUsuarioError::EmailInvalido =>{
            error_text = "email invalido".to_string();
        }
        CambiarDatosUsuarioError::NombreConNumeros => {
            error_text = "el nombre y apellido no pueden contener numeros!".to_string();
        }
        CambiarDatosUsuarioError::SinError => (),
        CambiarDatosUsuarioError::ErrorIndeterminado => {
            error_text = "error indeterminado".to_string();
        },
        CambiarDatosUsuarioError::MenorA18 => {
            error_text = "la edad no debe ser menor a 18".to_string();
        },
        CambiarDatosUsuarioError::EmailExistente => {
            error_text = "ese email ya pertenece a otro usuario".to_string();
        },
    }

    html! {
        <>
            <div class="edit-personal-info-box">
                if let Some(datos_viejos) = datos_viejos.as_ref() {
                    if (*my_error_state) != CambiarDatosUsuarioError::SinError{
                        <div> {error_text.clone()} </div>
                    }
                    <h2 class="information-text">{"Nombre y apellido: "} {datos_viejos.0.clone()}</h2>
                    <CheckedInputField name = "full_name_change" label="Ingresa tu nuevo nombre" tipo = "text" on_change = {name_changed} />
                    <h2 class="information-text">{"Email: "} {datos_viejos.1.clone()}</h2>
                    <CheckedInputField name = "email" label="Ingresa tu nuevo email" tipo = "email" on_change_checked = {email_changed} />
                    <h2 class="information-text">{"Fecha de nacimiento: "} {datos_viejos.2.clone()}</h2>
                    <CheckedInputField name = "full_date_change" label="Ingresa tu nueva fecha" tipo = "date" on_change = {full_born_date_changed} />
                    <Link<Route> to={Route::ChangePasswordFromProfile}>{"Cambiar Contraseña"}</Link<Route>>
                    <GenericButton text = "cambiar datos" onclick_event = {change_user_button} />
                    if (&*show_button_state).clone(){
                        <ConfirmPromptButtonMolecule text = "¿Confirma los cambios?" confirm_func = {change_user} reject_func = {reject_changes}  />
                    }
                } else {
                    {"Cargando..."}
                }
            </div>
        </>
    
    }
}
