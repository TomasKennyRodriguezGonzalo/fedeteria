use yew::prelude::*;

use crate::molecules::promote_publication_from_profile_molecule::PromotePublicationFromProfileMolecule;

#[function_component(PromotePublicationFromProfilePage)]
pub fn promote_publication_from_profile_page () -> Html {
    html!(
        <PromotePublicationFromProfileMolecule/>
    )
}