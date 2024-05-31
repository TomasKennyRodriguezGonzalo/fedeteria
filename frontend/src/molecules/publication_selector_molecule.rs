use std::clone;
use std::ops::Deref;

use yew_router::hooks::use_location;
use web_sys::window;
use crate::components::generic_button::GenericButton;
use crate::components::indexed_button::IndexedButton;
use crate::components::publication_thumbnail::PublicationThumbnail;
use crate::molecules::publication_grid_molecule::PublicationGridMolecule;
use crate::request_post;
use crate::{router::Route, store::UserStore};
use yew_router::hooks::use_navigator;
use yewdux::use_store;
use datos_comunes::{calcular_rango, Publicacion, QueryEliminarPublicacion, QueryPublicacionesFiltradas, QueryTogglePublicationPause, ResponseEliminarPublicacion, ResponsePublicacion, ResponsePublicacionesFiltradas, ResponseTogglePublicationPause};
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use crate::information_store::InformationStore;
use crate::molecules::confirm_prompt_button_molecule::ConfirmPromptButtonMolecule;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub price : u64,
}

#[function_component(PublicationSelectorMolecule)]
pub fn publication_selector_molecule (props: &Props) -> Html {

    let (store, _dispatch) = use_store::<UserStore>();
    let dni = store.dni;

    let cloned_price = props.clone().price;
    let price_range = calcular_rango(cloned_price); 

    let selected_publications_list_state: UseStateHandle<Vec<usize>> = use_state(|| vec![]);
    
    let filtered_publications = use_state(|| Vec::new());
    let filtered_publications_cloned = filtered_publications.clone();
    let cloned_price_range = price_range.clone();
    use_effect_once(move || {
        
        let query = QueryPublicacionesFiltradas {
            filtro_dni : dni,
            filtro_nombre : None,
            filtro_precio_min : None, 
            filtro_precio_max : Some(*cloned_price_range.end()),
            filtro_fecha_max : None,
            filtro_fecha_min : None,
            filtro_pausadas : true,
        };
        
        request_post("/api/obtener_publicaciones", query, move |respuesta: ResponsePublicacionesFiltradas| {
            filtered_publications_cloned.set(respuesta)
        });
        ||{}
    });
    
    let filtered_publications_cloned = filtered_publications.clone();
    
    let cloned_selected_publications_list_state = selected_publications_list_state.clone();
    let publication_selected = Callback::from( move |id| {
        // Logica de seleccion de una publicacion
        if (*cloned_selected_publications_list_state).len() <= 1 {
            let mut new_vec = cloned_selected_publications_list_state.deref().clone();
            new_vec.push(id);
            cloned_selected_publications_list_state.set(new_vec);
        }
    });
    
    let cloned_selected_publications_list_state: UseStateHandle<Vec<usize>> = selected_publications_list_state.clone();
    let publication_unselected = Callback::from( move|id| {
        let mut new_vec = cloned_selected_publications_list_state.deref().clone();
        if let Some(index) = new_vec.iter().position(|index| *index == id) {
            new_vec.remove(index);
        } else {

        }
        cloned_selected_publications_list_state.set(new_vec);
    });
    let cloned_selected_publications_list_state: UseStateHandle<Vec<usize>> = selected_publications_list_state.clone();
    
    html! {
        <div class="publication-selector-box">
            if !filtered_publications_cloned.is_empty() {
                <ul> 
                    {
                        filtered_publications_cloned.iter().enumerate().map(|(index, id)| {
                            html! {
                                <li>
                                    <a class="link-duller">
                                        <PublicationThumbnail id={id} linkless={true}/>
                                    </a>
                                    if !(&*cloned_selected_publications_list_state.clone()).contains(&id) {
                                        <IndexedButton text="Seleccionar" index={id.clone()} onclick_event={publication_selected.clone()}/>
                                    } else {
                                        <IndexedButton text="Seleccionada" index={id.clone()} onclick_event={publication_unselected.clone()}/>
                                    }
                                </li>
                            }
                        }).collect::<Html>()
                    }
                </ul>
            }
        </div>
    }
}