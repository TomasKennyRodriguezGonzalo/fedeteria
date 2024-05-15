use crate::molecules::create_publication_molecule::CreatePublicationMolecule;
use yew::prelude::*;

#[function_component(CreatePublicationPage)]
pub fn create_publication_page() -> Html {

    html!(
        <>
            <CreatePublicationMolecule/>
        </>
    )
}