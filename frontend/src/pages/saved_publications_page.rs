use datos_comunes::{QueryObtenerGuardadas, ResponseObtenerGuardadas};
use yew::prelude::*;
use yewdux::use_store;
use yew_hooks::use_effect_once;
use crate::{components::publication_thumbnail::PublicationThumbnail, request_post};
use crate::{molecules::unlock_account_molecule::UnlockAccountMolecule, store::UserStore};

#[function_component(SavedPublicationsPage)]
pub fn saved_publications_page() -> Html {
    let (store, _dispatch) = use_store::<UserStore>();
    let dni = store.dni;
    let publicaciones_guardadas:UseStateHandle<Vec<usize>> = use_state(||Vec::new());
    let cloned_publicaciones_guardadas = publicaciones_guardadas.clone();
    use_effect_once(move || {
       if let Some(dni) = dni{
            let query = QueryObtenerGuardadas{dni:dni};
            request_post("/api/obtener_guardadas", query, move |respuesta:ResponseObtenerGuardadas|{
                cloned_publicaciones_guardadas.set(respuesta.publicaciones_guardadas);
            });
       }

        || {}
    });



    html!(
        <>
        <div class="my-publications-box">
        <div class="title">
            <h1>{"Tus Publicaciones guardadas"}</h1>
            </div>
            <div class="publication-grid">
            if !(&*publicaciones_guardadas).is_empty() {
                <ul>
                    {
                        (&*publicaciones_guardadas).iter().enumerate().map(|(_index, id)| {
                            html! {
                                <li><PublicationThumbnail id={id}/></li>
                            }
                        }).collect::<Html>()
                    }
                </ul>
            } else{
                <div>{"aun no tienes publicaciones guardadas"}</div>
            }
            </div>
        </div>
        </>
    )
}