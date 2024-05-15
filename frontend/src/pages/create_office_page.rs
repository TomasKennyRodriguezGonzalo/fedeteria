use yew::prelude::*;
use crate::molecules::create_office_molecule::CreateOfficeMolecule;

#[function_component(CreateOfficePage)]
pub fn create_office_page() -> Html {
    html!(
        <CreateOfficeMolecule/>
    )
}