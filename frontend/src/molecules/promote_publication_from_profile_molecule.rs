use std::ops::Deref;

use chrono::{DateTime, Local, NaiveDate, TimeZone};
use datos_comunes::{Publicacion, QueryPagarPromocionPublicaciones, QueryPublicacionesFiltradas, ResponsePublicacion, ResponsePublicacionesFiltradas};
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yewdux::use_store;
use yew_router::{components::Link, hooks::use_navigator};

use crate::{components::{generic_button::GenericButton, indexed_button::IndexedButton, publication_thumbnail::PublicationThumbnail}, information_store::InformationStore, molecules::confirm_prompt_button_molecule::ConfirmPromptButtonMolecule, request_post, router::Route, store::UserStore};

#[function_component(PromotePublicationFromProfileMolecule)]
pub fn promote_publication_from_profile_molecule () -> Html {
    let (_information_store, information_dispatch) = use_store::<InformationStore>();

    let navigator = use_navigator().unwrap();
    let (store, _dispatch) = use_store::<UserStore>();
    let dni = store.dni;

    //precio acumulado
    let total_to_pay = use_state(|| 0);
    let total_to_pay_cloned = total_to_pay.clone();

    //logica selector
    let filtered_publications = use_state(|| Vec::new());
    let selected_publications_list_state: UseStateHandle<Vec<usize>> = use_state(|| vec![]);
    let cloned_selected_publications_list_state = selected_publications_list_state.clone();
    let publication_selected = Callback::from( move |id : usize| {
        // Logica de seleccion de una publicacion
        log::info!("Indice recibido {id}");
        let mut new_vec = cloned_selected_publications_list_state.deref().clone();
        let index = id.clone();
        let total_to_pay_cloned = total_to_pay_cloned.clone();
        spawn_local(async move {
            let respuesta = Request::get(&format!("/api/datos_publicacion?id={index}")).send().await;
            match respuesta{
                Ok(respuesta) => {
                    let respuesta: Result<ResponsePublicacion, reqwasm::Error> = respuesta.json().await;
                    match respuesta{
                        Ok(respuesta) => {
                            match respuesta {
                                Ok(_publicacion) => {},
                                Err(error) => {
                                    log::error!("Error de publicacion: {error:?}.");
                                }
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
        new_vec.push(id);
        cloned_selected_publications_list_state.set(new_vec.clone());
        total_to_pay_cloned.set(new_vec.len() as u64 * 1000);
    });

    let total_to_pay_cloned = total_to_pay.clone();
    let cloned_selected_publications_list_state: UseStateHandle<Vec<usize>> = selected_publications_list_state.clone();
    let publication_unselected = Callback::from( move|id : usize| {
        log::info!("Indice recibido {id}");
        let mut new_vec = cloned_selected_publications_list_state.deref().clone();
        let i = id.clone();
        let total_to_pay_cloned = total_to_pay_cloned.clone();
        if let Some(index) = new_vec.iter().position(|index| *index == id) {
            spawn_local(async move {
                let respuesta = Request::get(&format!("/api/datos_publicacion?id={i}")).send().await;
                match respuesta{
                    Ok(respuesta) => {
                        let respuesta: Result<ResponsePublicacion, reqwasm::Error> = respuesta.json().await;
                        match respuesta{
                            Ok(respuesta) => {
                                match respuesta {
                                    Ok(_publicacion) => {},
                                    Err(error) => {
                                        log::error!("Error de publicacion: {error:?}.");
                                    }
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
            new_vec.remove(index);
        } else {

        }
        cloned_selected_publications_list_state.set(new_vec.clone());
        total_to_pay_cloned.set(new_vec.len() as u64 * 1000);
    });

    //buscador publicaciones
    let dni_cloned = dni.clone();
    let filtered_publications_cloned = filtered_publications.clone();
    use_effect_once(move || {

        let dni_cloned = dni_cloned.clone();
        let filtered_publications_cloned = filtered_publications_cloned.clone();

        let query = QueryPublicacionesFiltradas {
            filtro_dni : dni_cloned.clone(),
            filtro_nombre : None,
            filtro_precio_min : None, 
            filtro_precio_max : None,
            filtro_fecha_max : None,
            filtro_fecha_min : None,
            filtro_pausadas : false,
            excluir_promocionadas : true,
            excluir_en_trueque : true,
        };
        
        request_post("/api/obtener_publicaciones", query, move |respuesta: ResponsePublicacionesFiltradas| {
            log::info!("Datos de publicacion!: {respuesta:?}");
            filtered_publications_cloned.set(respuesta);
        });
        || {}
    });

    //estado fecha
    let fecha_state = use_state(|| "".to_string());
    let fecha_state_cloned = fecha_state.clone();
    let cloned_selected_publications_list_state: UseStateHandle<Vec<usize>> = selected_publications_list_state.clone();
    let date_changed = Callback::from(move |event : Event| {
        let target = event.target().unwrap();
        let input:HtmlInputElement = target.unchecked_into();
        let input_value = input.value();
        fecha_state_cloned.set(input_value);
    });

    //botones de confirmacion de pago
    let information_dispatch_cloned = information_dispatch.clone();
    let fecha_state_cloned = fecha_state.clone();
    let confirm_button = use_state(||false);
    let confirm_button_clone = confirm_button.clone();
    let total_to_pay_cloned = total_to_pay.clone();
    let show_confirm_button = Callback::from(move|_|{
        //obtengo la fecha en DateTime
        let fecha_seleccionada = (&*fecha_state_cloned).clone();
        let fecha_seleccionada = fecha_seleccionada.to_string();
        log::info!("FECHA {:?}", fecha_seleccionada);
        let fecha_seleccionada = NaiveDate::parse_from_str(&fecha_seleccionada, "%Y-%m-%d").unwrap();
        let fecha_seleccionada = Local.from_local_datetime(&fecha_seleccionada.into()).unwrap();
        
        //verifico que la fecha seleccionada sea mayor a la actual, de ser asi, voy a pagar
        let fecha_actual: DateTime<Local> = Local::now();
        if fecha_seleccionada > fecha_actual {
            //obtengo la cantidad de dias
            // Calcula la diferencia
            let cant_dias_promocion = fecha_seleccionada.signed_duration_since(fecha_actual).num_days();
            let precio_a_setear = (&*total_to_pay_cloned).clone() + (cant_dias_promocion as u64 * 200);
            total_to_pay_cloned.set(precio_a_setear);
            confirm_button_clone.set(true);
        }
        else {
            information_dispatch_cloned.reduce_mut( |store| store.messages.push(format!("La fecha ingresada debe ser mayor a la actual")));
        }
    });

    let filtered_publications_cloned = filtered_publications.clone();
    
    //PARA LAS CUENTAS, USAR selected_publications_list_state, NO EL CLONE DE ESTE
    //para las cuentas: 1000 por publicacion, 200 por la cantidad de dias a partir de la actual
    //voy a pagar
    let total_to_pay_cloned = total_to_pay.clone();
    let fecha_state_cloned = fecha_state.clone();
    let navigator_cloned = navigator.clone();
    let go_to_pay = Callback::from(move |_e| {

        let cloned_selected_publications_list_state: UseStateHandle<Vec<usize>> = cloned_selected_publications_list_state.clone();

        //obtengo la fecha en DateTime
        let fecha_seleccionada = (&*fecha_state_cloned).clone();
        let fecha_seleccionada = fecha_seleccionada.to_string();
        log::info!("FECHA {:?}", fecha_seleccionada);
        let fecha_seleccionada = NaiveDate::parse_from_str(&fecha_seleccionada, "%Y-%m-%d").unwrap();
        let fecha_seleccionada = Local.from_local_datetime(&fecha_seleccionada.into()).unwrap();
        let vec_string = serde_json::to_string(&*cloned_selected_publications_list_state).unwrap();
        let query = QueryPagarPromocionPublicaciones {
            publicaciones: vec_string,
            fecha_fin_promocion: fecha_seleccionada,
            precio: (&*total_to_pay_cloned).clone(),
        };
        let _ = navigator_cloned.push_with_query(&Route::PayPublicationPromotion, &query);
    });

    let confirm_button_clone = confirm_button.clone();
    let hide_confirm_button = Callback::from(move|_|{
        confirm_button_clone.set(false);
    });
    
    let cloned_selected_publications_list_state: UseStateHandle<Vec<usize>> = selected_publications_list_state.clone();

    let total_to_pay_cloned = total_to_pay.clone();
    let mensaje = format!("¿Seguro que desea promocionar las publicaciones seleccionadas por ${}?", (&*total_to_pay_cloned).clone());


    html!(
        <div> //promocionar publicacion box
            <div class="publication-selector-box"> //selector de publicaciones
                if !filtered_publications_cloned.is_empty() {
                    <ul> 
                        {
                            filtered_publications_cloned.iter().map(|id| {
                                html! {
                                    <li>
                                        <a class="link-duller">
                                            <br/>
                                            <PublicationThumbnail id={id} linkless={true}/>
                                            <br/>
                                        </a>
                                        if !(&*cloned_selected_publications_list_state.clone()).contains(&id) {
                                            <IndexedButton text="Seleccionar" index={id.clone()} onclick_event={publication_selected.clone()} disabled={true}/>
                                            <br/>
                                        } else {
                                            <IndexedButton text="Seleccionada" index={id.clone()} onclick_event={publication_unselected.clone()}/>
                                            <br/>
                                        }
                                    </li>
                                }
                            }).collect::<Html>()
                        }
                    </ul>
                } else {
                    <h1>{"No tienes publicaciones para promocionar"}</h1>
                }
            </div>
            if !(&*cloned_selected_publications_list_state.clone()).is_empty() {
                <div class="publication-selector-box">
                    <h2>{"Ingrese la fecha hasta la que desea promocionar las publicaciones seleccionadas"}</h2>
                    <input type="date" name="fecha-trueque" onchange={date_changed}/>
                    <br/>
                    <GenericButton text="Pagar Promoción de Publicaciones Seleccionadas" onclick_event={show_confirm_button}/>
                </div>
            }
            if *confirm_button{
                <ConfirmPromptButtonMolecule text={mensaje} confirm_func={go_to_pay} reject_func={hide_confirm_button} />
            }
        </div>
    )
}