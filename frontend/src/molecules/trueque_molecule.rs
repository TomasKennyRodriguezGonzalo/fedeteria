use datos_comunes::{EstadoTrueque, QueryAceptarOferta, QueryCambiarTruequeADefinido, QueryGetUserInfo, QueryObtenerTrueque, QueryRechazarOferta, ResponseAceptarOferta, ResponseCambiarTruequeADefinido, ResponseGetOffices, ResponseGetUserInfo, ResponseObtenerTrueque, ResponseRechazarOferta, Sucursal, Trueque};
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, HtmlInputElement};
use yewdux::use_store;
use crate::convenient_request::send_notification;
use crate::request_post;
use crate::components::publication_thumbnail::PublicationThumbnail;
use crate::store::UserStore;
use crate::components::generic_button::GenericButton;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use reqwasm::http::Request;
use wasm_bindgen::JsCast;
use chrono::{DateTime, Local};

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
    let state_office_list:UseStateHandle<Vec<Sucursal>> = use_state(|| Vec::new());
    let cloned_state_office_list = state_office_list.clone();

    use_effect_once(move ||{
        let cloned_state_office_list = cloned_state_office_list.clone();
        let trueque_state = cloned_trueque_state.clone();
        let query = QueryObtenerTrueque{
            id : id_trueque,  
        };
        
        request_post("/api/obtener_trueque", query, move |respuesta:ResponseObtenerTrueque|{
            let cloned_state_office_list = cloned_state_office_list.clone();
            let cloned_trueque_state = trueque_state.clone();
            match respuesta {
                Ok(trueque) =>{
                    cloned_trueque_state.set(Some(trueque.clone()));

                    let ofertante_username = cloned_ofertante_username.clone();
                    let receptor_username = cloned_receptor_username.clone();

                    if trueque.estado == EstadoTrueque::Pendiente{
                        spawn_local(async move {
                            let state_office_list_clone = cloned_state_office_list.clone();
                            let state_office_list_clone = state_office_list_clone.clone();
                            log::info!("entre al spawn local");
                            let respuesta = Request::get("/api/obtener_sucursales")
                            .header("Content-Type", "application/json")
                            .send()
                            .await;
                        match respuesta{
                            Ok(respuesta) =>{
                                let response:Result<ResponseGetOffices, reqwasm::Error> = respuesta.json().await;
                                log::info!("deserailice la respuesta {:?}",response);
                                match response{
                                    Ok(respuesta) => {           
                                        state_office_list_clone.set(respuesta.office_list.clone());
                                        log::info!("las sucursales que tenes ahora son: {:?}",respuesta.office_list);
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


        
                }
                Err(error) =>{
                    log::error!("Error al obtener el trueque {:?}", error);
                }
            }
        });

        cloned_loaded.set(true);

        ||{}
    });

    let cloned_receptor_username = receptor_username.clone();
    let cloned_trueque_state = trueque_state.clone();
    let accept_offer = Callback::from(move |_event: MouseEvent| {
        // Lógica de aceptar oferta
        let trueque_state = cloned_trueque_state.clone();
        let receptor_username = cloned_receptor_username.clone();
        let query = QueryAceptarOferta{
            id : id_trueque,
        };
        request_post("/api/aceptar_oferta", query, move |_respuesta:ResponseAceptarOferta|{
            let receptor_username = receptor_username.clone();
            let trueque_state = trueque_state.clone();
            if let Some(window) = window() {
                //enviar notificacion al ofertante
                send_notification("Oferta Aceptada".to_string(), format!("{} ha aceptado tu oferta! cliquea aquí para ver los detalles!",(&*receptor_username)), window.location().href().unwrap(), (&*trueque_state).clone().unwrap().oferta.0); 
            }
            if let Some(window) = window() {
                window.location().reload().unwrap();
            }
        });
    });
    

    let cloned_receptor_username = receptor_username.clone();
    let cloned_trueque_state = trueque_state.clone();
    let decline_offer = Callback::from(move |_event: MouseEvent| {
        // Lógica de rechazar oferta
        let receptor_username = cloned_receptor_username.clone();
        let trueque_state = cloned_trueque_state.clone();
        let query = QueryRechazarOferta{
            id : id_trueque,
        };
        request_post("/api/rechazar_oferta", query, move |_respuesta:ResponseRechazarOferta|{
            let trueque_state = trueque_state.clone();
            let receptor_username = receptor_username.clone();
            if let Some(window) = window() {
                //enviar notificacion al ofertante
                send_notification("Oferta Rechazada".to_string(), format!("{} ha rechazado tu oferta :(",(&*receptor_username)), window.location().href().unwrap(), (&*trueque_state).clone().unwrap().oferta.0); 
            }
            if let Some(window) = window() {
                window.location().reload().unwrap();
            }
        });
    });

    let select_value_state = use_state(|| -1);
    let select_value_state_cloned = select_value_state.clone();
    let select_onchange = Callback::from(move|event: Event| {
        let select_value_state_cloned = select_value_state_cloned.clone();
        let target = event.target().unwrap();
        let input:HtmlInputElement = target.unchecked_into();
        let value: i32 = input.value().parse().unwrap();
        select_value_state_cloned.set(value);
        log::info!("Select changed to {}", value)
    });


    let horario_state:UseStateHandle<Option<DateTime<Local>>> = use_state(|| None);
    let cloned_horario_state = horario_state.clone();
    //agregar logica de change horario state


    let sucursal_state:UseStateHandle<Option<String>> = use_state(|| None);
    let cloned_sucursal_state = sucursal_state.clone();
    //agregar logica de change sucursal state
 
    let show_no_time_error_state = use_state(|| false);
    let cloned_show_no_time_error_state = show_no_time_error_state.clone();
    let show_no_sucursal_error_state = use_state(|| false);
    let cloned_show_no_sucursal_error_state = show_no_sucursal_error_state.clone();
    let cloned_horario_state = horario_state.clone();
    let cloned_sucursal_state = cloned_sucursal_state.clone();
    let change_trade_to_defined = Callback::from(move |_|{
        let cloned_show_no_sucursal_error_state = cloned_show_no_sucursal_error_state.clone();
        let cloned_show_no_time_error_state = cloned_show_no_time_error_state.clone();
        let cloned_sucursal_state = cloned_sucursal_state.clone();
        let cloned_horario_state = cloned_horario_state.clone();
        if let Some(sucursal) = (&*cloned_sucursal_state).clone(){
            if let Some(f_y_hora) = (&*cloned_horario_state){
                let query = QueryCambiarTruequeADefinido{
                    id : id_trueque,
                    sucursal : sucursal,
                    f_y_hora : *f_y_hora,
                };
                request_post("/api/cambiar_trueque_a_definido", query, move |_:ResponseCambiarTruequeADefinido|{
                    if let Some(window) = window() {
                        window.location().reload().unwrap();
                    }
                });
            } else {
                cloned_show_no_time_error_state.set(true);
            }
        } else {
            cloned_show_no_sucursal_error_state.set(true);
        }
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
                                <div class="arrow-left"></div>
                                <div class="arrow-right"></div>
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
                                        if *show_no_sucursal_error_state{
                                            <h2 class="error-text">{"Debes seleccionar una sucursal"}</h2>
                                        }
                                        if *show_no_time_error_state{
                                            <h2 class="error-text">{"Debes seleccionar una fecha y horario válidos"}</h2>
                                        }
                                        <h1 class="title">{"Trueque Pendiente"}</h1>
                                        <li>
                                        <div class="trueque-pendiente">
                                            <label for="select-sucursal">{"Seleccione una sucursal para concretar el trueque"}</label>
                                            <br/>
                                            <select value="select-sucursal" id="sucursales" onchange={select_onchange}>
                                                <option value="-1">{"---"}</option>
                                                {
                                                    (&*state_office_list).iter().enumerate().map(|(index, sucursal)| html!{
                                                        <option value={index.to_string()}>{sucursal.nombre.clone()}</option>
                                                    }).collect::<Html>()
                                                }
                                            </select>
                                            if (&*select_value_state).clone() != -1 { 
                                                <GenericButton text="Rellenar Datos de Trueque" onclick_event={change_trade_to_defined}/>
                                            }
                                        </div>
                                    </li>
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