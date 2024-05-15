use yew::prelude::*;
use crate::molecules::delete_office_molecule::DeleteOfficeMolecule;

#[function_component(DeleteOffice)]
pub fn delete_office_page() -> Html {

    html!(
        <>
            <DeleteOfficeMolecule/>
        </>
    )
}