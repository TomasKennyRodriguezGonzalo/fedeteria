use crate::molecules::publication_molecule::PublicationMolecule;
use datos_comunes::Trueque;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yew_router::hooks::use_location;



#[function_component(PublicationTradeOffersPage)]
pub fn publication_trade_offers_page() -> Html {
    let location = use_location().unwrap();
    let props = location.query::<usize>().unwrap();

    let offers_list_state:UseStateHandle<Vec<Trueque>> = use_state(|| Vec::new());
    use_effect_once(move ||{








        || {}
    });


    html!(
        <>
        </>
    )
}