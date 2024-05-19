use crate::{router::Route, store::UserStore};
use yew_router::hooks::use_navigator;
use yewdux::use_store;
use datos_comunes::{Publicacion, ResponsePublicacion};
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::use_effect_once;

#[derive(Properties,PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(PublicationMolecule)]
pub fn publication_molecule(props : &Props) -> Html {

    let navigator = use_navigator().unwrap();

    let (store, _dispatch) = use_store::<UserStore>();
    let dni = store.dni;
    
    let id = props.id.clone();
    let datos_publicacion: UseStateHandle<Option<Publicacion>> = use_state(|| None);
    let datos_publicacion_setter = datos_publicacion.setter();

    let cloned_id = id.clone();
    use_effect_once(move || {
        if dni.is_none(){
            navigator.push(&Route::LogInPage);
        } else {
            let id = cloned_id;
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
        }
        || {}
    });

    html!{
        <div class="publication-box">
            if let Some(publicacion) = &*datos_publicacion {
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
                    <h5 class="description">{publicacion.descripcion.clone()}</h5>
                </div>
            } else {
                {"Cargando..."}
            }
        </div>
    }
}