use crate::molecules::publication_molecule::PublicationMolecule;
use yew::prelude::*;

#[derive(Properties,PartialEq)]
pub struct Props {
    pub id : usize
}

#[function_component(PublicationPage)]
pub fn publication_page(props : &Props) -> Html {
    html!(
        <>
            <PublicationMolecule id={props.id}/>
        </>
    )
}