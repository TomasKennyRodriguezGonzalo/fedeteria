use std::clone;

use web_sys::window;
use crate::components::generic_button::GenericButton;
use crate::components::indexed_button::IndexedButton;
use crate::request_post;
use crate::{router::Route, store::UserStore};
use yew_router::hooks::use_navigator;
use yewdux::use_store;
use datos_comunes::{Publicacion, QueryEliminarPublicacion, QueryGetUserRole, QueryTogglePublicationPause, ResponseEliminarPublicacion, ResponseGetUserRole, ResponsePublicacion, ResponseTogglePublicationPause, RolDeUsuario};
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use crate::information_store::InformationStore;
use crate::molecules::confirm_prompt_button_molecule::ConfirmPromptButtonMolecule;
use crate::components::checked_input_field::CheckedInputField;

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

    let role_state: UseStateHandle<Option<RolDeUsuario>> = use_state(|| None);
    let cloned_role_state = role_state.clone();
    let cloned_dni = dni.clone();
    use_effect_once(move || {
        let cloned_role_state = cloned_role_state.clone();
        let cloned_dni = cloned_dni.clone();
        if cloned_dni.is_some() {
            let query = QueryGetUserRole { dni : cloned_dni.unwrap() };
            request_post("/api/obtener_rol", query, move |respuesta:ResponseGetUserRole|{
                cloned_role_state.set(Some(respuesta.rol));
            });
        }

        || {}
    });

    
    let id = props.id.clone();
    let datos_publicacion: UseStateHandle<Option<Publicacion>> = use_state(|| None);
    let datos_publicacion_setter = datos_publicacion.setter();

    let current_image_state = use_state(|| 0);

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

    let cloned_current_image_state = current_image_state.clone();
    let change_current_image = Callback::from(move |index| {
        cloned_current_image_state.set(index);
    });

    let cloned_current_image_state = current_image_state.clone();

    //este es el estado del input, el que va cambiando dinamicamente
    let input_publication_price_state = use_state(|| None);
    let cloned_input_publication_price_state = input_publication_price_state.clone();

    let price_changed = CallBack::from(move |precio:Option<u64>|{
        let input_publication_price_state = cloned_input_publication_price_state.clone();
        input_publication_price_state.set(precio);
        log::info!("{:?}",precio);
    });

    //este es el estado de la publicacion en si, el que cambia cuando se aprieta el boton "tasar publicacion"
    let cloned_input_publication_price_state = input_publication_price_state.clone();
    let publication_price_state = use_state(|| None);
    let cloned_publication_price_state = publication_price_state.clone();
    let assign_price = CallBack::from(move |()|{
        let publication_price_state = cloned_publication_price_state.clone();
        let input_publication_price_state = cloned_input_publication_price_state.clone();

    });


    html!{
        <div class="publication-box">
            if let Some(publicacion) = &*datos_publicacion {
                <div class="info">
                <div class="image">
                    <img src={
                        format!("/publication_images/{}", publicacion.imagenes[*cloned_current_image_state])
                    }/>
                    <div class="index-buttons">
                        {
                            publicacion.imagenes.iter().enumerate().map(|(index, _imagen)| {
                                if (*cloned_current_image_state == index) {
                                    html! {
                                        <button class="selected-button"></button>
                                    }
                                } else {
                                    html! {
                                        <IndexedButton index={index} text="" onclick_event={change_current_image.clone()}></IndexedButton>
                                    }
                                }
                            }).collect::<Html>()
                        }
                    </div> 
                </div> 
                    <div class="text">
                    <h3> {format!("DNI del dueño: {}", publicacion.dni_usuario) } </h3>
                    <h4 class="name">{publicacion.titulo.clone()}</h4>
                    <h2 class="price">{
                        if let Some(precio) = publicacion.precio {
                            if publicacion.pausada {
                                "Publicación Pausada".to_string()
                            } else {
                                precio.to_string()
                            }
                        } else {
                            "Sin Tasar".to_string()
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
                    }  else {
                        {
                            {
                                match (&*role_state).clone().unwrap() { 
                                    RolDeUsuario::Dueño => {
                                        html! {<GenericButton text="Tasar Publicación" onclick_event={assign_price}/>}
                                    },
                                    RolDeUsuario::Empleado{sucursal : _} => {
                                        html! {
                                            <>
                                                <CheckedInputField name = "publication_price_assignment" label="Ingrese el precio de la publicación" tipo = "number" on_change = {price_changed} />
                                                <GenericButton text="Tasar Publicación" onclick_event={assign_price}/>
                                            </>
                                        }
                                    },
                                    RolDeUsuario::Normal => {
                                        html!{}
                                    }
                                }
                            }

                                
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