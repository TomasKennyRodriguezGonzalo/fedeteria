use yewdux::use_store;
use yew_router::{components::Link, hooks::use_navigator, navigator};
use yew::prelude::*;
use datos_comunes::{RolDeUsuario, QueryGetUserRole, ResponseGetUserRole};
use crate::{router::Route, store::UserStore};
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;

#[function_component(PrivilegedActionsPage)]
pub fn privileged_actions_page() -> Html {
    let navigator = use_navigator().unwrap();

    let (store, _dispatch) = use_store::<UserStore>();
    let dni = store.dni;

    let role_state: UseStateHandle<Option<RolDeUsuario>> = use_state(|| None);
    
    let first_render_state = use_state(|| true);
    let cloned_first_render_state = first_render_state.clone();

    let cloned_role_state = role_state.clone();
    let cloned_dni = dni.clone();
    use_effect( move || {
        let cloned_first_render_state = cloned_first_render_state.clone();
        if *cloned_first_render_state{
            let cloned_dni = cloned_dni.clone();
            if cloned_dni.is_some() {
                spawn_local(async move {
                    let cloned_dni = cloned_dni.clone();
                    let cloned_role_state = cloned_role_state.clone();
                    log::info!("El dni es: {:?}", cloned_dni);
                    let query = QueryGetUserRole { dni : cloned_dni.unwrap() };
                    let response = Request::post("/api/obtener_rol").header("Content-Type", "application/json").body(serde_json::to_string(&query).unwrap()).send().await;
                    match response{
                        Ok(response) => {
                            let response:Result<Option<ResponseGetUserRole>, reqwasm::Error> = response.json().await;
                            log::info!("deserialice la respuesta {:?}", response);
                            match response{
                                Ok(response) => {  
                                    if response.is_some(){
                                        let user_role = response.unwrap().rol;
                                        cloned_role_state.set(Some(user_role));
                                    }else{
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


    html! {
        <div class="privileged-actions-box">
            if (&*role_state).clone().is_some(){
                <h1 class="title">{"Acciones privilegiadas"}</h1> 
                {
                    match (&*role_state).clone().unwrap() {
                        RolDeUsuario::Normal => {
                            navigator.push(&Route::Home);
                            html!()
                        },
                        RolDeUsuario::Dueño => {html!{
                            <ul class="option-list">
                                <li><Link<Route> to={Route::ChangeUserRole}>{"Cambiar Rol de Usuario"}</Link<Route>></li>
                                <li><Link<Route> to={Route::CreateOffice}>{"Agregar Sucursal"}</Link<Route>></li>
                                <li><Link<Route> to={Route::DeleteOffice}>{"Eliminar Sucursal"}</Link<Route>></li>
                                <li><Link<Route> to={Route::UnlockAccount}>{"Desbloquear Cuenta"}</Link<Route>></li>
                                <li><Link<Route> to={Route::AwaitingPricePublication}>{"Ver Publicaciones Esperando Tasación"}</Link<Route>></li>
                            </ul>
                        }},
                        RolDeUsuario::Empleado { sucursal : _ } => {html! {
                            <ul class="option-list">
                            </ul>
                        }},
                    }            
                }
            }
        </div>
    }
}