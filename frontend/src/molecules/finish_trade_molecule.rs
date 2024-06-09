use datos_comunes::{EstadoTrueque, QueryFinishTrade, QueryGetOffice, QueryObtenerTrueque, QueryTruequesFiltrados, ResponseFinishTrade, ResponseGetOffice, ResponseObtenerTrueque, ResponseTruequePorCodigos};
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yewdux::use_store;

use crate::{components::{dni_input_field::DniInputField, generic_button::GenericButton}, molecules::trueque_molecule::TruequeMolecule, request_post, store::UserStore};

#[function_component(FinishTradeMolecule)]
pub fn finish_trade_molecule () -> Html {
    let (user_store, _user_dispatch) = use_store::<UserStore>();
    let dni = user_store.dni.unwrap();
    let cloned_dni = dni.clone();

    //me guardo la sucursal si encontro alguna (es decir, es un empleado)
    let sucursal_state = use_state(|| None);
    let cloned_sucursal_state = sucursal_state.clone();

    use_effect_once(move || {
        let query = QueryGetOffice {dni: cloned_dni};
        request_post("/api/obtener_sucursal_por_dni", query, move |respuesta:ResponseGetOffice|{
            cloned_sucursal_state.set(respuesta.sucursal.clone());
            log::info!("RESPUESTA OBTENER SUCURSAL POR DNI: {:?}", respuesta.sucursal);
        });
        || {}
    });
    let cloned_sucursal_state = sucursal_state.clone();

    //estado del codigo del receptor
    let receptor_code_state = use_state(|| 0);
    let cloned_receptor_code_state = receptor_code_state.clone();
    let receptor_code_onchange = Callback::from(move |code: String| {
        cloned_receptor_code_state.set(code.parse::<u64>().unwrap());
    });
    let cloned_receptor_code_state = receptor_code_state.clone();

    //estado del codigo del ofertante
    let offer_code_state = use_state(|| 0);
    let cloned_offer_code_state = offer_code_state.clone();
    let offer_code_onchange = Callback::from(move |code: String| {
        cloned_offer_code_state.set(code.parse::<u64>().unwrap());
    });
    let cloned_offer_code_state = offer_code_state.clone();

    //estado de muestreo de trueque
    let show_trade_search_state = use_state(|| false);
    let cloned_show_trade_search_state = show_trade_search_state.clone();
    //obtengo el trueque (si hay coincidencia)
    let trade_index_state = use_state(|| None);
    let cloned_trade_index_state = trade_index_state.clone();
    let search_trade = Callback::from(move |()| {

        let cloned_offer_code_state = cloned_offer_code_state.clone();
        let cloned_receptor_code_state = cloned_receptor_code_state.clone();
        let cloned_trade_index_state = cloned_trade_index_state.clone();
        cloned_show_trade_search_state.set(true);

        let query;
        log::info!("SUCURSAL: {:?}", &*cloned_sucursal_state);
        //armo la query para empleado
        if let Some(sucursal) = &*cloned_sucursal_state {
            query = QueryTruequesFiltrados {
                filtro_codigo_ofertante: Some((&*cloned_offer_code_state).clone()),
                filtro_codigo_receptor: Some((&*cloned_receptor_code_state).clone()),
                filtro_dni_integrantes: None,
                filtro_estado: None,
                filtro_fecha: None,
                filtro_id_publicacion: None,
                filtro_sucursal: Some(sucursal.clone()),
            };
        }
        //armo la query para dueño
        else {
            query = QueryTruequesFiltrados {
                filtro_codigo_ofertante: Some((&*cloned_offer_code_state).clone()),
                filtro_codigo_receptor: Some((&*cloned_receptor_code_state).clone()),
                filtro_dni_integrantes: None,
                filtro_estado: None,
                filtro_fecha: None,
                filtro_id_publicacion: None,
                filtro_sucursal: None,
            };
        }

        //obtengo el id del trueque (si existe)
        request_post("/api/obtener_trueque_por_codigos", query, move |respuesta: ResponseTruequePorCodigos| {
            log::info!("Trueque encontrado: {:?}", respuesta.trueque_encontrado);
            if let Some (mut trueque) = respuesta.trueque_encontrado {
                cloned_trade_index_state.set(Some(trueque.remove(0)));
            }
            else {
                cloned_trade_index_state.set(None);
            }
        });
        || {};
    });

    //concreto el trueque (hay que ver como agregar la logica de las compras)
    let cloned_trade_index_state = trade_index_state.clone();
    let cloned_show_trade_search_state = show_trade_search_state.clone();
    let finish_trade = Callback::from(move |()| {
        let query = QueryFinishTrade {estado: EstadoTrueque::Finalizado, id_trueque: (&*cloned_trade_index_state).unwrap().clone()};
        request_post("/api/finalizar_trueque", query, move |_respuesta: ResponseFinishTrade| {
        });
        || {};
        cloned_show_trade_search_state.set(false);
    });

    //rechazo el trueque
    let cloned_trade_index_state = trade_index_state.clone();
    let cloned_show_trade_search_state = show_trade_search_state.clone();
    let abort_trade = Callback::from(move |()| {
        let query = QueryFinishTrade {estado: EstadoTrueque::Rechazado, id_trueque: (&*cloned_trade_index_state).unwrap().clone()};
        request_post("/api/finalizar_trueque", query, move |_respuesta: ResponseFinishTrade| {
        });
        || {};
        cloned_show_trade_search_state.set(false);
    });

    //cancelo operacion
    let cloned_show_trade_search_state = show_trade_search_state.clone();
    let cancel_operation = Callback::from(move |()| {
        cloned_show_trade_search_state.set(false);
    });

    let cloned_trade_index_state = trade_index_state.clone();
    let cloned_show_trade_search_state = show_trade_search_state.clone();

    html! {
        <div class="finish-trade">
            <div class="codes-input">  
                <h2>{"Ingrese Codigo de Trueque del Usuario Receptor de la oferta"}</h2>
                <br/>
                <DniInputField dni = "Codigo Receptor" tipo = "number" handle_on_change = {receptor_code_onchange} />
                <br/>
                <h2>{"Ingrese Codigo de Trueque del Usuario Ofertante de la oferta"}</h2>
                <br/>
                <DniInputField dni = "Codigo Ofertante" tipo = "number" handle_on_change = {offer_code_onchange} />
                <br/>
                <GenericButton text = "Buscar Trueque" onclick_event = {search_trade} />
            </div>
            <div class="show-trade">
                if *cloned_show_trade_search_state {
                    if let Some(id) = &*cloned_trade_index_state {
                        <TruequeMolecule id={id.clone()}/>
                            <li><GenericButton text = "Confirmar Trueque" onclick_event = {finish_trade}/></li>
                            <li><GenericButton text = "Rechazar Trueque" onclick_event = {abort_trade}/></li>
                            <li><GenericButton text = "Cancelar Operacion" onclick_event = {cancel_operation.clone()}/></li>
                    }
                    else {
                        <h2>{"Los codigos ingresados no corresponden a un trueque"}</h2>
                        <GenericButton text = "Cancelar Operacion" onclick_event = {cancel_operation} />
                    }
                }
            </div>
        </div>
    }
}