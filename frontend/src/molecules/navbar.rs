use web_sys::window;
use datos_comunes::{RolDeUsuario, ResponseGetUserRole, QueryGetUserRole};
use wasm_bindgen_futures::spawn_local;
use yew_router::hooks::use_navigator;
use yewdux::use_store;
use yew_router::prelude::Link;
use yew::prelude::*;
use crate::{components::generic_button::GenericButton, information_store::InformationStore};
use crate::store::UserStore;
use crate::router::Route;
use crate::components::indexed_button::IndexedButton;
use crate::components::checked_input_field::CheckedInputField;
use reqwasm::http::Request;
use datos_comunes::{Publicacion, QueryPublicacionesFiltradas, ResponsePublicacionesFiltradas};

#[function_component(Navbar)]
pub fn navbar() -> Html{

    let navigator = use_navigator().unwrap();
    let (store, dispatch) = use_store::<UserStore>();
    
    let role_state: UseStateHandle<Option<RolDeUsuario>> = use_state(|| None);
    
    let navigator_cloned = navigator.clone();
    let logout = Callback::from(move|_event| {
        dispatch.reduce_mut(|store| {store.dni = None; store.nombre = "".to_string()});
        navigator_cloned.push(&Route::Home);
        // Refreshes to reset the first load states all over the code
        if let Some(window) = window() {
            window.location().reload().unwrap();
        }
    });
    
    let first_render_state = use_state(|| true);
    let cloned_first_render_state = first_render_state.clone();
    
    let dni = store.dni;
    let username = store.nombre.clone();

    let cloned_role_state = role_state.clone();
    let cloned_dni = dni.clone();
    use_effect( move || {
        let cloned_dni = cloned_dni.clone();
        let username = store.nombre.clone();
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
        
    let (information_store, information_dispatch) = use_store::<InformationStore>();
    let messages = information_store.messages.clone();
        
    let onclick = Callback::from(move |button_index:usize|{
        information_dispatch.reduce_mut(|store| store.messages.remove(button_index));
    });

    let state_product_to_search = use_state(|| "".to_string());
    let state_product_to_search_cloned = state_product_to_search.clone();
    let product_name_change = Callback::from(move |value: String| {
        state_product_to_search_cloned.set(value);
    });
    let state_product_to_search_clone = state_product_to_search.clone();
    let navigator_cloned = navigator.clone();

    let search_products = Callback::from(move |()| {
        let state_product_to_search_string = &*state_product_to_search_clone;
        let search_query = QueryPublicacionesFiltradas {filtro_dni: None, filtro_nombre: Some(state_product_to_search_string.clone()), filtro_fecha_min: None, filtro_fecha_max: None};
        let search_query_json = serde_json::to_string(&search_query).unwrap();
        navigator_cloned.push(&Route::SearchResults{search_query: search_query_json});
    });
        
    html!{
        <>
            <header class="navbar">
                <div class="logo">
                    <Link<Route> to={Route::Home}><img src="/assets/img/Fedeteria_Solo_Logo.svg" alt="fedeteria"/></Link<Route>>
                </div>
                <div class="search-bar">
                    <CheckedInputField name="product-name" label="Buscador por nombre" tipo="text" on_change={product_name_change}/>
                    <GenericButton text="Buscar" onclick_event={search_products}/>
                </div>
                if dni.is_some() && (&*role_state).clone().is_some() {
                    <div>
                        <h2>{"Hola " }{username}{"!"}</h2>
                    </div>
                    <nav>
                            {
                                match (&*role_state).clone().unwrap() { 
                                    RolDeUsuario::Dueño => {
                                        html!{
                                        <ul class="option_list">
                                            <li><Link<Route> to={Route::Profile}>{"Perfil"}</Link<Route>></li>
                                            <li><Link<Route> to={Route::PrivilegedActions}>{"Menú de acciones"}</Link<Route>></li>
                                            <li><a onclick={logout}>{"Cerrar Sesion"}</a></li>
                                        </ul>
                                        }
                                    },
                                    RolDeUsuario::Empleado{sucursal : _} => {
                                        html!{
                                        <ul class="option_list">
                                            <li><Link<Route> to={Route::Profile}>{"Perfil"}</Link<Route>></li>
                                            <li><Link<Route> to={Route::PrivilegedActions}>{"Menú de acciones"}</Link<Route>></li>
                                            <li><a onclick={logout}>{"Cerrar Sesion"}</a></li>
                                        </ul>
                                        }
                                    },
                                    RolDeUsuario::Normal => {
                                        html!{
                                        <ul class="option_list">
                                            <li><Link<Route> to={Route::Profile}>{"Perfil"}</Link<Route>></li>
                                            <li><a onclick={logout}>{"Cerrar Sesion"}</a></li>
                                        </ul>
                                        }
                                    }
                                }
                            }
                    </nav>
                } else {
                    <div>
                        <h2>{"No tienes tu sesión iniciada." }</h2>
                    </div>
                    <nav>
                        <ul class="option_list">
                            <li><Link<Route> to={Route::LogInPage}>{"Iniciar Sesion"}</Link<Route>></li>
                        </ul>
                    </nav>
                }
            </header>
            if !messages.is_empty() {
                <div class="information-message-list">
                    {   
                        messages.iter().enumerate().map(move |(index, message)| html! {
                            <div class="information-message">
                                <h2>{ message.clone() }</h2>
                                <IndexedButton text="Cerrar mensaje" index={index.clone()} onclick_event={onclick.clone()}/>
                            </div>
                            }).collect::<Html>()
                    }
                </div>
            }
        </>
    }
}