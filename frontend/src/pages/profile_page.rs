use yew::prelude::*;
use yew_hooks::use_effect_once;
use yew_router::hooks::use_navigator;
use yewdux::prelude::*;
extern crate chrono;
use chrono::prelude::*;
use crate::{request_post, router::Route, store::UserStore};
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::Request;
use datos_comunes::{self, QueryGetUserInfo, ResponseGetUserInfo};
use yew_router::prelude::Link;

#[derive(Clone)]
pub struct User{
    pub full_name:String,
    pub email:String,
    pub born_date:DateTime<Local>,
    pub puntos: i64,
   // publications:
}

impl User{
    pub fn new(full_name:String,email:String,born_date:DateTime<Local>, puntos: i64) ->User{
            User {full_name, email, born_date, puntos}
        }
}


#[function_component(ProfilePage)]
pub fn profile_page() -> Html {

    let (store, _dispatch) = use_store::<UserStore>();
    let dni = store.dni;

    let default_local_date: DateTime<Local> = Local.with_ymd_and_hms(1,1,1,1,1,1).unwrap();
    let user_state = use_state(|| User::new("default".to_string(), "default".to_string(), default_local_date, 0));
    let cloned_user_state = user_state.clone();
    let first_render_state = use_state(|| true);
    let cloned_first_render_state = first_render_state.clone();

    let navigator = use_navigator().unwrap();
    
        use_effect_once(move ||{
            let cloned_first_render_state = cloned_first_render_state.clone();
            if (&*cloned_first_render_state).clone() {
                let cloned_dni = dni.clone();
                    let cloned_dni = cloned_dni.clone();
                    let cloned_user_state = cloned_user_state.clone();
                    let cloned_dni = cloned_dni.clone();
                    let query = QueryGetUserInfo { dni : cloned_dni.unwrap() };
                    request_post("/api/get_user_info", query, move |respuesta:Option<ResponseGetUserInfo>| {
                        
                        if let Some(respuesta) = respuesta {
                            let user_info = User::new(
                                respuesta.nombre_y_ap,
                                respuesta.email,
                                respuesta.nacimiento,
                                respuesta.puntos);
                            cloned_user_state.set(user_info);
                        } else {
                            log::error!("user not found (frontend)");
                        }
                    });
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
        
        html! (
            <div class="profile-page-box">
                <div class="profile-information-box">
                    <h1 class="title">{"Tu información"}</h1>
                    <h2 class="information-text">{"Nombre y apellido: "} {&*user_state.full_name}</h2>
                    <h2 class="information-text">{"Email: "} {&*user_state.email}</h2>
                    <h2 class="information-text">{"Fecha de nacimiento: "} {(&user_state.born_date).clone().format("%Y-%m-%d")}</h2>
                    <h2 class="information-text">{"Puntos: "} {user_state.puntos}</h2>
                </div>
            <div class="profile-actions-box">
                <h1 class="title">{"Acciones"}</h1>
                <ul>
                    // <li><Link<Route> to={Route::SavedPublications}>{"Articulos Guardados"}</Link<Route>></li>
                  //  <li><Link<Route> to={Route::RecentlySeenPublications}>{"Vistos Recientemente"}</Link<Route>></li>
                    <li><Link<Route> to={Route::MyPublications}>{"Tus Publicaciones"}</Link<Route>></li>
                    <li><Link<Route> to={Route::MyTradesOffers}>{"Ofertas de Trueque"}</Link<Route>></li>
                    <li><Link<Route> to={Route::MyPendingTrades}>{"Trueques Pendientes"}</Link<Route>></li>
                    <li><Link<Route> to={Route::MyDefinedTrades}>{"Trueques Definidos"}</Link<Route>></li>
                    <li><Link<Route> to={Route::MyCompletedTrades}>{"Trueques Concretados"}</Link<Route>></li>
                    <li><Link<Route> to={Route::EditPersonalInfo}>{"Editar Información Personal"}</Link<Route>></li>
                </ul>
            </div>
        </div>
            )
}