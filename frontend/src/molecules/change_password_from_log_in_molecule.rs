use datos_comunes::{QuerySendChangePasswordCode, ResponseSendChangePasswordCode};
use yew::prelude::*;
use yewdux::use_store;
use web_sys::window;
use crate::{components::{checked_input_field::CheckedInputField, generic_button::GenericButton}, information_store::InformationStore, request_post};

#[function_component(ChangePasswordFromLogInMolecule)]
pub fn change_password_from_log_in_molecule () -> Html {

    let (_information_store, information_dispatch) = use_store::<InformationStore>();
    let information_dispatch_cloned = information_dispatch.clone();

    let user_mail_state = use_state(|| "".to_string());
    let user_mail_state_cloned = user_mail_state.clone();

    let user_mail_onchange = Callback::from(move |mail| {
        user_mail_state_cloned.set(mail);
    });
    
    let user_mail_state_cloned = user_mail_state.clone();
    let send_code_to_email = Callback::from(move |()| {
        let email = (&*user_mail_state_cloned).clone();
        request_post("/api/enviar_codigo_de_recuperacion_contrasenia", QuerySendChangePasswordCode {email}, move |_respuesta: ResponseSendChangePasswordCode| {});
        information_dispatch_cloned.reduce_mut( |store| store.messages.push(format!("De ser válido el correo {} en el sistema, se ha enviado un correo con el código de seguridad para cambiar su contraseña. Siga los pasos en el Inicio de Sesion para cambiar su contraseña", (&*user_mail_state_cloned).clone())));
        if let Some(window) = window() {
            window.location().reload().unwrap();
        }
    });

    html!(
        <div class="text">
            <div class="edit-inputs">
                <h2>{"Ingrese su dirección de email en la página para pedir un código de recuperación de contraseña"}</h2>
                <CheckedInputField name="Mail de recuperacion de contraseña" placeholder="direccion@email" tipo="text" on_change={user_mail_onchange}/>
                <GenericButton text="Enviar Código" onclick_event={send_code_to_email}/>
            </div>
        </div>
    )
}