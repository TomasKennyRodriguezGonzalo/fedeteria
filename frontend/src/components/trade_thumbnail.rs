use std::ops::Deref;

use datos_comunes::*;
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew_hooks::use_effect_once;
use yew_router::prelude::Link;
use yew::prelude::*;

use crate::{request_post, router::Route};

#[derive(Properties,PartialEq)]
pub struct TradeThumbnailProps {
    pub id_trade: usize,
    #[prop_or_default]
    pub linkless : bool,
}

#[function_component(TradeThumbnail)]
pub fn trade_thumbnail (props: &TradeThumbnailProps) -> Html {
    let id = props.id_trade.clone();
    let trade_state: UseStateHandle<Option<Trueque>> = use_state(|| None);
    let cloned_trade_state = trade_state.clone();

    let offered_publications_title_state: UseStateHandle<Vec<String>> = use_state(|| vec![]);
    let cloned_offered_publications_title_state: UseStateHandle<Vec<String>> = offered_publications_title_state.clone();
    
    let receiver_publication_title_state: UseStateHandle<String> = use_state(|| "".to_string());
    let cloned_receiver_publication_title_state: UseStateHandle<String> = receiver_publication_title_state.clone();
    
    use_effect_once(move || {
        let trade_state = cloned_trade_state.clone();
        
        let query = QueryObtenerTrueque{
            id,  
        };
        
        let cloned_offered_publications_title_state: UseStateHandle<Vec<String>> = cloned_offered_publications_title_state.clone();
        let cloned_trade_state = trade_state.clone();
        request_post("/api/obtener_trueque", query, move |respuesta:ResponseObtenerTrueque|{
            let trade_state = cloned_trade_state.clone();
            match respuesta {
                Ok(trueque) =>{
                    trade_state.set(Some(trueque.clone()));

                    for i in (&trueque).oferta.1.clone() {
                        let offered_publications_title_state: UseStateHandle<Vec<String>> = cloned_offered_publications_title_state.clone();
                        spawn_local(async move {
                            let id = i;
                            let cloned_offered_publications_title_state: UseStateHandle<Vec<String>> = offered_publications_title_state.clone();
                            let respuesta = Request::get(&format!("/api/datos_publicacion?id={id}")).send().await;
                            match respuesta{
                                Ok(respuesta) => {
                                    let respuesta: Result<ResponsePublicacion, reqwasm::Error> = respuesta.json().await;
                                    match respuesta{
                                        Ok(respuesta) => {
                                            match respuesta {
                                                Ok(publicacion) => {
                                                    log::info!("Datos de publicacion!: {publicacion:?}");
                                                    let mut new_vec = cloned_offered_publications_title_state.deref().clone();
                                                    new_vec.push(publicacion.titulo);
                                                    cloned_offered_publications_title_state.set(new_vec);
                                                },
                                                Err(error) => {
                                                    log::error!("Error de publicacion: {error:?}.");
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
                    let id = trueque.receptor.1;
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
                                                cloned_receiver_publication_title_state.set(publicacion.titulo)
                                            },
                                            Err(error) => {
                                                log::error!("Error de publicacion: {error:?}.");
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
                Err(error) =>{
                    log::error!("Error al obtener el trueque {:?}", error);
                }
            }
        });

        || {}
    });

    let trueque_thumbnail = html! {
        <>
                <div class="offer">
                    {
                        (&*offered_publications_title_state).iter().map(|title| {
                            html! {
                                <h1>{title}</h1>
                            }
                        }).collect::<Html>()
                    }
                </div>
                <div class="trade-symbol">
                    <h1>{"X"}</h1>
                </div>
                <div class="receiver">
                    <h1>{&*receiver_publication_title_state}</h1>
                </div>
        </>
    };

    html! {
        <div class="trueque-thumbnail">
            if let Some(trueque) = (&*trade_state).clone() {
                if !(props.linkless) {
                    {
                        match trueque.estado {
                            EstadoTrueque::Oferta => { html! {
                                <h1 class="subtitle">{"OFERTA"}</h1>
                            }},
                            EstadoTrueque::Pendiente => { html! {
                                <h1 class="subtitle">{"PENDIENTE"}</h1>
                            }},
                            EstadoTrueque::Definido => { html! {
                                <h1 class="subtitle">{"DEFINIDO"}</h1>
                            }},
                            EstadoTrueque::Finalizado => { html! {
                                <h1 class="subtitle">{"FINALIZADO"}</h1>
                            }}
                            EstadoTrueque::Rechazado => { html! {
                                <h1 class="subtitle">{"RECHAZADO"}</h1>
                            }}
                        }
                    }
                    <Link<Route> to={Route::Trueque{id}}>
                        {trueque_thumbnail}
                    </Link<Route>>
                } else {
                    {trueque_thumbnail}
                }
            } else {
                {"Cargando..."}
            }
        </div>
    }
}