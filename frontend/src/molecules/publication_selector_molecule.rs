use std::clone;

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
use datos_comunes::{Publicacion, QueryEliminarPublicacion, QueryPublicacionesFiltradas, QueryTogglePublicationPause, ResponseEliminarPublicacion, ResponsePublicacion, ResponsePublicacionesFiltradas, ResponseTogglePublicationPause};
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use crate::information_store::InformationStore;
use crate::molecules::confirm_prompt_button_molecule::ConfirmPromptButtonMolecule;

#[function_component(PublicationSelectorMolecule)]
pub fn publication_selector_molecule () -> Html {
    let location = use_location().unwrap();
    let props = location.query::<QueryPublicacionesFiltradas>().unwrap();

    let filtered_publications = use_state(|| Vec::new());
    let filtered_publications_cloned = filtered_publications.clone();

    use_effect_once(move || {
        request_post("/api/obtener_publicaciones", props, move |respuesta: ResponsePublicacionesFiltradas| {
            filtered_publications_cloned.set(respuesta)
        });
        ||{}
    });


    let filtered_publications_cloned = filtered_publications.clone();

    html! {
        if !filtered_publications_cloned.is_empty() {
            <ul> 
                {
                    filtered_publications_cloned.iter().map(|id| {
                        html! {
                            <li><PublicationThumbnail id={id}/></li>
                        }
                    }).collect::<Html>()
                }
            </ul>
        }
    }
}