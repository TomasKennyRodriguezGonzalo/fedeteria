/*use crate::request_post;
use datos_comunes::{EstadoTrueque, QueryTruequesFiltrados, ResponseTruequesFiltrados, QueryOfertasDePublicacion, ResponseObtenerTrueques, ResponseOfertasDePublicacion, Trueque};
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yew_router::hooks::use_location;
use crate::components::trueque_thumbnail::TruequeThumbnail;

#[function_component(PublicationTradeOffersPage)]
pub fn publication_trade_offers_page() -> Html {
    let location = use_location().unwrap();
    let props = location.query::<QueryOfertasDePublicacion>().unwrap();
    let offers_list_state:UseStateHandle<Vec<usize>> = use_state(|| Vec::new());
    let cloned_offers_list_state = offers_list_state.clone();
    use_effect_once(move ||{
        /*let query = QueryObtenerTrueques{
            estado : EstadoTrueque::Oferta,
            id_publicacion : Some(props.id),
            dni : None,
        };*/
        let query = QueryTruequesFiltrados{
            filtro_codigo_ofertante: None,
            filtro_codigo_receptor: None,
            filtro_ofertante: None,
            filtro_receptor: None,
            filtro_estado: Some(EstadoTrueque::Oferta),
            filtro_fecha: None,
            filtro_id_publicacion: Some(props.id),
            filtro_sucursal: None,
        };
        let offers_list_state = cloned_offers_list_state.clone();
        request_post("/api/obtener_trueques", query, move |respuesta:ResponseTruequesFiltrados|{
            let offers_list_state = offers_list_state.clone();
            let offers = respuesta;
            log::info!("entre al use effect {:?}",offers);
            offers_list_state.set(offers);
        });

        || {}
    });



    let cloned_offers_list_state = offers_list_state.clone();
    html!(
        <>
        <ul> 
        {
            cloned_offers_list_state.iter().enumerate().map(|(_index, id)| {
                html! {
                    <li>
                        <TruequeThumbnail id_trueque={id}/>
                    </li>
                }
            }).collect::<Html>()
        }
    </ul>
        </>
    )
}*/