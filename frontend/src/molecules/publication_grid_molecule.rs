use crate::{components::publication_thumbnail::PublicationThumbnail, request_post};
use datos_comunes::{Publicacion, QueryPublicacionesFiltradas, ResponsePublicacionesFiltradas};
use yew::prelude::*;
use yew_hooks::use_effect_once;

#[function_component(PublicationGridMolecule)]
pub fn publication_grid_molecule() -> Html {
    let publication_list_state = use_state(|| None);
    let publication_list_state_c = publication_list_state.clone();
    use_effect_once(move || {
        // traigo todas las publicaciones
        let query = QueryPublicacionesFiltradas
        {
            filtro_dni: None,
            filtro_nombre: None,
            filtro_fecha_min: None,
            filtro_fecha_max: None,
        };
        request_post("/api/obtener_publicaciones", query, move |respuesta: ResponsePublicacionesFiltradas| {
            let publicaciones = respuesta;
            log::info!("ids de todas las publicaciones: {publicaciones:?}");
            publication_list_state_c.set(Some(publicaciones));
        });
        || {}
    });

    html!{
        <div class="publication-grid">
            if publication_list_state.is_some() {
                <ul>
                    {
                        (publication_list_state).as_ref().unwrap().iter().map(|id| {
                            html! {
                                <li><PublicationThumbnail id={id}/></li>
                            }
                        }).collect::<Html>()
                    }
                </ul>
            }
        </div>
    }
}