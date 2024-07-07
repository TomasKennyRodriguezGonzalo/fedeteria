use yew::prelude::*;
use yew_router::hooks::use_location;
use datos_comunes::QueryPagarPromocionPublicaciones;
use crate::molecules::pay_publication_promotion_molecule::PayPublicationPromotionMolecule;

#[function_component(PayPublicationPromotionPage)]
pub fn pay_publication_promotion_page () -> Html {
    let location = use_location().unwrap();
    let props = location.query::<QueryPagarPromocionPublicaciones>().unwrap();
    log::info!("RECIBI ESTO: {:?}", props);
    html!(
        <div>
            <PayPublicationPromotionMolecule query={Some(props)}/>
        </div>
    )
}