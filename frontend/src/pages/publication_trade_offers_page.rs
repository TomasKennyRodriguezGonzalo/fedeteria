use crate::{molecules::publication_molecule::PublicationMolecule, request_post};
use datos_comunes::{QueryOfertasDePublicacion, ResponseOfertasDePublicacion, Trueque};
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yew_router::hooks::use_location;



#[function_component(PublicationTradeOffersPage)]
pub fn publication_trade_offers_page() -> Html {
    let location = use_location().unwrap();
    let props = location.query::<QueryOfertasDePublicacion>().unwrap();
    let offers_list_state:UseStateHandle<Vec<usize>> = use_state(|| Vec::new());
    let cloned_offers_list_state = offers_list_state.clone();
    use_effect_once(move ||{
        let query = QueryOfertasDePublicacion{
            id : props.id,
        };
        let offers_list_state = cloned_offers_list_state.clone();
        request_post("/api/obtener_ofertas", query, move |respuesta:ResponseOfertasDePublicacion|{
            let offers_list_state = offers_list_state.clone();
            let offers = respuesta.ofertas;
            offers_list_state.set(offers);

        });



        || {}
    });


    html!(
        <>
        </>
    )
}