use std::clone;

use web_sys::window;
use crate::components::generic_button::GenericButton;
use crate::request_post;
use crate::{router::Route, store::UserStore};
use yew_router::hooks::use_navigator;
use yewdux::use_store;
use datos_comunes::{Publicacion, QueryEliminarPublicacion, QueryTogglePublicationPause, ResponseEliminarPublicacion, ResponsePublicacion, ResponseTogglePublicationPause};
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use crate::information_store::InformationStore;
use crate::molecules::confirm_prompt_button_molecule::ConfirmPromptButtonMolecule;

#[derive(Properties,PartialEq)]
pub struct Props {
    pub id: usize,
}

#[function_component(PublicationMolecule)]
pub fn publication_molecule(props : &Props) -> Html {

    let navigator = use_navigator().unwrap();

    let (_information_store, information_dispatch) = use_store::<InformationStore>();

    let (store, _dispatch) = use_store::<UserStore>();
    let dni = store.dni;
    
    let id = props.id.clone();
    let datos_publicacion: UseStateHandle<Option<Publicacion>> = use_state(|| None);
    let datos_publicacion_setter = datos_publicacion.setter();

    let cloned_information_dispatch = information_dispatch.clone();
    let cloned_id = id.clone();
    use_effect_once(move || {
        if dni.is_none(){
            navigator.push(&Route::LogInPage);
            cloned_information_dispatch.reduce_mut(|store| store.messages.push("Para acceder a una publicación debes iniciar sesión".to_string()))
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

    let (_information_store, information_dispatch) = use_store::<InformationStore>();
    let information_dispatch = information_dispatch.clone();
    let cloned_id = id.clone();
    let navigator = use_navigator().unwrap();
    let cloned_navigator = navigator.clone();
    let delete_publication = Callback::from(move |_e:MouseEvent|{
        let cloned_navigator = cloned_navigator.clone();
        let information_dispatch = information_dispatch.clone();
        let cloned_id = cloned_id.clone();
        let query = QueryEliminarPublicacion
        {
            id : cloned_id
        };
        request_post("/api/eliminar_publicacion", query, move |respuesta: ResponseEliminarPublicacion| {
            //si se elimino bien ok sera true
            let cloned_navigator = cloned_navigator.clone();
            let ok = respuesta.ok;
            let information_dispatch = information_dispatch.clone();
            information_dispatch.reduce_mut(|store| store.messages.push("La publicacion ha sido eliminada correctamente".to_string()));
            log::info!("resultado de eliminar publicacion : {ok}");
            cloned_navigator.push(&Route::Home);
        });
    });

    let cloned_datos_publicacion = datos_publicacion.clone();
    
    let (_information_store, information_dispatch) = use_store::<InformationStore>();
    let information_dispatch = information_dispatch.clone();
    let cloned_id = id.clone();
    let toggle_publication_pause = Callback::from(move |()| {
        let cloned_datos_publicacion = cloned_datos_publicacion.clone();
        let id = cloned_id.clone();
        let information_dispatch = information_dispatch.clone();
        spawn_local(async move{
            let cloned_datos_publicacion = cloned_datos_publicacion.clone();
            let information_dispatch = information_dispatch.clone();
            let query = QueryTogglePublicationPause{id : id.clone()};
            let response = Request::post("/api/alternar_pausa_publicacion").header("Content-Type", "application/json").body(serde_json::to_string(&query).unwrap()).send().await;
            match response{
            Ok(response) => {
                let cloned_datos_publicacion = cloned_datos_publicacion.clone();
                let response: Result<ResponseTogglePublicationPause, reqwasm::Error> = response.json().await;
                let information_dispatch = information_dispatch.clone();
                match response {
                    Ok(response) => {
                        let cloned_datos_publicacion = cloned_datos_publicacion.clone();
                        let information_dispatch = information_dispatch.clone();
                        if response.changed {
                            let nombre = ((&*cloned_datos_publicacion).clone().unwrap().titulo.clone());
                            let publicacion_pausada = (&*cloned_datos_publicacion).clone().unwrap().pausada.clone();
                            let information_dispatch = information_dispatch.clone();
                            if (publicacion_pausada).clone() {
                                information_dispatch.reduce_mut(|store| store.messages.push(format!("La publicacion {} ha sido despasuada con exito",nombre.clone())));
                            } else {
                                information_dispatch.reduce_mut(|store| store.messages.push(format!("La publicacion {} ha sido pausada con exito",nombre.clone())));
                            }
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

    let activate_delete_publication_state = use_state(||false);
    let cloned_activate_delete_publication_state = activate_delete_publication_state.clone();

    let activate_delete_publication = Callback::from(move |()|{
        let cloned_activate_delete_publication_state = cloned_activate_delete_publication_state.clone();
        cloned_activate_delete_publication_state.set(true);
    });

    let cloned_activate_delete_publication_state = activate_delete_publication_state.clone();
    let reject_func = Callback::from(move |_e:MouseEvent|{
        let cloned_activate_delete_publication_state = cloned_activate_delete_publication_state.clone();
        cloned_activate_delete_publication_state.set(false);
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
                    <GenericButton text="Eliminar Publicación" onclick_event={activate_delete_publication}/>
                    if (&*activate_delete_publication_state).clone(){
                        <ConfirmPromptButtonMolecule text="Seguro que quiere eliminar su publicacion?" confirm_func={delete_publication} reject_func={reject_func} />
                    }
                    if publicacion.precio.is_some() {
                        if publicacion.pausada {
                            <GenericButton text="Despausar Publicación" onclick_event={toggle_publication_pause}/>
                        } else {
                            <GenericButton text="Pausar Publicación" onclick_event={toggle_publication_pause}/>
                        }
                    }  
                </div>
                }
            } else {
                {"Cargando..."}
            }
            </div>
        }
}