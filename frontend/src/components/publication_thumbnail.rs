use std::ops::Deref;

use datos_comunes::*;
use yew_hooks::use_effect_once;
use yew_router::prelude::Link;
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::Request;

use crate::router::Route;

#[derive(Properties,PartialEq)]
pub struct PublicationThumbnailProps {
    pub id: String,
}

#[function_component(PublicationThumbnail)]
pub fn publication_thumbnail(props: &PublicationThumbnailProps) -> Html {
    let id = props.id.clone();
    let datos_publicacion: UseStateHandle<Option<Publicacion>> = use_state(|| None);
    let datos_publicacion_setter = datos_publicacion.setter();

    let idc = id.clone();
    use_effect_once(move || {
        let id = idc;
        spawn_local(async move {
            let respuesta = Request::get(&format!("/api/datos_publicacion?id={id}")).send().await;
            match respuesta{
                Ok(respuesta) => {
                    let respuesta: Result<ResponsePublicacion, reqwasm::Error> = respuesta.json().await;
                    match respuesta{
                        Ok(respuesta) => {
                            match respuesta {
                                Ok(publicacion) => {
                                    log::info!("Datos de publicacion!: {publicacion:?}");
                                    datos_publicacion_setter.set(Some(publicacion));
                                },
                                Err(error) => {
                                    log::error!("Error de publicacion: {error:?}. TODO INFORMAR BIEN?");
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

        || {}
    });
    
    html! {
        <Link<Route> to={Route::Publication{id}}>
            <div class="publication">
                if let Some(publicacion) = datos_publicacion.deref() {
                    <img src={
                        format!("/publication_images/{}", publicacion.imagenes[0])
                    }/>
                    <div class="info">
                        <h4 class="name">{publicacion.titulo.clone()}</h4>
                        <h2 class="price">{
                            if let Some(precio) = publicacion.precio {
                                precio.to_string()
                            }
                            else {
                                "Sin Tasar".to_string()
                            }
                        }</h2>
                    </div>
                } else {
                    {"Cargando..."}
                }
            </div>
        </Link<Route>>
    }
}
