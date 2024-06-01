use crate::molecules::trueque_molecule::TruequeMolecule;
use yew::prelude::*;

#[derive(Properties,PartialEq)]
pub struct Props {
    pub id : usize
}

#[function_component(TruequePage)]
pub fn trueque_page(props : &Props) -> Html {
    html!(
        <>
            <TruequeMolecule id={(&props).id.clone()}/>
        </>
    )
}