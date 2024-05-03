use crate::Molecules::{create_publication_molecule::CreatePublicationMolecule};
use yew::prelude::*;

#[function_component(CreatePublicationPage)]
pub fn create_publication() -> Html {

    html!(
        <>
            <CreatePublicationMolecule/>
        </>
    )
}