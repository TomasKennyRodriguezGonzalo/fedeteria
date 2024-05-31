use datos_comunes::{QueryPublicacionesSinTasar, ResponsePublicacionesSinTasar};
use yew_hooks::use_effect_once;
use yew_router::prelude::Link;
use yew::prelude::*;
use yewdux::use_store;
use crate::components::publication_thumbnail::PublicationThumbnail;

use crate::store::UserStore;
use crate::convenient_request::request_post;

#[function_component(AwaitingPricePublicationPage)]
pub fn awaiting_price_publication_page()-> Html {

    let (store, _dispatch) = use_store::<UserStore>();
    let dni = store.dni;

    
    let publications_list_state = use_state(|| Vec::new());
    let cloned_publications_list_state = publications_list_state.clone();
    
    let query = QueryPublicacionesSinTasar{
        dni : dni.unwrap_or_default(),
    };

    use_effect_once( move || {
        request_post("/api/obtener_publicaciones_sin_tasar", query, move |respuesta:ResponsePublicacionesSinTasar|{
            let cloned_publications_list_state = cloned_publications_list_state.clone();
            let publicaciones = respuesta.publicaciones;
            cloned_publications_list_state.set(publicaciones);
        });
        
        || {}
    });

    html!{
        <div class="awaiting-price-publications-box">  
            <h1 class="title">{"Publicaciones Esperando Tasación"}</h1>
            <ul>
                {
                    (&*publications_list_state).iter().map(|id|{
                        html!{
                            <li>
                                <PublicationThumbnail id={id}/>
                            </li>
                        }
                    }).collect::<Html>()

                }
            </ul>
        </div>
    }

}