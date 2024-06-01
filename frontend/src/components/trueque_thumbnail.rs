use std::ops::Deref;

use datos_comunes::*;
use yew_hooks::use_effect_once;
use yew_router::prelude::Link;
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::Request;

use crate::router::Route;

#[derive(Properties,PartialEq)]
pub struct TruequeThumbnailProps {
    pub id_trueque: usize,
    #[prop_or_default]
    pub linkless : bool,
}

#[function_component(TruequeThumbnail)]
pub fn trueque_thumbnail (props: &TruequeThumbnailProps) -> Html {
    let id = props.id_trueque.clone();
    let state_trueque: UseStateHandle<Option<Trueque>> = use_state(|| None);
    let state_trueque_setter = state_trueque.setter();

    //use el llamado al back primitivo, porque creo que es un llamado distinto al simplificado
    let id_trueque_cloned = id.clone();
    use_effect_once(move || {
        let id_trueque_cloned = id_trueque_cloned;
        spawn_local(async move {
            let respuesta = Request::get(&format!("/api/obtener_trueque?id={id_trueque_cloned}")).send().await;
            match respuesta{
                Ok(respuesta) => {
                    let respuesta: Result<ResponseObtenerTrueque, reqwasm::Error> = respuesta.json().await;
                    match respuesta{
                        Ok(respuesta) => {
                            match respuesta {
                                Ok(trueque) => {
                                    log::info!("Trueque: {:?}", trueque);
                                    state_trueque_setter.set(Some(trueque));
                                },
                                Err(error) => {
                                    log::error!("Error de trueque: {:?}", error);
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
        if !(props.linkless) {
            <Link<Route> to={Route::Trueque{id}}>
                <div class="publication-thumbnail">
                    if let Some(trueque) = state_trueque.deref() {
                        <h1>{"Hay Trueque con ruta"}</h1>
                    } else {
                        {"Cargando..."}
                    }
                </div>
            </Link<Route>>
        } else {
            <div class="publication-thumbnail">
                if let Some(trueque) = state_trueque.deref() {
                    <h1>{"Hay trueque sin ruta"}</h1>
                } else {
                    {"Cargando..."}
                }
            </div>
        }
    }
}