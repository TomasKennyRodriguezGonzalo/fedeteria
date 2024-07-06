use std::clone;

use crate::{components::{indexed_button::IndexedButton, publication_thumbnail::PublicationThumbnail}, pages, request_post};
use datos_comunes::{QueryPublicacionesFiltradas, ResponsePublicacionesFiltradas};
use yew::prelude::*;
use yew_hooks::use_effect_once;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub query: Option<QueryPublicacionesFiltradas>,
    #[prop_or(15)]
    pub quantity: u8,
}

#[function_component(PublicationGridMolecule)]
pub fn publication_grid_molecule(props: &Props) -> Html { 
    let publications_pages_list_state: UseStateHandle<Vec<Vec<usize>>> = use_state(|| vec![vec![]]);
    
    let publication_page_index = use_state(||(1, 0));
    let index_buttons: UseStateHandle<Vec<usize>> = use_state(|| vec![]);
    
    let cloned_index_buttons = index_buttons.clone();
    let cloned_publication_page_index = publication_page_index.clone();
    let cloned_publications_pages_list_state: UseStateHandle<Vec<Vec<usize>>> = publications_pages_list_state.clone();
    let props_clone = (*props).clone();
    use_effect_once(move || {
        // traigo todas las publicaciones
        let props_clone = props_clone.clone();
        let cloned_publication_page_index = cloned_publication_page_index.clone();
        let mut query = QueryPublicacionesFiltradas
        {
            filtro_dni: None,
            filtro_nombre: None,
            filtro_fecha_min: None,
            filtro_fecha_max: None,
            filtro_precio_max: None,
            filtro_precio_min: None,
            filtro_pausadas : true,
        };
        if let Some(query_options) = props_clone.query.clone() {
            query = query_options;
        }
        let props_clone = props_clone.clone();
        let cloned_publications_pages_list_state: UseStateHandle<Vec<Vec<usize>>> = cloned_publications_pages_list_state.clone();
        request_post("/api/obtener_publicaciones", query, move |respuesta: ResponsePublicacionesFiltradas| {
            let publicaciones = respuesta;
            let chunks = publicaciones.chunks(props_clone.clone().quantity as usize).map(|chunk| chunk.to_vec()).collect::<Vec<Vec<usize>>>();
            cloned_publications_pages_list_state.set(chunks.clone());
            cloned_publication_page_index.set((cloned_publication_page_index.0, chunks.len()));
            let mut vec = vec![];
            for id in 1..=(chunks.len()) {
                vec.push(id);
            }
            cloned_index_buttons.set(vec);
        });
        || {}
    });

    let cloned_publication_page_index = publication_page_index.clone();
    let change_page = Callback::from(move|index| {
        cloned_publication_page_index.set((index, cloned_publication_page_index.1));
    });

    html!{
        <div class="publication-grid">
            <ul>
                {
                    if let Some(page) = (*publications_pages_list_state).get((*publication_page_index).0 - 1) {
                        page.iter().map(|id| {
                            html! {
                                <li>
                                    <PublicationThumbnail id={id}/>
                                </li>
                            }
                        }).collect::<Html>()
                    } else {
                        html!(<h1>{"Cargando..."}</h1>)
                    }
                }
            </ul>
            <div class="index-buttons">
                {   
                    if (*index_buttons).len() > 1 { 
                        (*index_buttons).iter().map(|id| html!{
                            if id.clone() == (*publication_page_index).0 {
                                <IndexedButton text={id.to_string()} index={id.clone() as usize} onclick_event={change_page.clone()} disabled=true/>
                            } else {
                                <IndexedButton text={id.to_string()} index={id.clone() as usize} onclick_event={change_page.clone()}/>
                            }
                        })
                        .collect::<Html>()
                    } else {
                        html!()
                    }
                }
            </div>
        </div>
    }
}