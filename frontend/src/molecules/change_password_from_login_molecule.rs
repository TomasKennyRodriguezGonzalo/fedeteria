use datos_comunes::{QueryCambioContraseniaLogIn, QueryValidarCambioContrasenia, ResponseCambioContraseniaLogIn, ResponseValidarCambioContrasenia};
use yew::prelude::*;
use yewdux::use_store;
use web_sys::window;

use crate::{components::{checked_input_field::CheckedInputField, dni_input_field::DniInputField, generic_button::GenericButton}, information_store::InformationStore, molecules::confirm_prompt_button_molecule::ConfirmPromptButtonMolecule, request_post};

#[function_component(ChangePasswordFromLogInMolecule)]
pub fn change_password_from_login_molecule () -> Html {
    let (_information_store, information_dispatch) = use_store::<InformationStore>();
    let information_dispatch_cloned = information_dispatch.clone();

    //mail de recuperacion de contraseña (y por lo tanto del usuario)
    let user_mail_state = use_state(|| "".to_string());
    let user_mail_state_cloned = user_mail_state.clone();
    let user_mail_onchange = Callback::from(move |mail| {
        user_mail_state_cloned.set(mail);
    });

    //codigo de recuperacion de contraseña
    let code_state = use_state(|| 0);
    let code_state_cloned = code_state.clone();
    let code_onchange = Callback::from(move |code: String| {
        code_state_cloned.set(code.parse::<u64>().unwrap());
    });

    //estado que valida el cambio de contraseña efectivo
    let valid_inputs_state = use_state(|| false);
    
    //valido los datos ingresados
    let cloned_valid_inputs_state = valid_inputs_state.clone();
    let code_state_cloned = code_state.clone();
    let user_mail_state_cloned = user_mail_state.clone();
    let check_inputs = Callback::from(move |()| {
        let cloned_valid_inputs_state = cloned_valid_inputs_state.clone();
        let information_dispatch_cloned = information_dispatch_cloned.clone();
        let query = QueryValidarCambioContrasenia {email: (&*user_mail_state_cloned).clone(), codigo: (&*code_state_cloned).clone()};
        request_post("/api/validar_cambio_contrasenia", query, move |respuesta: ResponseValidarCambioContrasenia| {
            if respuesta.datos_validos {
                cloned_valid_inputs_state.set(true)
            }
            else {
                cloned_valid_inputs_state.set(false);
                information_dispatch_cloned.reduce_mut( |store| store.messages.push(format!("Los datos ingresados no corresponden a una petición de cambio de contraseña")));
            }
        });
    });

    //nueva contraseña
    let new_password_state = use_state(|| "".to_string());
    let new_password_state_cloned = new_password_state.clone();
    let new_password_onchange = Callback::from(move |password| {
        new_password_state_cloned.set(password);
    });

    //confimacion de nueva contraseña
    let confirm_new_password_state = use_state(|| "".to_string());
    let confirm_new_password_state_cloned = confirm_new_password_state.clone();
    let confirm_new_password_onchange = Callback::from(move |password| {
        confirm_new_password_state_cloned.set(password);
    });

    //cambio de contraseña, envio el mail y el codigo para primero buscar el usuario, segundo Francia, 
    //y tercero para marcar como usada la peticion de cambio de contraseña
    let information_dispatch_cloned = information_dispatch.clone();
    let new_password_state_cloned = new_password_state.clone();
    let user_mail_state_cloned = user_mail_state.clone();
    let code_state_cloned = code_state.clone();
    let change_password = Callback::from(move |_e| {
        let information_dispatch_cloned = information_dispatch_cloned.clone();
        let query = QueryCambioContraseniaLogIn {email: (&*user_mail_state_cloned).clone(), codigo: (&*code_state_cloned).clone(), nueva_contrasenia: (&*new_password_state_cloned).clone()};
        request_post("/api/cambiar_contrasenia_login", query, move |respuesta: ResponseCambioContraseniaLogIn| {
            if respuesta.cambio {
                information_dispatch_cloned.reduce_mut( |store| store.messages.push(format!("La contraseña ha sido cambiada exitosamente")));
                if let Some(window) = window() {
                    window.location().reload().unwrap();
                }
            }
            else {
                information_dispatch_cloned.reduce_mut( |store| store.messages.push(format!("No es posible cambiar la contraseña a la ingresada")));
            }
        });
    });

    //botones de confirmacion de cambio de contraseña
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
        <div> //cambio de contraseña desde el login
            <div class="text"> //validacion de datos
                <h2>{"Seccion de cambio de contraseña"}</h2>
                <ul>
                    <li>{"1 - Ingrese la dirección de email con la que pidió el codigo de recuperación de contraseña"}</li>
                    <li>{"2 - Ingrese el codigo de recuperación de contraseña"}</li>
                    <li>{"3 - Presiones 'Validar Datos'"}</li>
                    <li>{"4 - Si los datos son correctos, le aparecerán los campos para ingresar y confirmar su nueva contraseña"}</li>
                    <li>{"5 - Presiona 'Cambiar Contraseña' para efectivamente establecer la nueva contraseña. El Botón se habilitará de ser ambos campos coincidentes"}</li>
                </ul>
                <div class="edit-inputs">
                    <h2>{"Ingrese la dirección de email"}</h2>
                    <CheckedInputField name="Mail" placeholder="direccion@email" tipo="text" on_change={user_mail_onchange}/>
                    <br/>
                    <h2>{"Ingrese el código de recuperación de contraseña"}</h2>
                    <DniInputField dni = "Codigo de Recuperación" tipo = "number" handle_on_change = {code_onchange} />
                    <br/>
                    <GenericButton text="Validar Datos" onclick_event={check_inputs}/>
                </div>
            </div>
                if (&*valid_inputs_state).clone() {
                    <div class="text"> //cambio de contraseña efectivo
                        <h2>{"Ingrese la nueva contraseña"}</h2>
                        <CheckedInputField name = "password" label="Nueva Contraseña" tipo = "password" on_change={new_password_onchange}/>
                        <br/>
                        <h2>{"Ingrese la nueva contraseña"}</h2>
                        <CheckedInputField name="password" placeholder="Confirmación Nueva Contraseña" tipo="password" on_change={confirm_new_password_onchange}/>
                        <br/>
                        if ((&*new_password_state).clone() != "".to_string()) && ((&*confirm_new_password_state).clone() != "".to_string()) && ((&*new_password_state).clone() == (&*confirm_new_password_state).clone()) {
                            <GenericButton text="Cambiar Contraseña" onclick_event={show_confirm_buttons}/>
                        }
                        else {
                            <button class="disabled-dyn-element">{"Cambiar Contraseña"}</button>
                        }
                    </div>
                }
            if (&*confirm_buttons_state).clone() {
                <ConfirmPromptButtonMolecule text={format!("¿Desea establecer como contraseña, la contraseña ingresada?")} confirm_func={change_password} reject_func={reject_func} />
            }
        </div>
    )
}