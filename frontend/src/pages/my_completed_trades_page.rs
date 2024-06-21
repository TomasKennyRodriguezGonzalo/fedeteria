use yew::prelude::*;
use yew_hooks::use_effect_once;
use crate::{components::trade_thumbnail::TradeThumbnail, request_post};
use datos_comunes::{EstadoTrueque, QueryTruequesFiltrados, ResponseTruequesFiltrados};
use yewdux::prelude::*;
use crate::store::UserStore;

#[function_component(MyCompletedTradesPage)]
pub fn my_completed_trades_page() -> Html {

    let (store, _dispatch) = use_store::<UserStore>();
    let dni = store.dni;
    
    let concreted_trades_list_state: UseStateHandle<Vec<usize>> = use_state(|| Vec::new());
    let refused_trades_list_state: UseStateHandle<Vec<usize>> = use_state(|| Vec::new());

    //traigo los trueques concretados y rechazados
    let cloned_dni = dni.clone();
    let cloned_concreted_trades_list_state = concreted_trades_list_state.clone();
    let cloned_refused_trades_list_state = refused_trades_list_state.clone();
    use_effect_once(move || {
        //trueques concretados
        let query = QueryTruequesFiltrados {
            filtro_codigo_ofertante: None,
            filtro_codigo_receptor: None,
            //filtro_ofertante: None,
            //filtro_receptor: cloned_dni,
            filtro_dni_integrantes: cloned_dni,
            filtro_estado: Some(EstadoTrueque::Finalizado),
            filtro_fecha_pactada: None,
            filtro_fecha_trueque: None,
            filtro_id_publicacion: None,
            filtro_sucursal: None,
        };
        request_post("/api/obtener_trueques", query, move |respuesta: ResponseTruequesFiltrados| {
            let trades = respuesta;
            cloned_concreted_trades_list_state.set(trades);
        });

        //trueques rechazados
        let query = QueryTruequesFiltrados {
            filtro_codigo_ofertante: None,
            filtro_codigo_receptor: None,
            //filtro_ofertante: None,
            //filtro_receptor: cloned_dni,
            filtro_dni_integrantes: cloned_dni,
            filtro_estado: Some(EstadoTrueque::Rechazado),
            filtro_fecha_pactada: None,
            filtro_fecha_trueque: None,
            filtro_id_publicacion: None,
            filtro_sucursal: None,
        };
        request_post("/api/obtener_trueques", query, move |respuesta: ResponseTruequesFiltrados| {
            let trades = respuesta;
            cloned_refused_trades_list_state.set(trades);
        });
        || {}
    });

    html!(
        <div class="my-defined-trades-box">
            <div class="title">
                <h1>{"Tus Trueques Concretados"}</h1>
            </div>
            <div class="trueque-grid">
            if !(&*concreted_trades_list_state).is_empty() {
                <ul>
                    {
                        (&*concreted_trades_list_state).iter().enumerate().map(|(_index, id)| {
                            html! {
                                <li><TradeThumbnail id_trade={id}/></li>
                            }
                        }).collect::<Html>()
                    }
                </ul>
            } else{
                <div>{"aun no tienes trueques concretados"}</div>
            }
            </div>
            <div class="title">
                <h1>{"Tus Trueques Rechazados"}</h1>
            </div>
            <div class="trueque-grid">
            if !(&*refused_trades_list_state).is_empty() {
                <ul>
                    {
                        (&*refused_trades_list_state).iter().enumerate().map(|(_index, id)| {
                            html! {
                                <li><TradeThumbnail id_trade={id}/></li>
                            }
                        }).collect::<Html>()
                    }
                </ul>
            } else{
                <div>{"aun no tienes trueques rechazados"}</div>
            }
            </div>
        </div>
    )
}