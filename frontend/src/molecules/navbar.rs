use chrono::{Date, DateTime, Local, NaiveDate, TimeZone};
use web_sys::window;
use datos_comunes::{RolDeUsuario, ResponseGetUserRole, QueryGetUserRole};
use wasm_bindgen_futures::spawn_local;
use yew_router::hooks::use_navigator;
use yewdux::use_store;
use yew_router::prelude::Link;
use yew::prelude::*;
use crate::components::dni_input_field::DniInputField;
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

    let state_product_to_search = use_state(|| None);
    let state_product_to_search_cloned = state_product_to_search.clone();
    let product_name_change = Callback::from(move |value: String| {
        state_product_to_search_cloned.set(Some(value));
    });
    let state_product_to_search_clone = state_product_to_search.clone();
    let navigator_cloned = navigator.clone();

    /*let default_local_date: DateTime<Local> = Local.with_ymd_and_hms(1,1,1,1,1,1).unwrap();
    let state_min_date = use_state(|| default_local_date);
    let state_min_date_cloned = state_min_date.clone();
    let full_min_date_changed = Callback::from(move |new_date: String|{

        let parsed_date = NaiveDate::parse_from_str(&new_date, "%Y-%m-%d");


        let new_date = parsed_date.unwrap();

        // let time = NaiveTime::from_hms_opt(0, 0, 0);

        let naive_datetime = new_date.and_hms_opt(0, 0, 0).unwrap();

        let new_date: DateTime<Local> = Local.from_local_datetime(&naive_datetime)

            .single()

            .expect("Error al convertir NaiveDateTime a DateTime<Local>");

        state_min_date_cloned.set(new_date);

        log::info!("{:?}", (&*state_min_date_cloned))
    });
    let state_min_date_cloned = state_min_date.clone();

    let default_local_date: DateTime<Local> = Local.with_ymd_and_hms(1,1,1,1,1,1).unwrap();
    let state_max_date = use_state(|| default_local_date);
    let state_max_date_cloned = state_max_date.clone();
    let full_max_date_changed = Callback::from(move |new_date: String|{

        let parsed_date = NaiveDate::parse_from_str(&new_date, "%Y-%m-%d");


        let new_date = parsed_date.unwrap();

        // let time = NaiveTime::from_hms_opt(0, 0, 0);

        let naive_datetime = new_date.and_hms_opt(0, 0, 0).unwrap();

        let new_date: DateTime<Local> = Local.from_local_datetime(&naive_datetime)

            .single()

            .expect("Error al convertir NaiveDateTime a DateTime<Local>");

        state_max_date_cloned.set(new_date);
        log::info!("{:?}", (&*state_max_date_cloned))

    });
    let state_max_date_cloned = state_max_date.clone();
    */

    //PROBARLO
    let dni_state:UseStateHandle<Option<u64>> = use_state(|| None);
    let cloned_dni_state = dni_state.clone();
    let dni_changed = Callback::from(move |dni:String|{
            cloned_dni_state.set(Some(dni.parse::<u64>().unwrap()));
    });
    let cloned_dni_state = dni_state.clone();

    //PROBARLO
    let min_price_state:UseStateHandle<Option<u64>> = use_state(|| None);
    let cloned_min_price_state = min_price_state.clone();
    let min_price_changed = Callback::from(move |price:String|{
            cloned_min_price_state.set(Some(price.parse::<u64>().unwrap()));
    });
    let cloned_min_price_state = min_price_state.clone();

    //PROBARLO
    let max_price_state:UseStateHandle<Option<u64>> = use_state(|| None);
    let cloned_max_price_state = max_price_state.clone();
    let max_price_changed = Callback::from(move |price:String|{
            cloned_max_price_state.set(Some(price.parse::<u64>().unwrap()));
    });
    let cloned_max_price_state = max_price_state.clone();

    let search_products = Callback::from(move |()| {
        let state_product_to_search_string = &*state_product_to_search_clone;
        //let state_min_date_cloned = (&*state_min_date_cloned).clone();
        //(let state_max_date_cloned = (&*state_max_date_cloned).clone();
        let cloned_dni_state = &*cloned_dni_state;
        let cloned_min_price_state = &*cloned_min_price_state;
        let cloned_max_price_state = &*cloned_max_price_state;
        let search_query = QueryPublicacionesFiltradas {
            filtro_dni: cloned_dni_state.clone(), 
            filtro_nombre: state_product_to_search_string.clone(), 
            filtro_fecha_min: None, 
            filtro_fecha_max: None, 
            filtro_precio_max: cloned_max_price_state.clone(), 
            filtro_precio_min: cloned_min_price_state.clone()
        };
        navigator_cloned.push_with_query(&Route::SearchResults, &search_query);

        if let Some(window) = window() {
            window.location().reload().unwrap();
        }
    });
        
    html!{
        <>
            <header class="navbar">
                <div class="core">
                    <div class="logo">
                        <Link<Route> to={Route::Home}><img src="/assets/img/Fedeteria_Solo_Logo.svg" alt="fedeteria"/></Link<Route>>
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
                </div>
                <div class="search-bar">
                    <h1 class="title">{"Barra de búsqueda"}</h1>
                    <div class="inputs">
                        <div class="input-fields">
                            //<CheckedInputField name="product-min-date" label="Aplicar filtro por fecha más antigua" tipo="date" on_change={full_min_date_changed}/>
                            //<CheckedInputField name="product-max-date" label="Aplicar filtro por fecha más reciente" tipo="date" on_change={full_max_date_changed}/>
                            <CheckedInputField name="product-name" placeholder="Titulo" tipo="text" on_change={product_name_change}/>
                            <DniInputField dni = "dni" placeholder="Filtro de DNI" tipo = "camp-dni" handle_on_change = {dni_changed} />
                            <DniInputField dni = "precio-minimo" placeholder="Filtro de precio minimo" tipo = "camp-min-price" handle_on_change = {min_price_changed} />
                            <DniInputField dni = "precio-maximo" placeholder="Filtro de precio máximo" tipo = "camp-max-price" handle_on_change = {max_price_changed} />
                        </div>
                        <GenericButton text="Buscar" onclick_event={search_products}/>
                    </div>
                </div>
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