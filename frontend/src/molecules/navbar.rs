use datos_comunes::{RolDeUsuario, ResponseGetUserRole, QueryGetUserRole};
use wasm_bindgen_futures::spawn_local;
use yew_router::hooks::use_navigator;
use yewdux::use_store;
use yew_router::prelude::Link;
use yew::prelude::*;
use crate::information_store::InformationStore;
use crate::store::UserStore;
use crate::router::Route;
use crate::components::indexed_button::IndexedButton;
use reqwasm::http::Request;

#[function_component(Navbar)]
pub fn navbar() -> Html{

    let navigator = use_navigator().unwrap();
    
    let (store, dispatch) = use_store::<UserStore>();
    let dni = store.dni;
    let username = store.nombre.clone();

    let role_state: UseStateHandle<Option<RolDeUsuario>> = use_state(|| None);

    let logout = Callback::from(move|_event| {
        dispatch.reduce_mut(|store| store.dni = None);
        navigator.push(&Route::Home);
    });

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
        
        let (information_store, information_dispatch) = use_store::<InformationStore>();
        let messages = information_store.messages.clone();
        
        let onclick = Callback::from(move |button_index:usize|{
            information_dispatch.reduce_mut(|store| store.messages.remove(button_index));
        });
        
    html!{
        <>
            <header class="navbar">
                <div class="logo">
                    <Link<Route> to={Route::Home}><img src="/assets/img/Fedeteria_Solo_Logo.svg" alt="fedeteria"/></Link<Route>>
                </div>
                if dni.is_some(){
                    <div>
                        if (&*role_state).is_some(){
                            <h2>{"Hola " }{username}{"! Tu rol es "}{format!("{:?}", (&*role_state).clone().unwrap())}</h2>
                        } else {
                            <h2>{"Hola " }{username}{"!"}</h2>
                        }
                    </div>
                    <nav>
                        <ul class="option_list">
                            <li><Link<Route> to={Route::Profile}>{"Perfil"}</Link<Route>></li>
                            <li><a onclick={logout}>{"Cerrar Sesion"}</a></li>
                        </ul>
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