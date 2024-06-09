use yew::prelude::*;
use yew_hooks::use_effect_once;
use crate::{components::trade_thumbnail::TradeThumbnail, request_post};
use datos_comunes::{EstadoTrueque, QueryTruequesFiltrados, ResponseTruequesFiltrados};
use yewdux::prelude::*;
use crate::store::UserStore;

#[function_component(MyPendingTradesPage)]
pub fn my_pending_trades_page() -> Html {

    let (store, _dispatch) = use_store::<UserStore>();
    let dni = store.dni;
    
    let pending_trueques_list_state: UseStateHandle<Vec<usize>> = use_state(|| Vec::new());

    let cloned_dni = dni.clone();
    let cloned_pending_trueques_list_state = pending_trueques_list_state.clone();
    use_effect_once(move || {
        let query = QueryTruequesFiltrados {
            filtro_codigo_ofertante: None,
            filtro_codigo_receptor: None,
            //filtro_ofertante: None,
            //filtro_receptor: cloned_dni,
            filtro_dni_integrantes: cloned_dni,
            filtro_estado: Some(EstadoTrueque::Pendiente),
            filtro_fecha: None,
            filtro_id_publicacion: None,
            filtro_sucursal: None,
        };
        request_post("/api/obtener_trueques", query, move |respuesta: ResponseTruequesFiltrados| {
            let trueques = respuesta;
            log::info!("ids de todos los trueques: {trueques:?}");
            cloned_pending_trueques_list_state.set(trueques);
        });
        || {}
    });

    html!(
        <div class="my-pending-trueques-box">
            <div class="title">
                <h1>{"Tus Trueques Pendientes"}</h1>
            </div>
            <div class="trueque-grid">
            if !(&*pending_trueques_list_state).is_empty() {
                <ul>
                    {
                        (&*pending_trueques_list_state).iter().enumerate().map(|(_index, id)| {
                            html! {
                                <li><TradeThumbnail id_trade={id}/></li>
                            }
                        }).collect::<Html>()
                    }
                </ul>
            } else{
                <div>{"aun no tienes trueques pendientes"}</div>
            }
            </div>
        </div>
    )
}