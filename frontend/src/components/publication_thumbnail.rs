use std::ops::Deref;

use datos_comunes::*;
use yew_hooks::use_effect_once;
use yew_router::prelude::Link;
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::Request;
use yewdux::use_store;

use crate::{request_post, router::Route, store::UserStore};

#[derive(Properties,PartialEq)]
pub struct PublicationThumbnailProps {
    pub id: usize,
    #[prop_or_default]
    pub linkless : bool,
}

#[function_component(PublicationThumbnail)]
pub fn publication_thumbnail(props: &PublicationThumbnailProps) -> Html {
    let id = props.id.clone();
    let datos_publicacion: UseStateHandle<Option<Publicacion>> = use_state(|| None);
    let datos_publicacion_setter = datos_publicacion.setter();
    let current_id = use_state(|| usize::MAX);

    let (store, _dispatch) = use_store::<UserStore>();
    let dni = store.dni;
    let role_state: UseStateHandle<Option<RolDeUsuario>> = use_state(|| None);
    let cloned_role_state = role_state.clone();
    let cloned_dni = dni.clone();
    use_effect_once(move || {
        if let Some(dni) = cloned_dni {
            let query = QueryGetUserRole { dni };
            request_post("/api/obtener_rol", query, move |respuesta:ResponseGetUserRole|{
                cloned_role_state.set(Some(respuesta.rol));
            });
        }

        || {}
    });
    
    let cloned_current_id = current_id.clone();
    let idc = id.clone();
    use_effect(move || {
        let id = idc;
        if *cloned_current_id != id { 
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
            cloned_current_id.set(id)
        } else {

        }
        || {}
    });
    
    html! {
        if !(props.linkless) {
            <Link<Route> to={Route::Publication{id}}>
                <div class="publication-thumbnail">
                    if let Some(publicacion) = datos_publicacion.deref() {
                        <img src={
                            format!("/publication_images/{}", publicacion.imagenes[0])
                        }/>
                        <div class="info">
                            <h4 class="name">{publicacion.titulo.clone()}</h4>
                            <h2 class="price">{
                                if let Some(precio) = publicacion.precio {
                                    let mut incluir = false;
                                    if let Some(dni) = dni {
                                        if publicacion.dni_usuario == dni {
                                            incluir = true;
                                        }
                                        if let Some(role) = &*role_state {
                                            match role { 
                                                RolDeUsuario::Dueño | RolDeUsuario::Empleado{sucursal : _} => {
                                                    incluir = true;
                                                },
                                                _ => {}
                                            }
                                        }
                                    }
                                    get_string_de_rango(precio, incluir)
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
        } else {
            <div class="publication-thumbnail">
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
        }
    }
}
