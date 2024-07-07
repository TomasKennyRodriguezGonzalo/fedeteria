use crate::molecules::confirm_prompt_button_molecule::ConfirmPromptButtonMolecule;
use datos_comunes::{ErrorEnConcretacion, EstadoTrueque, QueryFinishTrade, /*QueryGetOffice,*/ QueryGetUserRole, /*QueryObtenerTrueque,*/ QueryTruequesFiltrados, ResponseFinishTrade, /*ResponseGetOffice,*/ ResponseGetUserRole, /*ResponseObtenerTrueque,*/ ResponseTruequePorCodigos, RolDeUsuario};
use web_sys::window;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yewdux::use_store;

use crate::{components::{dni_input_field::DniInputField, checked_input_field::CheckedInputField, generic_button::GenericButton}, information_store::InformationStore, molecules::trueque_molecule::TruequeMolecule, request_post, store::UserStore};

#[function_component(FinishTradeMolecule)]
pub fn finish_trade_molecule () -> Html {
    let (user_store, _user_dispatch) = use_store::<UserStore>();
    let dni = user_store.dni.unwrap();
    let cloned_dni = dni.clone();
    
    let ventas_ofertante_state:UseStateHandle<u64> = use_state(|| 0);
    let ventas_receptor_state:UseStateHandle<u64> = use_state(|| 0);
    let ventas_ofertante_state_cloned = ventas_ofertante_state.clone();
    let ventas_receptor_state_cloned = ventas_receptor_state.clone();
    let descuento_ofertante_state:UseStateHandle<Option<String>> = use_state(|| None);
    let descuento_receptor_state:UseStateHandle<Option<String>> = use_state(|| None);

    let error_text_state = use_state(|| "".to_string());

    let (_information_store, information_dispatch) = use_store::<InformationStore>();

    let finish_confirmation_state = use_state(|| false);
    let abort_confirmation_state = use_state(|| false);
    let cancel_confirmation_state = use_state(|| false);

    //obtengo el rol de usuario, para distinguir la sucursal de ser empleado
    let role_state = use_state(|| RolDeUsuario::Normal);
    let cloned_role_state = role_state.clone();

    use_effect_once(move || {
        let query = QueryGetUserRole {dni: cloned_dni};
        request_post("/api/obtener_rol", query, move |respuesta:ResponseGetUserRole|{
            cloned_role_state.set(respuesta.rol.clone());
            log::info!("RESPUESTA OBTENER ROL: {:?}", respuesta.rol);
        });
        || {}
    });
    let cloned_role_state = role_state.clone();

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

    /*//estado de la ganancia
    let gains_state = use_state(|| None);
    let cloned_gains_state = gains_state.clone();
    let gains_onchange = Callback::from(move |code: String| {
        cloned_gains_state.set(Some(code.parse::<u64>().unwrap()));
    });
    let cloned_gains_state = gains_state.clone();*/

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
        log::info!("ROL: {:?}", &*cloned_role_state);
        //armo la query para empleado
        if let RolDeUsuario::Empleado { sucursal }  = &*cloned_role_state {
            query = QueryTruequesFiltrados {
                filtro_codigo_ofertante: Some((&*cloned_offer_code_state).clone()),
                filtro_codigo_receptor: Some((&*cloned_receptor_code_state).clone()),
                filtro_dni_integrantes: None,
                filtro_estado: None,
                filtro_fecha_pactada: None,
                filtro_fecha_trueque: None,
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
                filtro_fecha_pactada: None,
                filtro_fecha_trueque: None,
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

    let cloned_information_dispatch = information_dispatch.clone();
    let cloned_finish_confirmation_state = finish_confirmation_state.clone();
    let cloned_trade_index_state = trade_index_state.clone();
    let cloned_show_trade_search_state = show_trade_search_state.clone();
    let ventas_ofertante_state_cloned = ventas_ofertante_state.clone();
    let ventas_receptor_state_cloned = ventas_receptor_state.clone();
    let descuento_ofertante_state_cloned = descuento_ofertante_state.clone();
    let descuento_receptor_state_cloned = descuento_receptor_state.clone();
    let cloned_error_text_state = error_text_state.clone();

    let finish_trade = Callback::from(move |_e| {
        let cloned_error_text_state = cloned_error_text_state.clone();
        let cloned_information_dispatch = cloned_information_dispatch.clone();
        let cloned_finish_confirmation_state = cloned_finish_confirmation_state.clone();
        let cloned_show_trade_search_state = cloned_show_trade_search_state.clone();
        let ventas_ofertante = *ventas_ofertante_state_cloned;
        let ventas_receptor = *ventas_receptor_state_cloned;
        if let Some(descuento_ofertante) = (*descuento_ofertante_state_cloned).clone() {
            if descuento_ofertante.is_empty(){
                descuento_ofertante_state_cloned.set(None);
            }
        }
        if let Some(descuento_receptor) = (*descuento_receptor_state_cloned).clone() {
            if descuento_receptor.is_empty(){
                descuento_receptor_state_cloned.set(None);
            }
        }
        let query = QueryFinishTrade {
            estado: EstadoTrueque::Finalizado, 
            id_trueque: (&*cloned_trade_index_state).unwrap().clone(),
            ventas_ofertante,
            ventas_receptor,
            codigo_descuento_ofertante: (*descuento_ofertante_state_cloned).clone(),
            codigo_descuento_receptor: (*descuento_receptor_state_cloned).clone(),
        };
        request_post("/api/finalizar_trueque", query, move |respuesta: ResponseFinishTrade| {
            log::info!("respuesta: {:?}", respuesta);
            let cloned_finish_confirmation_state = cloned_finish_confirmation_state.clone();
            let cloned_show_trade_search_state = cloned_show_trade_search_state.clone();
            let cloned_information_dispatch = cloned_information_dispatch.clone();
            match respuesta.respuesta {
                Ok(estado) => {
                    if estado {
                        log::info!("entre al if del estado en true");
                        // Resultado exitoso del concretar trueque
                        // - Informar por pantalla (feedback)
                        cloned_show_trade_search_state.set(false);
                        cloned_information_dispatch.reduce_mut(|store| store.messages.push(format!("Trueque concretado!")));
                        if let Some(window) = window() {
                            window.location().reload().unwrap();
                        }
                    }
                },
                Err(e) => {
                    match e {
                        ErrorEnConcretacion::DescuentoOfertanteInvalido => {
                            log::error!("descuento ofertante invalido");
                            cloned_error_text_state.set(format!("El descuento ingresado para el ofertante no existe o ha vencido. "));
                        },
                        ErrorEnConcretacion::DescuentoOfertanteUtilizado => {
                            log::error!("descuento ofertante utilizado");
                            cloned_error_text_state.set(format!("El descuento ingresado para el ofertante ya fue utilizado. "));
                        }
                        ErrorEnConcretacion::DescuentoReceptorInvalido => {
                            log::error!("descuento receptor invalido");
                            cloned_error_text_state.set(format!("El descuento ingresado para el receptor no existe o ha vencido. "));
                        },
                        ErrorEnConcretacion::DescuentoReceptorUtilizado => {
                            log::error!("descuento receptor utilizado");
                            cloned_error_text_state.set(format!("El descuento ingresado para el receptor ya fue utilizado. "));
                        }
                    }
                } 
            }
            log::info!("termine el callback!!");
            cloned_finish_confirmation_state.set(false);
        });   
    });

    //rechazo el trueque
    let cloned_information_dispatch = information_dispatch.clone();
    let cloned_abort_confirmation_state = abort_confirmation_state.clone();
    let cloned_trade_index_state = trade_index_state.clone();
    let cloned_show_trade_search_state = show_trade_search_state.clone();
    let abort_trade = Callback::from(move |_e| {
        let query = QueryFinishTrade {
            estado: EstadoTrueque::Rechazado, 
            id_trueque: (&*cloned_trade_index_state).unwrap().clone(),
            ventas_ofertante:0,
            ventas_receptor:0,
            codigo_descuento_ofertante: None,
            codigo_descuento_receptor: None,
        };
        request_post("/api/finalizar_trueque", query, move |_respuesta: ResponseFinishTrade| {
            // No hago nada porque se que me va a retornar vacio, esta implementacion del mismo metodo para dos cosas diferentes es medio extraña
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

    let ventas_ofertante_state_cloned = ventas_ofertante_state.clone();
    let ventas_ofertante_state_changed = Callback::from(move |ventas_ofertante:String|{
        ventas_ofertante_state_cloned.set(ventas_ofertante.parse::<u64>().unwrap())
    });

    let ventas_receptor_state_cloned = ventas_receptor_state.clone();
    let ventas_receptor_state_changed = Callback::from(move |ventas_receptor:String|{
        ventas_receptor_state_cloned.set(ventas_receptor.parse::<u64>().unwrap())
    });

    let descuento_ofertante_state_cloned = descuento_ofertante_state.clone();
    let descuento_ofertante_state_changed = Callback::from(move |descuento_ofertante:String|{
        if descuento_ofertante.is_empty() {
            descuento_ofertante_state_cloned.set(None)
        } else {
            descuento_ofertante_state_cloned.set(Some(descuento_ofertante))
        }
    });

    let descuento_receptor_state_cloned = descuento_receptor_state.clone();
    let descuento_receptor_state_changed = Callback::from(move |descuento_receptor:String|{
        if descuento_receptor.is_empty() {
            descuento_receptor_state_cloned.set(None)
        } else {
            descuento_receptor_state_cloned.set(Some(descuento_receptor))
        }
    });
    
    let cloned_trade_index_state = trade_index_state.clone();
    let cloned_show_trade_search_state = show_trade_search_state.clone();
    html! {
        <div class="finish-trade">
            <div class="codes-input">  
                <h2>{"Ingrese Código de Trueque del Usuario Receptor de la oferta"}</h2>
                <DniInputField dni = "Codigo Receptor" tipo = "number" handle_on_change = {receptor_code_onchange} />
                <h2>{"Ingrese Código de Trueque del Usuario Ofertante de la oferta"}</h2>
                <DniInputField dni = "Codigo Ofertante" tipo = "number" handle_on_change = {offer_code_onchange} />
                <GenericButton text = "Buscar Trueque" onclick_event = {search_trade} />
            </div>
            if *cloned_show_trade_search_state {
                if let Some(id) = &*cloned_trade_index_state {
                    <div class="show-trade">
                        <TruequeMolecule id={id.clone()}/>
                            <div class="sales-discounts">
                                <div class="sales-inputs">
                                    <CheckedInputField name = "ventas-ofertante" placeholder="Ventas Ofertante" tipo = "number" on_change = {ventas_ofertante_state_changed}/>
                                    <CheckedInputField name = "ventas-receptor" placeholder="Ventas Receptor" tipo = "number" on_change = {ventas_receptor_state_changed}/>
                                </div>
                                <div class="discounts-inputs">
                                    if (*ventas_ofertante_state.clone()) != 0 {
                                        <CheckedInputField name = "descuento-ofertante" placeholder="Descuento Ofertante" tipo = "text" on_change = {descuento_ofertante_state_changed}/>
                                    }
                                    if (*ventas_receptor_state.clone()) != 0 {
                                        <CheckedInputField name = "descuento-receptor" placeholder="Descuento Receptor" tipo = "text" on_change = {descuento_receptor_state_changed}/>
                                    }
                                </div>
                            </div>
                            <div class="operation-buttons">
                                <GenericButton text = "Concretar Trueque" onclick_event = {show_finish_trade_confirmation}/>                          
                                <GenericButton text = "Rechazar Trueque" onclick_event = {show_abort_trade_confirmation}/>
                                <GenericButton text = "Cancelar Operacion" onclick_event = {cancel_operation}/>
                            </div>
                            if !(*error_text_state).is_empty() {
                                <h2 class="error-text">{(*error_text_state).clone()}</h2>
                            }
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
            if !(*error_text_state).clone().is_empty() {
                <h1 class="error-text">{(*error_text_state).clone()}</h1>
            }
        </div>
    }
}