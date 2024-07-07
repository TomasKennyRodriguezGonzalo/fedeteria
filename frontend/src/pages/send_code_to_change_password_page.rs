use yew::prelude::*;

use crate::molecules::send_code_to_change_password_molecule::SendCodeToChangePasswordMolecule;

#[function_component(SendCodeToChangePasswordPage)]
pub fn send_code_to_change_password_page () -> Html {
    html!(
        <SendCodeToChangePasswordMolecule/>
    )
}