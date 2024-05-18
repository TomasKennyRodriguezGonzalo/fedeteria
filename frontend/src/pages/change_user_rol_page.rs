use yew::prelude::*;

use crate::molecules::change_user_role_molecule::ChangeUserRoleMolecule;

#[function_component(ChangeUserRolePage)]
pub fn change_user_rol_page () -> Html {
    html!(
        <ChangeUserRoleMolecule/>
    )
}