use yew::prelude::*;
use yew_router::{hooks::use_navigator, navigator};
use yewdux::prelude::*;
extern crate chrono;
use chrono::prelude::*;
use crate::components::generic_button::_Props::onclick_event;
use crate::{router::Route, store::UserStore};
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::Request;
use datos_comunes::{self, QueryGetUserInfo, ResponseGetUserInfo};
use yew_router::prelude::Link;
use crate::pages::profile_page::User;
use crate::components::generic_input_field::GenericInputField;
use crate::molecules::confirm_prompt_button_molecule::ConfirmPromptButtonMolecule;
use crate::components::generic_button::GenericButton;





#[function_component(EditPersonalInfoMolecule)]
pub fn publication_thumbnail() -> Html {
    let (store, _dispatch) = use_store::<UserStore>();
    let dni = store.dni;

    let default_local_date: DateTime<Local> = Local.with_ymd_and_hms(1,1,1,1,1,1).unwrap();
    let user_state = use_state(|| User::new("default".to_string(), "default".to_string(), default_local_date));
    let cloned_user_state = user_state.clone();
    let first_render_state = use_state(|| true);
    let cloned_first_render_state = first_render_state.clone();

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

        let cloned_user_state = user_state.clone();
        let user_state_before_confirm = use_state(|| User::new("".to_string(), "".to_string(), default_local_date));
        let cloned_user_state_before_confirm = user_state_before_confirm.clone();

        let full_name_changed = Callback::from(move |new_name|{
            let cloned_user_state_before_confirm = cloned_user_state_before_confirm.clone();
            let new_user = User::new(new_name, (cloned_user_state_before_confirm.email).clone(), (cloned_user_state_before_confirm.born_date).clone());
            cloned_user_state_before_confirm.set(new_user);
        });
        

        let cloned_show_button_state = show_button_state.clone();
        let change_name = Callback::from(move |e:MouseEvent|{
            let cloned_show_button_state = cloned_show_button_state.clone();
            let cloned_user_state_before_confirm = user_state_before_confirm.clone();
            let cloned_user_state = cloned_user_state.clone();
            let new_user = User::new((cloned_user_state_before_confirm.full_name).clone(), (cloned_user_state_before_confirm.email).clone(), (cloned_user_state_before_confirm.born_date).clone());
            cloned_user_state.set(new_user);
            cloned_show_button_state.set(false);
  
        });

        let cloned_show_button_state = show_button_state.clone();
        let reject_name = Callback::from(move |e:MouseEvent|{
            let cloned_show_button_state = cloned_show_button_state.clone();
            cloned_show_button_state.set(false);
        });

        let cloned_show_button_state = show_button_state.clone();
        let change_button = Callback::from(move |()|{
            let cloned_show_button_state = cloned_show_button_state.clone();
            cloned_show_button_state.set(true);
        });

    html! {
        <>
            <h2 class="information-text">{"nombre y apellido: "} {&*user_state.full_name}</h2>
              <GenericInputField name = "full_name_change" label="Ingresa tu nuevo nombre" tipo = "text" handle_on_change = {full_name_changed} />
              <GenericButton text = "cambiar nombre" onclick_event = {change_button} />
              if (&*show_button_state).clone(){
                  <ConfirmPromptButtonMolecule text = "Seguro de que quiere cambiar su nombre?" confirm_func = {change_name} reject_func = {reject_name}  />
              }
            <h2 class="information-text">{"email: "} {&*user_state.email}</h2>
            <h2 class="information-text">{"fecha de nacimiento: "} {&*user_state.born_date.to_string()}</h2>
        </>
    
    }
}
