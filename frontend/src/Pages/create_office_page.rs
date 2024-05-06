use yew::prelude::*;
use crate::Molecules::create_office_molecule::CreateOfficeMolecule;

#[function_component(CreateOfficePage)]
pub fn create_office_page() -> Html {
    html!(
        <CreateOfficeMolecule/>
    )
}