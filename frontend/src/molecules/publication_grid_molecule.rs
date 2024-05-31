use crate::{components::publication_thumbnail::PublicationThumbnail, request_post};
use datos_comunes::{QueryPublicacionesFiltradas, ResponsePublicacionesFiltradas};
use yew::prelude::*;
use yew_hooks::use_effect_once;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub query: Option<QueryPublicacionesFiltradas>,
}

#[function_component(PublicationGridMolecule)]
pub fn publication_grid_molecule(props: &Props) -> Html {
    let publication_list_state = use_state(|| None);
    let publication_list_state_c = publication_list_state.clone();
    let props_clone = props.query.clone();
    use_effect_once(move || {
        // traigo todas las publicaciones
        let props_clone = props_clone.clone();
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
        if let Some(query_options) = props_clone {
            query = query_options;
        }
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