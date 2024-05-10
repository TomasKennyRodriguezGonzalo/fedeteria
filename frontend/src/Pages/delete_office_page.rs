use yew::prelude::*;
use crate::Molecules::delete_office_molecule::DeleteOfficeMolecule;

#[function_component(DeleteOffice)]
pub fn delete_office_page() -> Html {

    html!(
        <>
            <DeleteOfficeMolecule/>
        </>
    )
}