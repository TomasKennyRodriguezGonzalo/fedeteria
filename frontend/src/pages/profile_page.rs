use yew::prelude::*;
use yew_router::{hooks::use_navigator, navigator};
use yewdux::prelude::*;
extern crate chrono;
use chrono::prelude::*;
use crate::{router::Route, store::UserStore};
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::Request;
use datos_comunes::{self, QueryGetUserInfo, ResponseGetUserInfo};



pub struct User{
    full_name:String,
    email:String,
    born_date:DateTime<Local>,
   // publications:
}

impl User{
    fn new(full_name:String,email:String,born_date:DateTime<Local>,) ->User{
            User {full_name, email, born_date}
        }
}


#[function_component(ProfilePage)]
pub fn profile_page() -> Html {

    let (store, _dispatch) = use_store::<UserStore>();
    let dni = store.dni;

    let default_local_date: DateTime<Local> = Local.with_ymd_and_hms(1,1,1,1,1,1).unwrap();
    let user_state = use_state(|| User::new("default".to_string(), "default".to_string(), default_local_date));
    let cloned_user_state = user_state.clone();

    let navigator = use_navigator().unwrap();

    let cloned_dni = dni.clone();
    let get_user = Callback::from(move |()| {
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
    });

    let navigator = navigator.clone();
    let dni = dni.clone();
    use_effect(move || {
        if dni.is_none() {
            navigator.push(&Route::LogInPage)
        }
    });

    html! (
        <div class="profile-page">
            <h1>{"PERFIL"}</h1>
            <div>{"listo para realizar unos trueques?"}</div>
            <div>{"nombre y apellido: "} {&*user_state.full_name}</div>
            <div>{"email: "} {&*user_state.email}</div>
            <div>{"fecha de nacimiento: "} {&*user_state.born_date.to_string()}</div>
        </div>
    )
}