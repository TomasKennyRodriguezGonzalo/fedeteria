use datos_comunes::{Descuento, QueryCreateDiscount, QueryGetUserRole, QueryObtenerDescuentos, QueryObtenerGuardadas, ResponseCreateDiscount, ResponseGetUserRole, ResponseObtenerDescuentos, ResponseObtenerGuardadas, RolDeUsuario};
use web_sys::js_sys::Math::exp;
use web_sys::window;
use yew::prelude::*;
use yewdux::use_store;
use yew_hooks::use_effect_once;
use crate::{request_post, store::UserStore};
use crate::components::checked_input_field::CheckedInputField;
use crate::components::dni_input_field::DniInputField;
use chrono::prelude::*;
use crate::components::generic_button::GenericButton;
use crate::molecules::confirm_prompt_button_molecule::ConfirmPromptButtonMolecule;

#[function_component(CreateDiscountPage)]
pub fn create_discount_page() -> Html {
    let (store, _dispatch) = use_store::<UserStore>();
    let dni = store.dni;
    
    let discount_code = use_state(|| None);
    let discount_percentage = use_state(|| None);
    let max_refund = use_state(|| None);
    let min_level = use_state(|| None);
    let expiration_date = use_state(|| None);
    let discounts_list_state = use_state(|| None);

    let cloned_discounts_list = discounts_list_state.clone();
    let role_state = use_state(||None);
    let cloned_role_state = role_state.clone();
    use_effect_once(move || {
        if let Some(dni) = dni {
            let query = QueryGetUserRole { dni };
            //creo que esto no iria si ya se que si o si va a ser el dueño el que entra aca xd
            request_post("/api/obtener_rol", query, move |respuesta:ResponseGetUserRole|{
                cloned_role_state.set(Some(respuesta.rol));
            });
            let query = QueryObtenerDescuentos {nada: true};
            //creo que esto no iria si ya se que si o si va a ser el dueño el que entra aca xd
            request_post("/api/obtener_descuentos", query, move |respuesta:ResponseObtenerDescuentos|{
                cloned_discounts_list.set(Some(respuesta.descuentos.iter().map(|d| d.1.clone()).collect::<Vec<Descuento>>()));
            });
        }
        || {}
    });
    
    let error_state = use_state(||"".to_string());
    let cloned_discounts_list = discounts_list_state.clone();
    let cloned_discount_percentage = discount_percentage.clone();
    let cloned_max_refund = max_refund.clone();
    let cloned_min_level = min_level.clone();
    let cloned_expiration_date = expiration_date.clone();
    let cloned_discount_code = discount_code.clone();
    let error_state_c = error_state.clone();
    let discount_code_changed = Callback::from(move |code:String|{
        cloned_discount_code.set(Some(code.clone()));
        error_state_c.set(detectar_errores((&*cloned_expiration_date).clone(), *cloned_discount_percentage, *cloned_max_refund, *cloned_min_level, Some(code), (&*cloned_discounts_list).clone()));
    });

    let cloned_discounts_list = discounts_list_state.clone();
    let cloned_discount_percentage = discount_percentage.clone();
    let cloned_max_refund = max_refund.clone();
    let cloned_min_level = min_level.clone();
    let cloned_expiration_date = expiration_date.clone();
    let cloned_discount_code = discount_code.clone();
    let error_state_c = error_state.clone();
    let discount_percentage_changed = Callback::from(move |percentage:String|{
        let numero: Result<i64, _> = percentage.parse();
        match numero{
            Ok(percentage)=>{
                cloned_discount_percentage.set(Some(percentage as f64 / 100.0));
                error_state_c.set(detectar_errores((&*cloned_expiration_date).clone(), Some(percentage as f64), *cloned_max_refund, *cloned_min_level, (&*cloned_discount_code).clone(), (&*cloned_discounts_list).clone()));
            }
            Err(_)=>{}
        }
    });
    let cloned_discounts_list = discounts_list_state.clone();
    let cloned_discount_percentage = discount_percentage.clone();
    let cloned_max_refund = max_refund.clone();
    let cloned_min_level = min_level.clone();
    let cloned_expiration_date = expiration_date.clone();
    let cloned_discount_code = discount_code.clone();
    let error_state_c = error_state.clone();
    let max_refund_changed = Callback::from(move |max_refund:String|{
        let numero: Result<i64, _> = max_refund.parse();
        match numero{
            Ok(max_refund)=>{
                cloned_max_refund.set(Some(max_refund));
                error_state_c.set(detectar_errores((&*cloned_expiration_date).clone(), *cloned_discount_percentage, (Some(max_refund).clone()), *cloned_min_level, (&*cloned_discount_code).clone(), (&*cloned_discounts_list).clone()));
            }
            Err(e)=>{}
        }
    });
    let cloned_discounts_list = discounts_list_state.clone();
    let cloned_discount_percentage = discount_percentage.clone();
    let cloned_max_refund = max_refund.clone();
    let cloned_min_level = min_level.clone();
    let cloned_expiration_date = expiration_date.clone();
    let cloned_discount_code = discount_code.clone();
    let error_state_c = error_state.clone();
    let min_level_changed = Callback::from(move |min_level:String|{
        let numero: Result<i64, _> = min_level.parse();
        match numero{
            Ok(min_level)=>{
                cloned_min_level.set(Some(min_level));
                error_state_c.set(detectar_errores((&*cloned_expiration_date).clone(), *cloned_discount_percentage, *cloned_max_refund, Some(min_level), (&*cloned_discount_code).clone(), (&*cloned_discounts_list).clone()));
            }
            Err(e)=>{{}}
        }
    });

    let cloned_discounts_list = discounts_list_state.clone();
    let cloned_discount_percentage = discount_percentage.clone();
    let cloned_max_refund = max_refund.clone();
    let cloned_min_level = min_level.clone();
    let cloned_expiration_date = expiration_date.clone();
    let cloned_discount_code = discount_code.clone();
    let error_state_c = error_state.clone();
    let expiration_date_changed = Callback::from(move |new_date: String| {
        let parsed_date = NaiveDate::parse_from_str(&new_date, "%Y-%m-%d");
        let new_date = parsed_date.unwrap();
        // let time = NaiveTime::from_hms_opt(0, 0, 0);
        let naive_datetime = new_date.and_hms_opt(0, 0, 0).unwrap();
        let new_date: DateTime<Local> = Local.from_local_datetime(&naive_datetime)
            .single()
            .expect("Error al convertir NaiveDateTime a DateTime<Local>");
        cloned_expiration_date.set(Some(new_date.clone()));
        error_state_c.set(detectar_errores(Some(new_date), *cloned_discount_percentage, *cloned_max_refund, *cloned_min_level, (&*cloned_discount_code).clone(), (&*cloned_discounts_list).clone()));
    });

    let show_button_state = use_state(||false);
    let cloned_show_button_state = show_button_state.clone();
    let activate_show_button = Callback::from(move|_|{
        cloned_show_button_state.set(true);
    });

    let cloned_show_button_state = show_button_state.clone();
    let reject_changes = Callback::from(move|_|{
        cloned_show_button_state.set(false);
    });

    let cloned_discounts_list = discounts_list_state.clone();
    let cloned_show_button_state = show_button_state.clone();
    let cloned_discount_code = discount_code.clone();
    let cloned_discount_percentage = discount_percentage.clone();
    let cloned_max_refund = max_refund.clone();
    let cloned_min_level = min_level.clone();
    let cloned_expiration_date = expiration_date.clone();
    let create_discount = Callback::from(move|_|{
        let cloned_show_button_state = cloned_show_button_state.clone();
        if detectar_errores(
        (*cloned_expiration_date).clone(),        
        (*cloned_discount_percentage).clone(),
        (*cloned_max_refund).clone(),
        (*cloned_min_level).clone(),
        (&*cloned_discount_code).clone(),
        (&*cloned_discounts_list).clone(),
        ).is_empty(){
            let query = QueryCreateDiscount{
                codigo_descuento : (&*cloned_discount_code).clone().unwrap(),
                porcentaje : (*cloned_discount_percentage).unwrap(),
                reembolso_max : ((*cloned_max_refund).unwrap() as u64),
                nivel_min : ((*cloned_min_level).unwrap() as u64),
                fecha_exp : (*cloned_expiration_date),
            };
            request_post("/api/crear_descuento", query, move |r:ResponseCreateDiscount|{       
                if r.ok{
                    if let Some(window) = window() {
                        window.location().reload().unwrap();
                        }
                    }
                cloned_show_button_state.set(false);
            });
        }
    });


    html!(
        <div class="create-discount">
            <h1 class="title">{"Crear Descuento"}</h1>
            <CheckedInputField name = "discount_code" label="Ingrese un nuevo codigo de descuento" tipo = "text" on_change = {discount_code_changed} />
            <DniInputField dni = "porcentaje" label="Porcentaje" tipo = "number" handle_on_change = {discount_percentage_changed} />
            <DniInputField dni = "reintegro maximo" label="Reintegro Maximo" tipo = "number" handle_on_change = {max_refund_changed} />
            <DniInputField dni = "nivel minimo" label="Nivel Minimo" tipo = "number" handle_on_change = {min_level_changed} />
            <CheckedInputField name = "expiration_date" label="Fecha De Vencimiento" tipo = "date" on_change = {expiration_date_changed} />

            if (*expiration_date).is_some() && (*discount_code).is_some() && !(*discount_code).clone().unwrap().is_empty() && (*discount_percentage).is_some() && (*max_refund).is_some() && (*error_state).is_empty()  { 
                <GenericButton text = "Crear Descuento" onclick_event = {activate_show_button} /> 
            } else {
                <button class="disabled-dyn-element">{"Crear Descuento"}</button>
            }
            if (&*show_button_state).clone(){
                <ConfirmPromptButtonMolecule text = "Confirmar creacion del descuento" confirm_func = {create_discount} reject_func = {reject_changes}  />
            }
            if !(*error_state).is_empty(){
                <h2 class="error-text">{&*error_state}</h2>
            }
        </div>
    )
}


fn detectar_errores(exp_date:Option<DateTime<Local>>, percentage:Option<f64>, max_refund:Option<i64>, min_level:Option<i64>, discount_code:Option<String>, discount_list: Option<Vec<Descuento>>) -> String {
    let mut error = "".to_string();
    if percentage >= Some(100.0) || Some(0.0) >= percentage && percentage.is_some() {
       error+= "\nEl porcentaje debe ser un numero mayor a 0 y menor a 100. ";
    }
    if max_refund.is_some() && max_refund <= Some(0) {
        error+= "\nEl reintegro debe ser un numero mayor a 0.";
    }
    if min_level.is_some() && min_level < Some(0){
        error+= "\nEl nivel minimo no puede ser negativo. ";       
    }
    if let Some(exp_date) = exp_date{
        if exp_date <= Local::now(){
            error+= "\nLa fecha no puede ser pasada. "; 
        }
    }
    if discount_list.is_some() && discount_code.is_some() {
        if discount_list.clone().unwrap().iter().map(|d| d.codigo.clone()).collect::<Vec<String>>().contains(&discount_code.clone().unwrap()) {
            error+= "\nEl código ingresado ya le pertence a otro descuento. " 
        }
    }
    log::info!("Date {:?}, %{:?}, Refund {:?}, Level {:?}, Code {:?}, List {:?}", exp_date, percentage, max_refund, min_level, discount_code, discount_list);
    error
}