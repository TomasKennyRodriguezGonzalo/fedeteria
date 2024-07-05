use yew::prelude::*;

use crate::molecules::change_password_from_login_molecule::ChangePasswordFromLogInMolecule;

#[function_component(ChangePasswordFromLogInPage)]
pub fn change_password_from_login_page () -> Html {
    html!(
        <ChangePasswordFromLogInMolecule/>
    )
}