use yew::prelude::*;

use crate::molecules::change_password_from_profile_molecule::ChangePasswordFromProfileMolecule;

#[function_component(ChangePasswordFromProfilePage)]
pub fn change_password_from_profile_page () -> Html {
    html!(
        <ChangePasswordFromProfileMolecule/>
    )
}