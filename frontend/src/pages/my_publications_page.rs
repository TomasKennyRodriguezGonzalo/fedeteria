use reqwasm::http::Request;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use crate::{components::publication_thumbnail::PublicationThumbnail, request_post};
use datos_comunes::{Publicacion, QueryPublicacionesFiltradas, ResponsePublicacionesFiltradas};
use yew_router::prelude::*;
use yewdux::prelude::*;
use crate::store::UserStore;




#[function_component(MyPublicationsPage)]
pub fn my_publications_page() -> Html {


    let (store, dispatch) = use_store::<UserStore>();
    let dni = store.dni;
    
    let publication_list_state: UseStateHandle<Vec<usize>> = use_state(|| Vec::new());

    


    let cloned_dni = dni.clone();
    let cloned_publication_list_state = publication_list_state.clone();
    use_effect_once(move || {
        // traigo todas las publicaciones
        let query = QueryPublicacionesFiltradas
        {
            filtro_dni : Some(cloned_dni.clone().unwrap()),
            filtro_nombre: None,
            filtro_fecha_min: None,
            filtro_fecha_max: None,
        };
        request_post("/api/obtener_publicaciones", query, move |respuesta: ResponsePublicacionesFiltradas| {
            let publicaciones = respuesta;
            log::info!("ids de todas las publicaciones: {publicaciones:?}");
            cloned_publication_list_state.set(publicaciones);
        });
        || {}
    });








    html!(
        <>
            <div class="publication-grid">
            if !(&*publication_list_state).is_empty() {
                <ul>
                    {
                        (&*publication_list_state).iter().map(|id| {

                            html! {
                                <li><PublicationThumbnail id={id.to_string()}/></li>
                            }
                        }).collect::<Html>()
                    }
                </ul>
            } else{
                <div>{"aun no tienes publicaciones"}</div>
            }
            </div>
        </>
    )
}