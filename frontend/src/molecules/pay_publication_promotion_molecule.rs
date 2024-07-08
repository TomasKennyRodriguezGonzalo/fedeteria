use chrono::{Local, NaiveDate, TimeZone};
use datos_comunes::{QueryPagarPromocion, QueryPagarPromocionPublicaciones, ResponsePagarPromocion, ResponseRechazarOferta, Tarjeta};
use web_sys::{window, HtmlInputElement};
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_router::{hooks::use_navigator, navigator};
use yewdux::use_store;


use crate::{components::{checked_input_field::CheckedInputField, dni_input_field::DniInputField, generic_button::GenericButton}, information_store::InformationStore, molecules::confirm_prompt_button_molecule::ConfirmPromptButtonMolecule, request_post, router::Route};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub query: Option<QueryPagarPromocionPublicaciones>,
}

#[function_component(PayPublicationPromotionMolecule)]
pub fn pay_publication_promotion_molecule (props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let (_information_store, information_dispatch) = use_store::<InformationStore>();

    //me la juego a que es Some() siempre
    let query = props.query.as_ref().unwrap();
    log::info!("PUBLICACIONES: {:?}", query.publicaciones);

    //obtengo el vec de indices de publicaciones
    let indices_publicaciones: Vec<usize> = serde_json::from_str(&query.publicaciones).unwrap();

    //dni del titular
    let dni_state = use_state(|| 0);
    let dni_state_cloned = dni_state.clone();
    let dni_onchange = Callback::from(move |dni: String| {
        dni_state_cloned.set(dni.parse::<u64>().unwrap());
    });

    //nombre titular
    let titular_name_state = use_state(|| "".to_string());
    let titular_name_state_cloned = titular_name_state.clone();
    let titular_name_onchange = Callback::from(move |titular_name| {
        titular_name_state_cloned.set(titular_name);
    });

    //numero de tarjeta
    let card_number_state: UseStateHandle<u64> = use_state(|| 0);
    let card_number_state_cloned = card_number_state.clone();
    let card_number_onchange = Callback::from(move |card_number: String| {
        card_number_state_cloned.set(card_number.parse::<u64>().unwrap());
    });

    //codigo de seguridad
    let card_security_code_state = use_state(|| 0);
    let card_security_code_state_cloned = card_security_code_state.clone();
    let card_security_code_onchange = Callback::from(move |card_security_code: String| {
        card_security_code_state_cloned.set(card_security_code.parse::<u64>().unwrap());
    });

    //anio
    let anio_state = use_state(|| 0);
    let anio_state_cloned = anio_state.clone();
    let anio_onchange = Callback::from(move |anio: String| {
        anio_state_cloned.set(anio.parse::<u64>().unwrap());
    });

    //select mes
    let select_mes_value_state = use_state(|| -1);
    let select_mes_value_state_cloned = select_mes_value_state.clone();
    let select_mes_onchange = Callback::from(move|event: Event| {
        let select_mes_value_state_cloned = select_mes_value_state_cloned.clone();
        let target = event.target().unwrap();
        let input:HtmlInputElement = target.unchecked_into();
        let value: i32 = input.value().parse().unwrap();
        select_mes_value_state_cloned.set(value);
        log::info!("Select mes changed to {}", value)
    });
    
    //pago
    let precio = query.precio;
    let fecha_limite_promocion = query.fecha_fin_promocion;
    let navigator_cloned = navigator.clone();
    let information_dispatch_cloned = information_dispatch.clone();
    let cloned_select_mes_value_state = select_mes_value_state.clone();
    let dni_state_cloned = dni_state.clone();
    let titular_name_state_cloned = titular_name_state.clone();
    let card_number_state_cloned = card_number_state.clone();
    let card_security_code_state_cloned = card_security_code_state.clone();
    let anio_state_cloned = anio_state.clone();
    let indices_publicaciones_cloned = indices_publicaciones.clone();
    let pay = Callback::from(move |_e| {
        let precio = precio.clone();
        let meses = obtener_meses();
        let indice_mes = (&*cloned_select_mes_value_state).clone() as usize;
        let mes_ingresado = &meses[indice_mes];
        let information_dispatch_cloned = information_dispatch_cloned.clone();
        let navigator_cloned = navigator_cloned.clone();
        let indices_publicaciones_cloned = indices_publicaciones_cloned.clone();
        let fecha_limite_promocion_cloned = fecha_limite_promocion.clone();
        //creo una tarjeta de juguete
        let tarjeta_ingresada = Tarjeta {
            codigo_seguridad: (&*card_security_code_state_cloned).clone(),
            dni_titular: (&*dni_state_cloned).clone(),
            numero_tarjeta: (&*card_number_state_cloned).clone(),
            anio_caducidad: (&*anio_state_cloned).clone(),
            mes_caducidad: mes_ingresado.parse::<u64>().unwrap(),
            monto: 0,
            nombre_titular: (&*titular_name_state_cloned).clone(),
        };

        let query = QueryPagarPromocion {
            tarjeta: tarjeta_ingresada, 
            precio, 
            publicaciones: indices_publicaciones_cloned, 
            fecha_limite_promocion: fecha_limite_promocion_cloned
        };

        request_post("/api/pagar", query, move |respuesta:ResponsePagarPromocion| {
            if respuesta.pago {
                information_dispatch_cloned.reduce_mut(|store| store.messages.push(format!("El pago se ha realizado correctamente!")));
                let _ = navigator_cloned.push(&Route::Home);
            }
            else {
                information_dispatch_cloned.reduce_mut(|store| store.messages.push(format!("La tarjeta ingresada no es válida o cuenta con fondos insuficientes")));
                if let Some(window) = window() {
                    window.location().reload().unwrap();
                }
            }
        });
    });

    //botones de confirmacion de pago
    let confirm_buttons_state = use_state(|| false);
    let confirm_buttons_state_cloned = confirm_buttons_state.clone();
    let show_confirm_buttons = Callback::from(move |()| {
        confirm_buttons_state_cloned.set(true);
    });

    let confirm_buttons_state_cloned = confirm_buttons_state.clone();
    let reject_func = Callback::from(move |_e| {
        confirm_buttons_state_cloned.set(false);
    });

    html!(
        <div class="pay-promotion-section"> //pagos box
            <h1 class="title">{"Seccion pagos Fedeteria"}</h1>
            <h2>{"Todos los datos ingresados deben coincidir con una tarjeta existente y vigente, de lo contrario, no se realizará la transacción"}</h2>
            <h2>{format!("Monto a pagar: ${}", query.precio)}</h2>
            <div class="data-inputs">
                <h2>{"Ingrese DNI del titular de la tarjeta"}</h2>
                <DniInputField dni = "DNI" tipo = "number" handle_on_change = {dni_onchange} />
                <h2>{"Ingrese el nombre del titular de la tarjeta, tal cual se indica en la tarjeta"}</h2>
                <CheckedInputField name="Nombre Titular" placeholder="Nombre Titular" tipo="text" on_change={titular_name_onchange}/>
                <h2>{"Ingrese el número de la tarjeta"}</h2>
                <DniInputField dni = "Numero Tarjeta" tipo = "number" handle_on_change = {card_number_onchange} />
                <h2>{"Ingrese el código de seguridad de la tarjeta"}</h2>
                <DniInputField dni = "Codigo de Seguridad" tipo = "number" handle_on_change = {card_security_code_onchange} />
                <h2>{"Ingrese la fecha de caducidad de la tarjeta"}</h2>
                <h3>{"Año"}</h3>
                <DniInputField dni = "Anio" tipo = "number" handle_on_change = {anio_onchange} />
                <h3>{"Mes"}</h3>
                <div class="time-selector">
                    <select value="select-hora" id="horas" onchange={select_mes_onchange.clone()}>
                        <option value="-1" selected=true>{"---"}</option>
                        {
                            obtener_meses().iter().enumerate().map(|(index, mes)| html!{
                                <option value={index.to_string()}>{mes}</option>
                            }).collect::<Html>()
                        }
                    </select>
                </div>
                <GenericButton text="Validar Datos" onclick_event={show_confirm_buttons}/>
            </div>
            if (&*confirm_buttons_state).clone() {
                <ConfirmPromptButtonMolecule text={format!("¿Desea realizar el pago por ${}", query.precio)} confirm_func={pay} reject_func={reject_func} />
            }
        </div>
    )
}

fn obtener_meses() -> Vec<String> {
    let mut vec_horas = Vec::new();
    vec_horas.push("1".to_string());
    vec_horas.push("2".to_string());
    vec_horas.push("3".to_string());
    vec_horas.push("4".to_string());
    vec_horas.push("5".to_string());
    vec_horas.push("6".to_string());
    vec_horas.push("7".to_string());
    vec_horas.push("8".to_string());
    vec_horas.push("9".to_string());
    vec_horas.push("10".to_string());
    vec_horas.push("11".to_string());
    vec_horas.push("12".to_string());
    vec_horas
}