use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use datos_comunes::{self, QueryDeleteOffice, ResponseDeleteOffice, ResponseGetOffices, Sucursal};
use reqwasm::http::Request;
use crate::components::generic_input_field::GenericInputField;
use crate::components::generic_button::GenericButton;
//use crate::components::indexed_button::IndexedButton;

#[function_component(ChangeUserRoleMolecule)]
pub fn change_user_rol_molecule () -> Html {
    let state_input_text = use_state(|| "".to_string());
    let state_input_text_clone = state_input_text.clone();
    let state_input_text_changed = Callback::from(move |text|{
        state_input_text_clone.set(text);
    });
    let state_input_text_clone = state_input_text.clone();

    html!(
        <div class="change-user-role-box">
            <h2>{"Ingrese DNI del usuario al que desea cambiarle el Rol"}</h2>
            <GenericInputField name ="DNI a buscar" label="Ingrese DNI a buscar" tipo = "change_role" handle_on_change = {state_input_text_changed} />
        </div>
    )
}