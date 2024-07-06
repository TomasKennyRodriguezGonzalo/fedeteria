use datos_comunes::{QueryCreateDiscount, QueryGetUserRole, QueryObtenerGuardadas, ResponseCreateDiscount, ResponseGetUserRole, ResponseObtenerGuardadas, RolDeUsuario};
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
    let discount_code = use_state(|| "".to_string());
    let discount_porcentage = use_state(|| 0.0);
    let max_refund = use_state(|| 0);
    let min_level = use_state(|| 0);

    let role_state = use_state(||None);
    let cloned_role_state = role_state.clone();
    use_effect_once(move || {
        if let Some(dni) = dni {
            let query = QueryGetUserRole { dni };
            //creo que esto no iria si ya se que si o si va a ser el dueño el que entra aca xd
            request_post("/api/obtener_rol", query, move |respuesta:ResponseGetUserRole|{
                cloned_role_state.set(Some(respuesta.rol));
            });
        }
        || {}
    });

    let cloned_discount_code = discount_code.clone();
    let discount_code_changed = Callback::from(move |code:String|{
        cloned_discount_code.set(code);
    });

    let cloned_discount_porcentage = discount_porcentage.clone();
    let discount_porcentage_changed = Callback::from(move |porcentage:String|{
        let numero: Result<u64, _> = porcentage.parse();
        match numero{
            Ok(porcentage)=>{
                if porcentage > 0 && porcentage < 100{
                    cloned_discount_porcentage.set(porcentage as f64 / 100.0);
                }
            }
            Err(_)=>{}
        }
    });

    let cloned_max_refund = max_refund.clone();
    let max_refund_changed = Callback::from(move |max_refund:String|{
        let numero: Result<u64, _> = max_refund.parse();
        match numero{
            Ok(max_refund)=>{
                cloned_max_refund.set(max_refund);
            }
            Err(_)=>{}
        }
    });

    let cloned_min_level = min_level.clone();
    let min_level_changed = Callback::from(move |min_level:String|{
        let numero: Result<u64, _> = min_level.parse();
        match numero{
            Ok(min_level)=>{
                cloned_min_level.set(min_level);
            }
            Err(_)=>{}
        }
    });



    let expiration_date = use_state(|| None);
    let cloned_expiration_date = expiration_date.clone();


    let expiration_date_changed = Callback::from(move |new_date: String| {
        let parsed_date = NaiveDate::parse_from_str(&new_date, "%Y-%m-%d");
        let new_date = parsed_date.unwrap();
        // let time = NaiveTime::from_hms_opt(0, 0, 0);
        let naive_datetime = new_date.and_hms_opt(0, 0, 0).unwrap();
        let new_date: DateTime<Local> = Local.from_local_datetime(&naive_datetime)
            .single()
            .expect("Error al convertir NaiveDateTime a DateTime<Local>");

        //ATENCION ATENCION falta chequear que esto no sea anterior al dia de hoy
        cloned_expiration_date.set(Some(new_date));
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

    let cloned_discount_code = discount_code.clone();
    let cloned_discount_porcentage = discount_porcentage.clone();
    let cloned_max_refund = max_refund.clone();
    let cloned_min_level = min_level.clone();
    let cloned_expiration_date = expiration_date.clone();
    let create_discount = Callback::from(move|_|{
        let query = QueryCreateDiscount{
            codigo_descuento : (&*cloned_discount_code).clone(),
            porcentaje : (*cloned_discount_porcentage),
            reembolso_max : (*cloned_max_refund),
            nivel_min : (*cloned_min_level),
            fecha_exp : (*cloned_expiration_date),
        };
        request_post("/api/crear_descuento", query, move |_:ResponseCreateDiscount|{

        });

        if let Some(window) = window() {
            window.location().reload().unwrap();
        }

        
    });

    html!(
        <h1>
            <CheckedInputField name = "discount_code" label="Ingrese un nuevo codigo de descuento" tipo = "text" on_change = {discount_code_changed} />
            <DniInputField dni = "porcentaje" label="porcentaje" tipo = "number" handle_on_change = {discount_porcentage_changed} />
            <DniInputField dni = "reintegro maximo" label="Reintegro Maximo" tipo = "number" handle_on_change = {max_refund_changed} />
            <DniInputField dni = "nivel minimo" label="Nivel Minimo" tipo = "number" handle_on_change = {min_level_changed} />
            <CheckedInputField name = "expiration_date" label="Fecha De Vencimiento" tipo = "date" on_change = {expiration_date_changed} />
            <GenericButton text = "crear descuento" onclick_event = {activate_show_button} />
            if (&*show_button_state).clone(){
                <ConfirmPromptButtonMolecule text = "Confirmar creacion del descuento" confirm_func = {create_discount} reject_func = {reject_changes}  />
            }
        </h1>

    )
}