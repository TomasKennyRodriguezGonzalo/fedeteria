use datos_comunes::{EstadoTrueque, QueryAceptarOferta, QueryCalificarOfertante, QueryCalificarReceptor, QueryCambiarTruequeADefinido, QueryGetOffice, QueryGetUserInfo, QueryObtenerTrueque, QueryRechazarOferta, QueryTruequesFiltrados, ResponseAceptarOferta, ResponseCalificarOfertante, ResponseCalificarReceptor, ResponseCambiarTruequeADefinido, ResponseGetOffice, ResponseGetOffices, ResponseGetUserInfo, ResponseObtenerTrueque, ResponseRechazarOferta, Sucursal, Trueque};
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, FormData, HtmlFormElement, HtmlInputElement};
use yew_router::hooks::use_navigator;
use yewdux::use_store;
use crate::information_store::InformationStore;
use crate::router::Route;
use crate::request_post;
use crate::components::publication_thumbnail::PublicationThumbnail;
use crate::store::UserStore;
use crate::components::generic_button::GenericButton;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use reqwasm::http::Request;
use wasm_bindgen::JsCast;
use chrono::{DateTime, Datelike, Local, NaiveDate, TimeZone, Weekday};
use crate::molecules::confirm_prompt_button_molecule::ConfirmPromptButtonMolecule;
use crate::components::dni_input_field::DniInputField;

#[derive(Properties,PartialEq)]
pub struct Props {
    pub id: usize,
}

#[function_component(TruequeMolecule)]
pub fn trueque_molecule (props : &Props) -> Html {

    let (user_store, user_dispatch) = use_store::<UserStore>();
    let dni = user_store.dni.unwrap();

    let (_information_store, information_dispatch) = use_store::<InformationStore>();

    let show_accept_offer_state = use_state(|| false);
    let show_decline_offer_state = use_state(|| false);
    let show_call_off_offer_state = use_state(|| false);

    let loading_state = use_state(|| false);

    let loaded: UseStateHandle<bool> = use_state(|| false);
    let cloned_loaded = loaded.clone();

    let id_trueque = props.id;
    let trueque_state: UseStateHandle<Option<Trueque>> = use_state(||None);
    let cloned_trueque_state = trueque_state.clone();

    let sucursal_state = use_state(|| "".to_string());
    let cloned_sucursal_state = sucursal_state.clone();

    let receptor_username: UseStateHandle<String> = use_state(|| "".to_string());
    let cloned_receptor_username = receptor_username.clone();
    
    let ofertante_username: UseStateHandle<String> = use_state(|| "".to_string());
    let cloned_ofertante_username = ofertante_username.clone();
    let state_office_list:UseStateHandle<Vec<Sucursal>> = use_state(|| Vec::new());
    let cloned_state_office_list = state_office_list.clone();
    //hago un estado para la fecha para la verificacion de cancelacion antes de 1 dia del trueque
    let trade_date_state = use_state(|| None);
    let cloned_trade_date_state = trade_date_state.clone();

    use_effect_once(move ||{
        let cloned_state_office_list = cloned_state_office_list.clone();
        let trueque_state = cloned_trueque_state.clone();
        let cloned_trade_date_state = cloned_trade_date_state.clone();
        let cloned_sucursal_state = cloned_sucursal_state.clone();
        let query = QueryObtenerTrueque{
            id : id_trueque,  
        };
        
        request_post("/api/obtener_trueque", query, move |respuesta:ResponseObtenerTrueque|{
            let cloned_state_office_list = cloned_state_office_list.clone();
            let cloned_trueque_state = trueque_state.clone();
            let cloned_trade_date_state = cloned_trade_date_state.clone();
            let cloned_sucursal_state = cloned_sucursal_state.clone();
            match respuesta {
                Ok(trueque) =>{
                    cloned_trueque_state.set(Some(trueque.clone()));

                    let ofertante_username = cloned_ofertante_username.clone();
                    let receptor_username = cloned_receptor_username.clone();
                    cloned_trade_date_state.set(trueque.fecha_pactada);

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
        
                    request_post("/api/get_user_info", query, move |respuesta: ResponseGetUserInfo|{
                        ofertante_username.set(respuesta.nombre_y_ap)
                    });
        
                    let query = QueryGetUserInfo{
                        dni: trueque.receptor.0  
                    };
                    
                    request_post("/api/get_user_info", query, move |respuesta: ResponseGetUserInfo|{
                        receptor_username.set(respuesta.nombre_y_ap)
                    });

                    if let Some(sucursal_index) = trueque.sucursal {
                        let query = QueryGetOffice {index: sucursal_index};
                        request_post("/api/obtener_string_sucursal", query, move |respuesta: ResponseGetOffice| {
                            cloned_sucursal_state.set(respuesta.sucursal);
                        });
                    }

        
                }
                Err(error) =>{
                    log::error!("Error al obtener el trueque {:?}", error);
                }
            }
        });


        cloned_loaded.set(true);

        ||{}
    });

    let cloned_trade_date_state = trade_date_state.clone();

    let cloned_trade_date_state = trade_date_state.clone();

    let cloned_information_dispatch = information_dispatch.clone();
    let cloned_receptor_username = receptor_username.clone();
    let cloned_trueque_state = trueque_state.clone();
    let accept_offer = Callback::from(move |_event: MouseEvent| {
        let cloned_information_dispatch = cloned_information_dispatch.clone();
        // Lógica de aceptar oferta
        let trueque_state = cloned_trueque_state.clone();
        let receptor_username = cloned_receptor_username.clone();
        let query = QueryAceptarOferta{
            id : id_trueque,
        };
        request_post("/api/aceptar_oferta", query, move |respuesta: ResponseAceptarOferta|{
            if respuesta.aceptada {
                if let Some(window) = window() {
                    window.location().reload().unwrap();
                }
                cloned_information_dispatch.reduce_mut(|store| store.messages.push(format!("Aceptaste la oferta con exito")));
            } else {
                cloned_information_dispatch.reduce_mut(|store| store.messages.push(format!("Error al aceptar la oferta")));
            }
        });
    });

    //traigo el navigator para volver para atras    
    let navigator = use_navigator().unwrap();
    let navigator_cloned = navigator.clone();

    //traigo y clone el dispatch para el feedback

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

        //hardcodeo una diferencia, para cancelar en el caso de que sea una oferta
        let mut diferencia_dias = 2;

        //si hay una fecha, quiere decir que se definio el trueque, entonces, obtengo la diferencia verdadera
        if let Some (fecha_trueque) = &*cloned_trade_date_state {
            let fecha_actual = Local::now();
            diferencia_dias = fecha_trueque.signed_duration_since(fecha_actual).num_days();
            log::info!("CALCULE LA DIFERENCIA DE DIAS, ES: {:?}", diferencia_dias);
        }
        
        //si la diferencia de dias, es mayor a 1, rechazo el trueque, (u oferta, dependiendo del estado)
        if diferencia_dias >= 1 {
            request_post("/api/rechazar_oferta", query, move |_respuesta:ResponseRechazarOferta|{
                let trueque_state = trueque_state.clone();
                let receptor_username = receptor_username.clone();
                if let Some(window) = window() {
                    window.location().reload().unwrap();
                }
            });
    
            //obtengo DNI de receptor e ID de su publicacion, y armo la query
            let dni_receptor = (cloned_trueque_state.as_ref()).unwrap().receptor.0;
            let id_publication = (cloned_trueque_state.as_ref()).unwrap().receptor.1;
            let query = QueryTruequesFiltrados {
                filtro_codigo_ofertante: None,
                filtro_codigo_receptor: None,//Some(dni_receptor.clone()),
                //filtro_ofertante: None,
                //filtro_receptor: cloned_dni,
                filtro_dni_integrantes: Some(dni_receptor.clone()),
                filtro_estado: Some(EstadoTrueque::Oferta),
                filtro_fecha_pactada: None,
                filtro_fecha_trueque: None,
                filtro_id_publicacion: Some(id_publication.clone()),
                filtro_sucursal: None,
            };
            
            log::info!("ESTADO: {:?}", (cloned_trueque_state.as_ref()).unwrap().estado);

            //hago el mensaje
            if (cloned_trueque_state.as_ref()).unwrap().estado == EstadoTrueque::Definido || (cloned_trueque_state.as_ref()).unwrap().estado == EstadoTrueque::Pendiente {
                information_dispatch.reduce_mut(|store| store.messages.push(format!("Cancelaste el trueque con exito")));
            }
            else {
                information_dispatch.reduce_mut(|store| store.messages.push(format!("Rechazaste la oferta con exito")));
            }
            //vuelvo para atras
            let _ = navigator_cloned.push_with_query(&Route::SearchTrueques, &query); 
        }
        else {
            if (cloned_trueque_state.as_ref()).unwrap().estado == EstadoTrueque::Definido {
                information_dispatch.reduce_mut(|store| store.messages.push(format!("No es posible cancelar un trueque definido antes de un día de su concretacion")));
            }
        }
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

    let cloned_loading_state = loading_state.clone();
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
        cloned_loading_state.set(true);
        let cloned_show_time_error_state = cloned_show_time_error_state.clone();
        let cloned_show_another_trade_error_state = cloned_show_another_trade_error_state.clone();
        let cloned_minutos_state = cloned_select_minutes_value_state.clone();
        let sucursal_trueque = (&*cloned_sucursal_state).clone() as usize;
        let cloned_state_office_list = (&*cloned_state_office_list).clone();
        //inicializo una hora y minutos
        let mut hora_trueque = "".to_string();
        let mut minutos_trueque = "".to_string();
        let fecha_trueque = (&*cloned_fecha_state).clone();
        //obtengo la hora
        if let Some(hora) = obtener_horas_sucursal().get((&*cloned_select_hora_value_state).clone() as usize) {
            hora_trueque = hora.clone();
        }
        //obtengo los minutos
        if let Some(minutos) = obtener_minutos_posibles().get((&*cloned_minutos_state).clone() as usize) {
            minutos_trueque = minutos.clone();
        }
        //obtengo la fecha en DateTime
        let fecha = NaiveDate::parse_from_str(&fecha_trueque, "%Y-%m-%d").unwrap();
        let fecha_trueque = Local.from_local_datetime(&fecha.into()).unwrap();

        //verifico que la fecha sea posterior a la actual y no sea un domingo
        let fecha_actual: DateTime<Local> = Local::now();
        if (fecha_trueque > fecha_actual) && (fecha_trueque.weekday() != Weekday::Sun) {
            let query = QueryCambiarTruequeADefinido{
                id : id_trueque,
                sucursal : cloned_state_office_list[sucursal_trueque].id,
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
        cloned_loading_state.set(false);
    }
    );

    let cloned_show_accept_offer = show_accept_offer_state.clone();
    let show_accept_offer = Callback::from(move |_| {
        cloned_show_accept_offer.set(true);
    });

    let cloned_show_accept_offer = show_accept_offer_state.clone();
    let hide_accept_offer = Callback::from(move |_| {
        cloned_show_accept_offer.set(false);
    });
    
    let cloned_show_decline_offer_state = show_decline_offer_state.clone();
    let show_decline_offer = Callback::from(move |_|{
        let cloned_show_decline_offer_state = cloned_show_decline_offer_state.clone();
        cloned_show_decline_offer_state.set(true);
    });

    let cloned_show_decline_offer_state = show_decline_offer_state.clone();
    let hide_decline_offer = Callback::from(move |_|{
        let cloned_show_decline_offer_state = cloned_show_decline_offer_state.clone();
        cloned_show_decline_offer_state.set(false);
    });

    let cloned_show_call_off_offer_state = show_call_off_offer_state.clone();
    let show_call_off_offer = Callback::from(move |_|{
        let cloned_show_call_off_offer_state = cloned_show_call_off_offer_state.clone();
        cloned_show_call_off_offer_state.set(true);
    });

    let cloned_show_call_off_offer_state = show_call_off_offer_state.clone();
    let hide_call_off_offer = Callback::from(move |_|{
        let cloned_show_call_off_offer_state = cloned_show_call_off_offer_state.clone();
        cloned_show_call_off_offer_state.set(false);
    });


    
    // Llamado cuando el ofertante cancela la oferta, se utiliza la misma lógica que el rechazar oferta salvo en el frontend
    //traigo el navigator para volver para atras    
    let navigator = use_navigator().unwrap();
    let navigator_cloned = navigator.clone();
    //traigo y clone el dispatch para el feedback
    let (_information_store, information_dispatch) = use_store::<InformationStore>();
    let information_dispatch = information_dispatch.clone();
    let cloned_receptor_username = receptor_username.clone();
    let cloned_trueque_state = trueque_state.clone();
    let call_off_offer = Callback::from(move |_event: MouseEvent| {
        // Lógica de cancelar oferta
        let receptor_username = cloned_receptor_username.clone();
        let trueque_state = cloned_trueque_state.clone();
        //hago otro clone para armar la query mas abajo
        let cloned_trueque_state = cloned_trueque_state.clone();
        let query = QueryRechazarOferta{
            id : id_trueque,
        };
        request_post("/api/cancelar_oferta", query, move |_respuesta:ResponseRechazarOferta|{
            let trueque_state = trueque_state.clone();
            let receptor_username = receptor_username.clone();
            if let Some(window) = window() {
                window.location().reload().unwrap();
            }
        });

        //obtengo DNI de receptor e ID de su publicacion, y armo la query
        let dni_receptor = (cloned_trueque_state.as_ref()).unwrap().receptor.0;
        let id_publication = (cloned_trueque_state.as_ref()).unwrap().receptor.1;
        let query = QueryTruequesFiltrados {
            filtro_codigo_ofertante: None,
            filtro_codigo_receptor: None,//Some(dni_receptor.clone()),
            //filtro_ofertante: None,
            //filtro_receptor: cloned_dni,
            filtro_dni_integrantes: Some(dni_receptor.clone()),
            filtro_estado: Some(EstadoTrueque::Oferta),
            filtro_fecha_pactada: None,
            filtro_fecha_trueque: None,
            filtro_id_publicacion: Some(id_publication.clone()),
            filtro_sucursal: None,
        };

        //hago el mensaje
        information_dispatch.reduce_mut(|store| store.messages.push(format!("Cancelaste la oferta con exito")));
        //vuelvo para atras
        let _ = navigator_cloned.push_with_query(&Route::SearchTrueques, &query);

    });

    fn parse_u64(s: String) -> Result<u64, std::num::ParseIntError> {
        s.parse::<u64>()
    }
    let error_state = use_state(||"".to_string());
    let cloned_error_state = error_state.clone();
    
    
    let calificacion_ofertante:UseStateHandle<Option<u64>> = use_state(||None);
    let calificacion_ofertante_clone = calificacion_ofertante.clone();

    let calificacion_ofertante_changed = Callback::from(move|calificacion:String|{
        match parse_u64(calificacion) {
            Ok(n) => {
                calificacion_ofertante_clone.set(Some(n));
            }
            Err(_e) => {cloned_error_state.set("el numero ingresado debe estar entre 0 y 10".to_string())},
        }
    });

    let calificacion_receptor:UseStateHandle<Option<u64>> = use_state(||None);
    let calificacion_receptor_clone = calificacion_receptor.clone();
    let cloned_error_state = error_state.clone();
    let calificacion_receptor_changed = Callback::from(move|calificacion:String|{
        match parse_u64(calificacion) {
            Ok(n) => {
                calificacion_receptor_clone.set(Some(n));
            }
            Err(_e) => {cloned_error_state.set("el numero ingresado debe estar entre 0 y 10".to_string())},
        }
    });

    let calification_button = use_state(||false);
    let calification_button_clone = calification_button.clone();

    let show_calification_button = Callback::from(move|_|{
        calification_button_clone.set(true);
    });


    let calification_button_clone = calification_button.clone();
    let hide_calification_button = Callback::from(move|_|{
        calification_button_clone.set(false);
    });


    //EL OFERTANTE CALIFICA AL RECEPTOR
    let calification_button_clone = calification_button.clone();
    let cloned_error_state = error_state.clone();
    let calificacion_receptor_clone = calificacion_receptor.clone();
    let calificate_receptor = Callback::from(move|_|{
        let calificacion_receptor_clone = calificacion_receptor_clone.clone();
        let calification_button_clone = calification_button_clone.clone();
        let cloned_error_state = cloned_error_state.clone();
        let query = QueryCalificarReceptor{
            dni : dni,
            calificacion : (&*calificacion_receptor_clone).clone(),
            id_trueque : id_trueque,
        };
        request_post("/api/calificar_receptor", query, move|r:ResponseCalificarReceptor|{
            if r.ok && (&*calificacion_receptor_clone).clone().is_some(){
                if let Some(window) = window() {
                    window.location().reload().unwrap();
                }
            } else {
                
                cloned_error_state.set("el numero ingresado debe estar entre 0 y 10".to_string())
            }
            calification_button_clone.set(false);
        });
    });

    let calification_button_clone = calification_button.clone();
    let cloned_error_state = error_state.clone();
    let calificacion_ofertante_clone = calificacion_ofertante.clone();
    let calificate_ofertante = Callback::from(move|_|{
        let cloned_error_state = cloned_error_state.clone();
        let calification_button_clone = calification_button_clone.clone();
        let calificacion_ofertante_clone = calificacion_ofertante_clone.clone();
        let query = QueryCalificarOfertante{
            dni : dni,
            calificacion : (&*calificacion_ofertante_clone).clone(),
            id_trueque : id_trueque,
        };
        request_post("/api/calificar_ofertante", query, move|r:ResponseCalificarOfertante|{
            if r.ok && (&*calificacion_ofertante_clone).clone().is_some(){
                if let Some(window) = window() {
                    window.location().reload().unwrap();
                }
            } else { 
                cloned_error_state.set("el numero ingresado debe estar entre 0 y 10".to_string())
            }
            calification_button_clone.set(false);


        });
    });

    let sucursal = (&*sucursal_state).clone();

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
                                datos_comunes::EstadoTrueque::Rechazado => html! {  
                                    <h1 class="title">{"Trueque Rechazado"}</h1>
                                },
                                datos_comunes::EstadoTrueque::Cancelado => html! {  
                                    <h1 class="title">{"Trueque Cancelado"}</h1>
                                },
                            }
                        }
                        <div class="publications-container">
                            <div class="publications">
                            <h1>{format!("{}", (&*ofertante_username))}</h1>
                            <h2>{"Ofertante"}</h2>
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
                                <h2>{"Receptor"}</h2>
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
                                        if trueque.valido {
                                            <div class="accept-offer">
                                                <button class="accept" onclick={show_accept_offer}>{"Aceptar Oferta"}</button>
                                                <button class="decline" onclick={show_decline_offer}>{"Rechazar Oferta"}</button>
                                            </div>
                                        } else {
                                            <p> {"No se puede aceptar la oferta ya que uno o más de los productos están involucrados en otro trueque."} </p>
                                        }
                                    }
                                    else {
                                        if dni == trueque.oferta.0 {
                                            <button class="decline" onclick={show_call_off_offer}>{"Cancelar Oferta"}</button>
                                        }
                                    }
                                },
                                datos_comunes::EstadoTrueque::Pendiente => html! {
                                    <>
                                        if dni == trueque.receptor.0 {
                                            if *show_time_error_state{
                                                <h2 class="error-text">{"Debes seleccionar una fecha valida"}</h2>
                                            }
                                            if *show_another_trade_error_state{
                                                <h2 class="error-text">{"Selecciona otra fecha y/o horario, la seleccionada esta ocupada"}</h2>
                                            }
                                            <h1 class="title">{"Trueque Pendiente"}</h1>
                                            <div class="trueque-pendiente">
                                                <h2>{"Seleccione una sucursal para concretar el trueque (los domingos ninguna sucursal se encontrará abierta)"}</h2>
                                                <div class="input">
                                                    <select value="select-sucursal" id="sucursales" onchange={select_sucursal_onchange.clone()}>
                                                        <option value="-1" selected=true>{"---"}</option>
                                                        {
                                                            (&*state_office_list).iter().enumerate().map(|(index, sucursal)| html!{
                                                                <option value={index.to_string()}>{sucursal.nombre.clone()}</option>
                                                            }).collect::<Html>()
                                                        }
                                                    </select>
                                                    <h2>{"Ingrese una fecha"}</h2>
                                                    <input type="date" name="fecha-trueque" onchange={date_changed}/>
                                                    <h2>{"Horario: "}</h2>
                                                    <div class="time-selector">
                                                        <select value="select-hora" id="horas" onchange={select_hora_onchange.clone()}>
                                                            <option value="-1" selected=true>{"---"}</option>
                                                            {
                                                                obtener_horas_sucursal().iter().enumerate().map(|(index, hora)| html!{
                                                                    <option value={index.to_string()}>{hora}</option>
                                                                }).collect::<Html>()
                                                            }
                                                        </select>
                                                        <h2>{":"}</h2>
                                                        <select value="select-minutos" id="minutos" onchange={select_minutos_onchange.clone()}>
                                                            <option value="-1" selected=true>{"---"}</option>
                                                            {
                                                                obtener_minutos_posibles().iter().enumerate().map(|(index, minutos)| html!{
                                                                    <option value={index.to_string()}>{minutos}</option>
                                                                }).collect::<Html>()
                                                            }
                                                        </select>
                                                        <button class="decline" onclick={decline_offer.clone()}>{"Cancelar Trueque"}</button>
                                                    </div>
                                                    if ((&*select_sucursal_value_state).clone() != -1) && ((&*select_hora_value_state).clone() != -1) && ((&*select_minutos_value_state).clone() != -1) && (!(&*fecha_state).clone().is_empty()) { 
                                                        <GenericButton text="Confirmar Datos Ingresados" onclick_event={change_trade_to_defined}/>
                                                    } else {
                                                        <button class="disabled-dyn-element">{"Confirmar Datos Ingresados"}</button> 
                                                    }
                                                </div>
                                            </div>
                                        }
                                        else {
                                            <h2>{format!("Contactate con el usuario {} para acordar una sucursal y un horario", &*receptor_username)}</h2>
                                        }
                                    </>
                                },
                                datos_comunes::EstadoTrueque::Definido => html! {
                                    <>
                                        <h1 class="title"> {
                                            msg_trueque_definido(trueque, dni, sucursal)
                                        } </h1>
                                        <button class="decline" onclick={decline_offer.clone()}>{"Cancelar Trueque"}</button>
                                    </>
                                },
                                datos_comunes::EstadoTrueque::Finalizado => html! {
                                    <>
                                        <h1 class="title">{"Trueque Finalizado"}</h1>
                                        if trueque.oferta.0 == dni && trueque.calificacion_receptor.is_none(){
                                            <h2>{"califique del 1 al 10 a la persona con la que realizó el trueque"}</h2>
                                            <DniInputField dni = "Calificacion" label="Calificacion" tipo = "number" handle_on_change = {calificacion_ofertante_changed} />
                                            if (*calificacion_ofertante).is_some(){
                                                <GenericButton text = "Calificar Usuario" onclick_event = {(show_calification_button).clone()} />
                                            } else {
                                                <button class="disabled-dyn-element">{"Calificar Usuario"}</button>
                                            }
                                        if *calification_button{
                                            <ConfirmPromptButtonMolecule text = "Confirmar calificacion" confirm_func = {calificate_ofertante} reject_func = {(hide_calification_button).clone()}  />
                                        }
                                        } else if trueque.oferta.0 == dni{
                                            <h2>{format!("puntuaste a {} con {} puntos!", (&*receptor_username), trueque.calificacion_receptor.unwrap())}</h2>
                                        }
                                        if trueque.receptor.0 == dni && trueque.calificacion_ofertante.is_none(){
                                            <h2>{"califique del 1 al 10 a la persona con la que realizó el trueque"}</h2>
                                            <DniInputField dni = "Calificacion" label="Calificacion" tipo = "number" handle_on_change = {calificacion_receptor_changed} />
                                            if (*calificacion_receptor).is_some(){
                                                <GenericButton text = "Calificar Usuario" onclick_event = {show_calification_button} />
                                            }else{
                                                <button class="disabled-dyn-element">{"Calificar Usuario"}</button>
                                            }
                                            if *calification_button{
                                                <ConfirmPromptButtonMolecule text = "Confirmar calificacion" confirm_func = {calificate_receptor} reject_func = {hide_calification_button}  />
                                            }
                                        } else if trueque.receptor.0 == dni {
                                            <h2>{format!("puntuaste a {} con {} puntos!", (&*ofertante_username), trueque.calificacion_ofertante.unwrap())}</h2>
                                        }
                                    </>
                            },
                                datos_comunes::EstadoTrueque::Rechazado => html! {
                                    <>
                                        <h1 class="title">{"Trueque Rechazado"}</h1>
                                    </>
                                },
                                datos_comunes::EstadoTrueque::Cancelado => html! {
                                    <>
                                        <h1 class="title">{"Trueque Cancelado"}</h1>
                                    </>
                                },
                            }
                        }
                }
                if *show_decline_offer_state{
                    <ConfirmPromptButtonMolecule text="¿Seguro que quiere rechazar la oferta?" confirm_func={(decline_offer).clone()} reject_func={hide_decline_offer} />
                }
                if *show_accept_offer_state{
                    <ConfirmPromptButtonMolecule text="¿Seguro que quiere aceptar la oferta?" confirm_func={accept_offer} reject_func={hide_accept_offer} />
                }
                if *show_call_off_offer_state{
                    <ConfirmPromptButtonMolecule text="¿Seguro que quiere cancelar la oferta?" confirm_func={call_off_offer} reject_func={hide_call_off_offer} />
                }
                if *loading_state {
                    <div class="loading">
                    </div>
                }
                if !(&*error_state).clone().is_empty(){
                    <h1 class="error-text">{&*error_state}</h1>

                }
        } else {
            <div class="loading"></div>
        } 
        </div>
    }
}

fn msg_trueque_definido(trueque: &Trueque, dni: u64, sucursal: String) -> String {
    let mut msg = format!("Trueque Definido para el día {} a las {}:{} en la sucursal '{}'.",
        trueque.fecha_pactada.unwrap().format("%Y-%m-%d"),
        trueque.hora.as_ref().unwrap(),
        trueque.minutos.as_ref().unwrap(),
        sucursal
    );
    if dni == trueque.receptor.0 {
        msg += &format!(" Su código de receptor es {}", trueque.codigo_receptor.unwrap());
    }
    if dni == trueque.oferta.0 {
        msg += &format!(" Su código de ofertante es {}", trueque.codigo_ofertante.unwrap());
    }
    msg
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