use datos_comunes::{QueryCambioContraseniaPerfil, ResponseCambioContrasenia};
use web_sys::window;
use yew::prelude::*;
use yewdux::use_store;

use crate::{components::{checked_input_field::CheckedInputField, generic_button::GenericButton}, information_store::InformationStore, molecules::confirm_prompt_button_molecule::ConfirmPromptButtonMolecule, request_post, store::UserStore};

#[function_component(ChangePasswordFromProfileMolecule)]
pub fn change_password_from_profile_molecule () -> Html {
    let (_information_store, information_dispatch) = use_store::<InformationStore>();

    let (store, _dispatch) = use_store::<UserStore>();
    let dni = store.dni;

    //vieja contraseña
    let old_password_state = use_state(|| "".to_string());
    let old_password_state_cloned = old_password_state.clone();
    let old_password_onchange = Callback::from(move |password| {
        old_password_state_cloned.set(password);
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

    //cambio la contraseña
    let information_dispatch_cloned = information_dispatch.clone();
    let new_password_state_cloned = new_password_state.clone();
    let old_password_state_cloned = old_password_state.clone();
    let dni_cloned = dni.clone();
    let change_password = Callback::from(move |_e| {
        let information_dispatch_cloned = information_dispatch_cloned.clone();
        let query = QueryCambioContraseniaPerfil {nueva_contrasenia: (&*new_password_state_cloned).clone(), dni: dni_cloned.unwrap(), vieja_contrasenia: (&*old_password_state_cloned).clone()};
        request_post("/api/cambiar_contrasenia_perfil", query, move |respuesta: ResponseCambioContrasenia| {
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
        <div>
            <div class="edit-personal-info-box"> //cambio de contraseña efectivo
                <h2>{"Ingrese la vieja contraseña"}</h2>
                <CheckedInputField name = "password" placeholder="Vieja Contraseña" tipo = "password" on_change={old_password_onchange}/>
                <br/>
                <h2>{"Ingrese la nueva contraseña"}</h2>
                <CheckedInputField name = "password" placeholder="Nueva Contraseña" tipo = "password" on_change={new_password_onchange}/>
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
            if (&*confirm_buttons_state).clone() {
                <ConfirmPromptButtonMolecule text={format!("¿Desea establecer como contraseña, la contraseña ingresada?")} confirm_func={change_password} reject_func={reject_func} />
            }
        </div>
    )
}