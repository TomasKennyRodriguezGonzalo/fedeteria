use std::ops::Deref;

use datos_comunes::*;
use yew_hooks::use_effect_once;
use yew_router::prelude::Link;
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::Request;

use crate::{request_post, router::Route};

#[derive(Properties,PartialEq)]
pub struct TruequeThumbnailProps {
    pub id_trueque: usize,
    #[prop_or_default]
    pub linkless : bool,
}

#[function_component(TruequeThumbnail)]
pub fn trueque_thumbnail (props: &TruequeThumbnailProps) -> Html {
    let id = props.id_trueque.clone();
    let trueque_state: UseStateHandle<Option<Trueque>> = use_state(|| None);
    let cloned_trueque_state = trueque_state.clone();

    //use el llamado al back primitivo, porque creo que es un llamado distinto al simplificado
    let id_trueque_cloned = id.clone();
    use_effect_once(move || {
        let trueque_state = cloned_trueque_state.clone();
        
        let query = QueryObtenerTrueque{
            id,  
        };

        request_post("/api/obtener_trueque", query, move |respuesta:ResponseObtenerTrueque|{
            let trueque_state = trueque_state.clone();
            match respuesta {
                Ok(trueque) =>{
                    trueque_state.set(Some(trueque));
                }
                Err(error) =>{
                    log::error!("Error al obtener el trueque {:?}", error);
                }
            }
        });

        || {}
    });

    

    let trueque_thumbnail = html! {
        <div class="trueque-thumbnail">
            if let Some(trueque) = (&*trueque_state).clone() {
                {
                    match trueque.estado {
                        EstadoTrueque::Oferta => { html! {
                            <h1>{"OFERTA"}</h1>
                        }},
                        EstadoTrueque::Pendiente => { html! {
                            <h1>{"PENDIENTE"}</h1>
                        }},
                        EstadoTrueque::Definido => { html! {
                            <h1>{"DEFINIDO"}</h1>
                        }},
                        EstadoTrueque::Finalizado => { html! {
                            <h1>{"FINALIZADO"}</h1>
                        }}
                    }
                }
            } else {
                {"Cargando..."}
            }
        </div>
    };

    html! {
        if !(props.linkless) {
            <Link<Route> to={Route::Trueque{id}}>
                {trueque_thumbnail}
            </Link<Route>>
        } else {
            {trueque_thumbnail}
        }
    }
}