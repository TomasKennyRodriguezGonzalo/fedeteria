use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::components::publication_thumbnail::PublicationThumbnail;
use datos_comunes::{Publicacion, QueryPublicacionesUsuario, ResponsePublicacionesUsuario};
use yew_router::prelude::*;
use yewdux::prelude::*;
use crate::store::UserStore;

#[function_component(MyPublicationsPage)]
pub fn my_publications_page() -> Html {


    let (store, dispatch) = use_store::<UserStore>();
    let dni = store.dni;
    
    let publication_list_state: UseStateHandle<Vec<String>> = use_state(|| Vec::new());

    let first_load = use_state(|| true);

    let cloned_dni = dni.clone();
    let cloned_publication_list_state = publication_list_state.clone();
    let cloned_first_load = first_load.clone();
    use_effect(move || {
        if (&*cloned_first_load).clone() {
            // traigo todas las publicaciones
            spawn_local(async move{
                let query = QueryPublicacionesUsuario{dni : cloned_dni.clone().unwrap()};
                let respuesta = Request::post("/api/obtener_mis_publicaciones").header("Content-Type", "application/json").body(serde_json::to_string(&query).unwrap()).send().await;
                match respuesta{
                Ok(respuesta) => {
                    let respuesta: Result<ResponsePublicacionesUsuario, reqwasm::Error> = respuesta.json().await;
                    match respuesta{
                        Ok(respuesta) => {
                                let publicaciones = respuesta;
                                log::info!("ids de todas las publicaciones: {publicaciones:?}");
                                cloned_publication_list_state.set(publicaciones);
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
            
            cloned_first_load.set(false)
        }
    });

    html!(
        <div class="my-publications-box">
            <div class="title">
                <h1>{"Tus Publicaciones"}</h1>
            </div>
            <div class="publication-grid">
            if !(&*publication_list_state).is_empty() {
                <ul>
                    {
                        (&*publication_list_state).iter().enumerate().map(|(index, id)| {
                            html! {
                                <li><PublicationThumbnail id={id.clone()}/></li>
                            }
                        }).collect::<Html>()
                    }
                </ul>
            } else{
                <div>{"aun no tienes publicaciones"}</div>
            }
            </div>
        </div>
    )
}