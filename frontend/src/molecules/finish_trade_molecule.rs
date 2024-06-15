use crate::molecules::confirm_prompt_button_molecule::ConfirmPromptButtonMolecule;
use datos_comunes::{EstadoTrueque, QueryFinishTrade, QueryGetOffice, QueryObtenerTrueque, QueryTruequesFiltrados, ResponseFinishTrade, ResponseGetOffice, ResponseObtenerTrueque, ResponseTruequePorCodigos};
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yewdux::use_store;

use crate::{components::{dni_input_field::DniInputField, generic_button::GenericButton}, information_store::InformationStore, molecules::trueque_molecule::TruequeMolecule, request_post, store::UserStore};

#[function_component(FinishTradeMolecule)]
pub fn finish_trade_molecule () -> Html {
    let (user_store, _user_dispatch) = use_store::<UserStore>();
    let dni = user_store.dni.unwrap();
    let cloned_dni = dni.clone();

    let (_information_store, information_dispatch) = use_store::<InformationStore>();

    let finish_confirmation_state = use_state(|| false);
    let abort_confirmation_state = use_state(|| false);
    let cancel_confirmation_state = use_state(|| false);

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
    });

    //concreto el trueque (hay que ver como agregar la logica de las compras)
    let cloned_information_dispatch = information_dispatch.clone();
    let cloned_finish_confirmation_state = finish_confirmation_state.clone();
    let cloned_trade_index_state = trade_index_state.clone();
    let cloned_show_trade_search_state = show_trade_search_state.clone();
    let finish_trade = Callback::from(move |_e| {
        let query = QueryFinishTrade {estado: EstadoTrueque::Finalizado, id_trueque: (&*cloned_trade_index_state).unwrap().clone()};
        request_post("/api/finalizar_trueque", query, move |_respuesta: ResponseFinishTrade| {
        });
        cloned_show_trade_search_state.set(false);
        cloned_finish_confirmation_state.set(false);
        cloned_information_dispatch.reduce_mut(|store| store.messages.push(format!("Trueque concretado!")));
    });

    //rechazo el trueque
    let cloned_information_dispatch = information_dispatch.clone();
    let cloned_abort_confirmation_state = abort_confirmation_state.clone();
    let cloned_trade_index_state = trade_index_state.clone();
    let cloned_show_trade_search_state = show_trade_search_state.clone();
    let abort_trade = Callback::from(move |_e| {
        let query = QueryFinishTrade {estado: EstadoTrueque::Rechazado, id_trueque: (&*cloned_trade_index_state).unwrap().clone()};
        request_post("/api/finalizar_trueque", query, move |_respuesta: ResponseFinishTrade| {
        });
        cloned_show_trade_search_state.set(false);
        cloned_abort_confirmation_state.set(false);
        cloned_information_dispatch.reduce_mut(|store| store.messages.push(format!("Trueque rechazado!")));
    });

    //cancelo operacion
    let cloned_show_trade_search_state = show_trade_search_state.clone();
    let cancel_operation = Callback::from(move |_e| {
        cloned_show_trade_search_state.set(false);
    });

    // Manejo de prompt de confirmacion de CONCRETAR de trueque
    let cloned_finish_confirmation_state = finish_confirmation_state.clone();
    let show_finish_trade_confirmation = Callback::from(move |_e| {
        cloned_finish_confirmation_state.set(true);
        });
        
    let cloned_finish_confirmation_state = finish_confirmation_state.clone();
    let hide_finish_trade_confirmation = Callback::from(move |_e| {
        cloned_finish_confirmation_state.set(false);
    });

    // Manejo de prompt de confirmacion de RECHAZO de trueque
    let cloned_abort_confirmation_state = abort_confirmation_state.clone();
    let show_abort_trade_confirmation = Callback::from(move |_e| {
        cloned_abort_confirmation_state.set(true);
        });
        
    let cloned_abort_confirmation_state = abort_confirmation_state.clone();
    let hide_abort_trade_confirmation = Callback::from(move |_e| {
        cloned_abort_confirmation_state.set(false);
    });

    
    let cloned_trade_index_state = trade_index_state.clone();
    let cloned_show_trade_search_state = show_trade_search_state.clone();
    html! {
        <div class="finish-trade">
            <div class="codes-input">  
                <h2>{"Ingrese Codigo de Trueque del Usuario Receptor de la oferta"}</h2>
                <DniInputField dni = "Codigo Receptor" tipo = "number" handle_on_change = {receptor_code_onchange} />
                <h2>{"Ingrese Codigo de Trueque del Usuario Ofertante de la oferta"}</h2>
                <DniInputField dni = "Codigo Ofertante" tipo = "number" handle_on_change = {offer_code_onchange} />
                <GenericButton text = "Buscar Trueque" onclick_event = {search_trade} />
            </div>
            if *cloned_show_trade_search_state {
                if let Some(id) = &*cloned_trade_index_state {
                    <div class="show-trade">
                        <TruequeMolecule id={id.clone()}/>
                        <ul>
                            <li><GenericButton text = "Concretar Trueque" onclick_event = {show_finish_trade_confirmation}/></li>
                            <li><GenericButton text = "Rechazar Trueque" onclick_event = {show_abort_trade_confirmation}/></li>
                            <li><GenericButton text = "Cancelar Operacion" onclick_event = {cancel_operation}/></li>
                        </ul>
                    </div>
                }
                else {
                    <h2 class="error-text">{"Los códigos ingresados no corresponden a un trueque."}</h2>
                    <GenericButton text = "Cancelar Operación" onclick_event = {cancel_operation} />
                }
                }
            if *finish_confirmation_state{
                <ConfirmPromptButtonMolecule text = "¿Confirma la finalización de este trueque?" confirm_func = {finish_trade} reject_func = {hide_finish_trade_confirmation}/>
            }
            if *abort_confirmation_state{
                <ConfirmPromptButtonMolecule text = "¿Confirma el rechazo a este trueque?" confirm_func = {abort_trade} reject_func = {hide_abort_trade_confirmation}/>
            }
        </div>
    }
}