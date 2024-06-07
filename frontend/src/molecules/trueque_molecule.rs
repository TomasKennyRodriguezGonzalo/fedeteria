use datos_comunes::{EstadoTrueque, QueryAceptarOferta, QueryCambiarTruequeADefinido, QueryGetUserInfo, QueryObtenerTrueque, QueryRechazarOferta, QueryTruequesFiltrados, ResponseAceptarOferta, ResponseCambiarTruequeADefinido, ResponseGetOffices, ResponseGetUserInfo, ResponseObtenerTrueque, ResponseRechazarOferta, Sucursal, Trueque};
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, FormData, HtmlFormElement, HtmlInputElement};
use yew_router::hooks::use_navigator;
use yewdux::use_store;
use crate::information_store::InformationStore;
use crate::{convenient_request::send_notification, router::Route};
use crate::request_post;
use crate::components::publication_thumbnail::PublicationThumbnail;
use crate::store::UserStore;
use crate::components::generic_button::GenericButton;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use reqwasm::http::Request;
use wasm_bindgen::JsCast;
use chrono::{DateTime, Datelike, Local, NaiveDate, TimeZone, Weekday};

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

    //traigo el navigator para volver para atras    
    let navigator = use_navigator().unwrap();
    let navigator_cloned = navigator.clone();

    //traigo y clone el dispatch para el feedback
    let (_information_store, information_dispatch) = use_store::<InformationStore>();
    let information_dispatch = information_dispatch.clone();

    let cloned_receptor_username = receptor_username.clone();
    let cloned_trueque_state = trueque_state.clone();
    let decline_offer = Callback::from(move |_event: MouseEvent| {
        // Lógica de rechazar oferta
        let receptor_username = cloned_receptor_username.clone();
        let trueque_state = cloned_trueque_state.clone();
        //hago otro clone para armar la query mas abajo
        let cloned_trueque_state = cloned_trueque_state.clone();
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

        //obtengo DNI de receptor e ID de su publicacion, y armo la query
        let dni_receptor = (cloned_trueque_state.as_ref()).unwrap().receptor.0;
        let id_publication = (cloned_trueque_state.as_ref()).unwrap().receptor.1;
        let query = QueryTruequesFiltrados {
            filtro_codigo_ofertante: None,
            filtro_codigo_receptor: Some(dni_receptor.clone()),
            //filtro_ofertante: None,
            //filtro_receptor: cloned_dni,
            filtro_dni_integrantes: None,
            filtro_estado: Some(EstadoTrueque::Oferta),
            filtro_fecha: None,
            filtro_id_publicacion: Some(id_publication.clone()),
            filtro_sucursal: None,
        };

        //hago el mensaje
        information_dispatch.reduce_mut(|store| store.messages.push(format!("Rechazaste la oferta con exito")));
        //vuelvo para atras
        let _ = navigator_cloned.push_with_query(&Route::SearchTrueques, &query);

    });

    //select_sucursal
    let select_sucursal_value_state = use_state(|| -1);
    let select_sucursal_value_state_cloned = select_sucursal_value_state.clone();
    let select_sucursal_onchange = Callback::from(move|event: Event| {
        let select_sucursal_value_state_cloned = select_sucursal_value_state_cloned.clone();
        let target = event.target().unwrap();
        let input:HtmlInputElement = target.unchecked_into();
        let value: i32 = input.value().parse().unwrap();
        select_sucursal_value_state_cloned.set(value);
        log::info!("Select sucursal changed to {}", value)
    });
    let cloned_select_sucursal_value_state = select_sucursal_value_state.clone();

    //select horas
    let select_hora_value_state = use_state(|| -1);
    let select_hora_value_state_cloned = select_hora_value_state.clone();
    let select_hora_onchange = Callback::from(move|event: Event| {
        let select_hora_value_state_cloned = select_hora_value_state_cloned.clone();
        let target = event.target().unwrap();
        let input:HtmlInputElement = target.unchecked_into();
        let value: i32 = input.value().parse().unwrap();
        select_hora_value_state_cloned.set(value);
        log::info!("Select hora changed to {}", value)
    });
    let cloned_select_hora_value_state = select_hora_value_state.clone();

    //select minutos
    let select_minutos_value_state = use_state(|| -1);
    let select_minutos_value_state_cloned = select_minutos_value_state.clone();
    let select_minutos_onchange = Callback::from(move|event: Event| {
        let select_minutos_value_state_cloned = select_minutos_value_state_cloned.clone();
        let target = event.target().unwrap();
        let input:HtmlInputElement = target.unchecked_into();
        let value: i32 = input.value().parse().unwrap();
        select_minutos_value_state_cloned.set(value);
        log::info!("Select minutos changed to {}", value)
    });
    let cloned_select_minutes_value_state = select_minutos_value_state.clone();

    //cambio de fecha
    let fecha_state = use_state(|| "".to_string());
    let fecha_state_cloned = fecha_state.clone();
    let date_changed = Callback::from(move |event : Event| {
        let target = event.target().unwrap();
        let input:HtmlInputElement = target.unchecked_into();
        let input_value = input.value();
        fecha_state_cloned.set(input_value);
    });

    //clono estados para botones
    let show_another_trade_error_state = use_state(|| false);
    let cloned_show_another_trade_error_state = show_another_trade_error_state.clone();
    let show_time_error_state = use_state(|| false);
    let cloned_show_time_error_state = show_time_error_state.clone();
    //clono lo referido a sucursales y fecha de trueque
    let cloned_sucursal_state = cloned_select_sucursal_value_state.clone();
    let cloned_state_office_list = state_office_list.clone();
    let cloned_fecha_state = fecha_state.clone();
    //cambiar trueque a definido
    let change_trade_to_defined = Callback::from(move |()| {
        let cloned_show_time_error_state = cloned_show_time_error_state.clone();
        let cloned_show_another_trade_error_state = cloned_show_another_trade_error_state.clone();
        let cloned_minutos_state = cloned_select_minutes_value_state.clone();
        let cloned_sucursal_state = cloned_sucursal_state.clone();
        //inicializo una hora. minutos y sucursal
        let mut hora_trueque = "".to_string();
        let mut minutos_trueque = "".to_string();
        let mut sucursal_trueque = "".to_string();
        //obtengo la fecha en String y la lista de sucursales
        let fecha_trueque = (&*cloned_fecha_state).clone();
        let office_list = &*cloned_state_office_list.clone();
        //obtengo la hora
        if let Some(hora) = obtener_horas_sucursal().get((&*cloned_select_hora_value_state).clone() as usize) {
            hora_trueque = hora.clone();
        }
        //obtengo los minutos
        if let Some(minutos) = obtener_minutos_posibles().get((&*cloned_minutos_state).clone() as usize) {
            minutos_trueque = minutos.clone();
        }
        //obtengo la sucursal
        if let Some(sucursal) = office_list.get((&*cloned_sucursal_state).clone() as usize) {
            sucursal_trueque = sucursal.clone().nombre;
        }
        //obtengo la fecha en DateTime
        let fecha = NaiveDate::parse_from_str(&fecha_trueque, "%Y-%m-%d").unwrap();
        let fecha_trueque = Local.from_local_datetime(&fecha.into()).unwrap();

        //verifico que la fecha sea posterior a la actual y no sea un domingo
        let fecha_actual: DateTime<Local> = Local::now();
        if (fecha_trueque > fecha_actual) && (fecha_trueque.weekday() != Weekday::Sun) {
            let query = QueryCambiarTruequeADefinido{
                id : id_trueque,
                sucursal : sucursal_trueque,
                fecha : fecha_trueque.clone(),
                hora: hora_trueque,
                minutos: minutos_trueque,
            };
            log::info!("Llamo al back con esto: {:?}", query);
            request_post("/api/cambiar_trueque_a_definido", query, move |respuesta:ResponseCambiarTruequeADefinido|{
                if respuesta.cambiado {
                    if let Some(window) = window() {
                        window.location().reload().unwrap();
                    }
                }
                else {
                    cloned_show_another_trade_error_state.set(true);
                }
            });
        }
        else {
            cloned_show_time_error_state.set(true);
        }
    }
    );

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
                                    else {
                                        if dni == trueque.oferta.0 {
                                            <button class="decline" onclick={decline_offer}>{"Cancelar Oferta"}</button>
                                        }
                                    }
                                },
                                datos_comunes::EstadoTrueque::Pendiente => html! {
                                    <>
                                        if dni == trueque.receptor.0 {
                                            if *show_time_error_state{
                                                <h2 class="error-text">{"Debes seleccionar una fecha y horario válidos"}</h2>
                                            }
                                            if *show_another_trade_error_state{
                                                <h2 class="error-text">{"Selecciona otra fecha y/o horario, la seleccionada esta ocupada"}</h2>
                                            }
                                            <h1 class="title">{"Trueque Pendiente"}</h1>
                                            <li>
                                            <div class="trueque-pendiente">
                                                <h2>{"Seleccione una sucursal para concretar el trueque (los domingos ninguna sucursal se encontrará abierta)"}</h2>
                                                <br/>
                                                <select value="select-sucursal" id="sucursales" onchange={select_sucursal_onchange.clone()}>
                                                    <option value="-1">{"---"}</option>
                                                    {
                                                        (&*state_office_list).iter().enumerate().map(|(index, sucursal)| html!{
                                                            <option value={index.to_string()}>{sucursal.nombre.clone()}</option>
                                                        }).collect::<Html>()
                                                    }
                                                </select>
                                                <br/>
                                                <h2>{"Ingrese una fecha"}</h2>
                                                <input type="date" name="fecha-trueque" onchange={date_changed}/>
                                                <br/>
                                                <h2>{"Seleccione hora"}</h2>
                                                <select value="select-hora" id="horas" onchange={select_hora_onchange.clone()}>
                                                    <option value="-1">{"---"}</option>
                                                    {
                                                        obtener_horas_sucursal().iter().enumerate().map(|(index, hora)| html!{
                                                            <option value={index.to_string()}>{hora}</option>
                                                        }).collect::<Html>()
                                                    }
                                                </select>
                                                <br/>
                                                <h2>{"Seleccione minutos"}</h2>
                                                <select value="select-minutos" id="minutos" onchange={select_minutos_onchange.clone()}>
                                                    <option value="-1">{"---"}</option>
                                                    {
                                                        obtener_minutos_posibles().iter().enumerate().map(|(index, minutos)| html!{
                                                            <option value={index.to_string()}>{minutos}</option>
                                                        }).collect::<Html>()
                                                    }
                                                </select>
                                                if ((&*select_sucursal_value_state).clone() != -1) && ((&*select_hora_value_state).clone() != -1) && ((&*select_minutos_value_state).clone() != -1) && (!(&*fecha_state).clone().is_empty()) { 
                                                    <GenericButton text="Rellenar Datos de Trueque" onclick_event={change_trade_to_defined}/>
                                                }
                                            </div>
                                            </li>
                                        }
                                        else {
                                            <h2>{format!("Contactate con el usuario {} para acordar una sucursal y un horario", &*receptor_username)}</h2>
                                        }
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

fn obtener_horas_sucursal() -> Vec<String> {
    let mut vec_horas = Vec::new();
    vec_horas.push("9".to_string());
    vec_horas.push("10".to_string());
    vec_horas.push("11".to_string());
    vec_horas.push("12".to_string());
    vec_horas.push("13".to_string());
    vec_horas.push("14".to_string());
    vec_horas.push("15".to_string());
    vec_horas.push("16".to_string());
    vec_horas
}

fn obtener_minutos_posibles() -> Vec<String> {
    let mut vec_minutos = Vec::new();
    vec_minutos.push("00".to_string());
    vec_minutos.push("05".to_string());
    vec_minutos.push("10".to_string());
    vec_minutos.push("15".to_string());
    vec_minutos.push("20".to_string());
    vec_minutos.push("25".to_string());
    vec_minutos.push("30".to_string());
    vec_minutos.push("35".to_string());
    vec_minutos.push("40".to_string());
    vec_minutos.push("45".to_string());
    vec_minutos.push("50".to_string());
    vec_minutos.push("55".to_string());
    vec_minutos
}