/*use std::ops::Deref;

use datos_comunes::{QueryPublicacionesFiltradas, ResponsePublicacion, ResponsePublicacionesFiltradas};
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{components::{checked_input_field::CheckedInputField, dni_input_field::DniInputField, generic_button::GenericButton, indexed_button::IndexedButton, publication_thumbnail::PublicationThumbnail}, request_post};

#[function_component(PromotePublicationFromOfficeMolecule)]
pub fn promote_publication_molecule () -> Html {

    //DNI
    let dni_state = use_state(|| None);
    let dni_state_cloned = dni_state.clone();
    let dni_onchange = Callback::from(move |dni: String| {
        dni_state_cloned.set(Some(dni.parse::<u64>().unwrap()));
    });

    //Nombre de Publicacion
    let publication_name_state = use_state(|| None);
    let publication_name_cloned = publication_name_state.clone();
    let publication_name_onchange = Callback::from(move |publication_name| {
        publication_name_cloned.set(Some(publication_name));
    });

    //estado selector
    let selector_state = use_state(|| false);
    let selector_state_cloned = selector_state.clone();
    let hide_selector = Callback::from(move |()| {
        selector_state_cloned.set(false);
    });

    //logica selector
    let filtered_publications = use_state(|| Vec::new());
    let selected_publications_list_state: UseStateHandle<Vec<usize>> = use_state(|| vec![]);
    let cloned_selected_publications_list_state = selected_publications_list_state.clone();
    let publication_selected = Callback::from( move |id : usize| {
        // Logica de seleccion de una publicacion
        log::info!("Indice recibido {id}");
        let mut new_vec = cloned_selected_publications_list_state.deref().clone();
        let index = id.clone();
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
        cloned_selected_publications_list_state.set(new_vec);
    });

    let cloned_selected_publications_list_state: UseStateHandle<Vec<usize>> = selected_publications_list_state.clone();
    let publication_unselected = Callback::from( move|id : usize| {
        log::info!("Indice recibido {id}");
        let mut new_vec = cloned_selected_publications_list_state.deref().clone();
        let i = id.clone();
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
        cloned_selected_publications_list_state.set(new_vec);
    });

    //buscador publicaciones
    let selector_state_cloned = selector_state.clone();
    let dni_state_cloned = dni_state.clone();
    let publication_name_state_cloned = publication_name_state.clone();
    let filtered_publications_cloned = filtered_publications.clone();
    let search_publications = Callback::from(move |()| {

        selector_state_cloned.set(true);
        let dni_state_cloned = dni_state_cloned.clone();
        let publication_name_state_cloned = publication_name_state_cloned.clone();
        let filtered_publications_cloned = filtered_publications_cloned.clone();

        let query = QueryPublicacionesFiltradas {
            filtro_dni : (&*dni_state_cloned).clone(),
            filtro_nombre : (&*publication_name_state_cloned).clone(),
            filtro_precio_min : None, 
            filtro_precio_max : None,
            filtro_fecha_max : None,
            filtro_fecha_min : None,
            filtro_pausadas : true,
            filtro_promocionadas : false,
        };
        
        request_post("/api/obtener_publicaciones", query, move |respuesta: ResponsePublicacionesFiltradas| {
            log::info!("Datos de publicacion!: {respuesta:?}");
            filtered_publications_cloned.set(respuesta);
        });
    });

    let filtered_publications_cloned = filtered_publications.clone();
    let cloned_selected_publications_list_state: UseStateHandle<Vec<usize>> = selected_publications_list_state.clone();
    
    //PARA LAS CUENTAS, USAR selected_publications_list_state, NO EL CLONE DE ESTE
    //para las cuentas: 1000 por publicacion, 200 por la cantidad de dias a partir de la actual

    html!(
        <div> //promocionar publicacion box
            <div class="edit-personal-info-box"> //buscar publicaciones
                <h2>{"Filtros:"}</h2>
                <h2>{"Ingrese el DNI del usuario que desea promocionar publicaciones"}</h2>
                <DniInputField dni = "DNI" tipo = "number" handle_on_change = {dni_onchange} />
                <br/>
                <h2>{"Ingrese el nombre de la publicacion que desea promocionar"}</h2>
                <CheckedInputField name="Nombre Publicación" placeholder="Nombre Publicacion" tipo="text" on_change={publication_name_onchange}/>
                <br/>
                <GenericButton text="Buscar Publicaciones" onclick_event={search_publications}/>
            </div> 
            <div class="publication-selector-box"> //selector de publicaciones
                if (&*selector_state).clone() {
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
                        <h1>{"No se encontraron publicaciones"}</h1>
                    }
                }
                else {
                    <h2>{"Publicaciones"}</h2>
                }
                <GenericButton text="Cancelar" onclick_event={hide_selector}/>
            </div>
        </div>
    )
}*/