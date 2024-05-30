use yew::prelude::*;

use crate::molecules::publication_selector_molecule::PublicationSelectorMolecule;

#[function_component(PublicationSelectorPage)]
pub fn publication_selector_page () -> Html {
    html! {
        <PublicationSelectorMolecule/>
    }
}