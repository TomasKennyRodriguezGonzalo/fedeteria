use datos_comunes::QueryPagarPromocionPublicaciones;
use yew::prelude::*;
use yew_router::hooks::use_location;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub query: Option<QueryPagarPromocionPublicaciones>,
}

#[function_component(PayPublicationPromotionMolecule)]
pub fn pay_publication_promotion_molecule (props: &Props) -> Html {
    log::info!("Recibi esto: {:?}", props.query);
    html!(
        <h1>{"Hola"}</h1>
    )
}