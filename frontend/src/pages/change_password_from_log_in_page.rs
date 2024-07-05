use yew::prelude::*;

use crate::molecules::change_password_from_log_in_molecule::ChangePasswordFromLogInMolecule;

#[function_component(ChangePasswordFromLogInPage)]
pub fn change_password_from_log_in_page () -> Html {
    html!(
        <ChangePasswordFromLogInMolecule/>
    )
}