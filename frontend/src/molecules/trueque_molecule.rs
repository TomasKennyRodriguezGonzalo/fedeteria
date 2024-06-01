use datos_comunes::{QueryGetUserInfo, QueryObtenerTrueque, ResponseGetUserInfo, ResponseObtenerTrueque, Trueque};
use yewdux::use_store;
use crate::request_post;
use crate::components::publication_thumbnail::PublicationThumbnail;
use crate::store::UserStore;
use yew::prelude::*;
use yew_hooks::use_effect_once;

#[derive(Properties,PartialEq)]
pub struct Props {
    pub id: usize,
}

#[function_component(TruequeMolecule)]
pub fn trueque_molecule (props : &Props) -> Html {

    let (user_store, user_dispatch) = use_store::<UserStore>();
    let dni = user_store.dni.unwrap();

    let loaded: UseStateHandle<bool> = use_state(|| false);
    let cloned_loaded = loaded.clone();

    let id_trueque = props.id;
    let trueque_state: UseStateHandle<Option<Trueque>> = use_state(||None);
    let cloned_trueque_state = trueque_state.clone();

    let receptor_username: UseStateHandle<String> = use_state(|| "".to_string());
    let cloned_receptor_username = receptor_username.clone();
    
    let ofertante_username: UseStateHandle<String> = use_state(|| "".to_string());
    let cloned_ofertante_username = ofertante_username.clone();

    use_effect_once(move ||{
        let trueque_state = cloned_trueque_state.clone();
        let query = QueryObtenerTrueque{
            id : id_trueque,  
        };
        
        request_post("/api/obtener_trueque", query, move |respuesta:ResponseObtenerTrueque|{
            let cloned_trueque_state = trueque_state.clone();
            match respuesta {
                Ok(trueque) =>{
                    cloned_trueque_state.set(Some(trueque.clone()));

                    let ofertante_username = cloned_ofertante_username.clone();
                    let receptor_username = cloned_receptor_username.clone();

                    let query = QueryGetUserInfo{
                        dni: trueque.oferta.0 
                    };
        
                    request_post("/api/get_user_info", query, move |respuesta:ResponseGetUserInfo|{
                        ofertante_username.set(respuesta.nombre_y_ap)
                    });
        
                    let query = QueryGetUserInfo{
                        dni: trueque.receptor.0  
                    };
                    
                    request_post("/api/get_user_info", query, move |respuesta:ResponseGetUserInfo|{
                        receptor_username.set(respuesta.nombre_y_ap)
                    });

                    log::info!("Usuarios: O {} y R {} ", (&*cloned_ofertante_username).clone(), &*cloned_receptor_username)
                }
                Err(error) =>{
                    log::error!("Error al obtener el trueque {:?}", error);
                }
            }
        });

        cloned_loaded.set(true);

        ||{}
    });

    let accept_offer = Callback::from(|event: MouseEvent| {
        // Lógica de aceptar oferta
    });
    
    let decline_offer = Callback::from(|event: MouseEvent| {
        // Lógica de rechazar oferta
    });

    html! {
        <div class="trueque-box">
            if *loaded {
                if let Some(trueque) = &*trueque_state{    
                        {
                            match trueque.estado {
                                datos_comunes::EstadoTrueque::Oferta => html!{  
                                        <h1 class="title">{"Oferta"}</h1>    
                                },
                                datos_comunes::EstadoTrueque::Pendiente => html! {   
                                        <h1 class="title">{"Trueque Pendiente"}</h1>
                                },
                                datos_comunes::EstadoTrueque::Definido => html! {  
                                        <h1 class="title">{"Trueque Definido"}</h1>
                                },
                                datos_comunes::EstadoTrueque::Finalizado => html! {  
                                        <h1 class="title">{"Trueque Finalizado"}</h1>
                                },
                            }
                        }
                        <div class="publications-container">
                            <div class="publications">
                            <h1>{format!("{}", (&*ofertante_username))}</h1>
                            <h2>{"ofrece:"}</h2>
                                <ul>
                                    <li><PublicationThumbnail id={trueque.oferta.1.get(0).unwrap()}/></li>
                                    if let Some(segunda_publicacion_oferta) = trueque.oferta.1.get(1){
                                        <li><PublicationThumbnail id={segunda_publicacion_oferta}/></li>
                                    }
                                </ul>
                            </div>
                            <div class="trade-symbol">
                                <h1>{"◄"}</h1>
                                <h1>{"►"}</h1>
                            </div>
                            <div class="publications">
                                <h1>{format!("{}", (&*receptor_username))}</h1>
                                <h2>{"ofrece:"}</h2>
                                <ul>
                                    <li><PublicationThumbnail id={trueque.receptor.1}/></li>
                                </ul>
                            </div>
                        </div>
                        {
                            match trueque.estado {
                                datos_comunes::EstadoTrueque::Oferta => html!{
                                    if dni == trueque.receptor.0 {
                                        <div class="accept-offer">
                                            <button class="accept" onclick={accept_offer}>{"Aceptar Oferta"}</button>
                                            <button class="decline" onclick={decline_offer}>{"Rechazar Oferta"}</button>
                                        </div>
                                    }
                                },
                                datos_comunes::EstadoTrueque::Pendiente => html! {
                                    <>
                                        <h1 class="title">{"Trueque Pendiente"}</h1>
                                    </>
                                },
                                datos_comunes::EstadoTrueque::Definido => html! {
                                    <>
                                        <h1 class="title">{"Trueque Definido"}</h1>
                                    </>
                                },
                                datos_comunes::EstadoTrueque::Finalizado => html! {
                                    <>
                                        <h1 class="title">{"Trueque Finalizado"}</h1>
                                    </>
                                },
                            }
                        }
                }
        } else {
            <div class="loading"></div>
        } 
        </div>
    }
}