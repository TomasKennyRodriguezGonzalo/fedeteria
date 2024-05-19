use web_sys::window;
use crate::components::generic_button::GenericButton;
use crate::{router::Route, store::UserStore};
use yew_router::hooks::use_navigator;
use yewdux::use_store;
use datos_comunes::{Publicacion, QueryTogglePublicationPause, ResponsePublicacion, ResponseTogglePublicationPause};
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


    let delete_publication = Callback::from( |()|{

    });

    let cloned_id = id.clone();
    let toggle_publication_pause = Callback::from(move |()| {
        let id = cloned_id.clone();
        spawn_local(async move{
            let query = QueryTogglePublicationPause{id : id.clone()};
            let response = Request::post("/api/alternar_pausa_publicacion").header("Content-Type", "application/json").body(serde_json::to_string(&query).unwrap()).send().await;
            match response{
            Ok(response) => {
                let response: Result<ResponseTogglePublicationPause, reqwasm::Error> = response.json().await;
                match response {
                    Ok(response) => {
                        if response.changed {
                            // Refreshes to reset the first load states all over the code
                            if let Some(window) = window() {
                                window.location().reload().unwrap();
                            }
                        } else {
                            log::info!("No se cambió la publicación.")
                        }
                    }
                    Err(error) => {
                        log::error!("{:?}", error)
                    }
                }
            }
            Err(error)=>{
                log::error!("Error en llamada al backend: {}", error);
            }
        }
        });
    });

    html!{
        <div class="publication-box">
            if let Some(publicacion) = &*datos_publicacion {
                <div class="info">
                <img src={
                    format!("/publication_images/{}", publicacion.imagenes[0])
                    }/>
                    <div class="text">
                    <h4 class="name">{publicacion.titulo.clone()}</h4>
                    <h2 class="price">{
                            if publicacion.pausada {
                                "Publicación Pausada".to_string()
                            } else {
                                if let Some(precio) = publicacion.precio {
                                    precio.to_string()
                                }
                                else {
                                    "Sin Tasar".to_string()
                                }
                            }
                        }</h2>
                        <h5 class="description">{publicacion.descripcion.clone()}</h5>
                        </div>
                </div>
                if publicacion.dni_usuario == dni.clone().unwrap(){
                <div class="moderation-buttons">
                    <GenericButton text="Eliminar Publicación" onclick_event={delete_publication}/>
                    if publicacion.pausada {
                        <GenericButton text="Despausar Publicación" onclick_event={toggle_publication_pause}/>
                    } else {
                        <GenericButton text="Pausar Publicación" onclick_event={toggle_publication_pause}/>
                    }
                </div>
                }
            } else {
                {"Cargando..."}
            }
            </div>
        }
}