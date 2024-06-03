use crate::{components::trade_thumbnail::TradeThumbnail, request_post};
use datos_comunes::{EstadoTrueque, QueryTruequesFiltrados, ResponseTruequesFiltrados};
use yew::prelude::*;
use yew_hooks::use_effect_once;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub query: Option<QueryTruequesFiltrados>,
}

#[function_component(TradeGridMolecule)]
pub fn trade_grid_molecule(props: &Props) -> Html {
    let trade_list_state = use_state(|| None);
    let trade_list_state_cloned = trade_list_state.clone();
    let props_cloned = props.query.clone();
    use_effect_once(move || {
        // obtengo todos los trueques
        let props_cloned = props_cloned.clone();
        //por defecto muestra solo los concretados, para usarlo en el futuro en la pagina principal
        let mut query = QueryTruequesFiltrados
            {   
                filtro_id_publicacion: None,
                filtro_codigo_ofertante: None,
                filtro_codigo_receptor: None,
                //filtro_ofertante: None,
                //filtro_receptor: None,
                filtro_dni_integrantes: None,
                filtro_estado: Some(EstadoTrueque::Finalizado),
                filtro_fecha: None,
                filtro_sucursal: None,
            };
        if let Some(query_options) = props_cloned {
            query = query_options;
            request_post("/api/obtener_trueques", query, move |respuesta: ResponseTruequesFiltrados| {
                let trades = respuesta;
                log::info!("ids de todos los trades: {trades:?}");
                trade_list_state_cloned.set(Some(trades));
            });
            || {}
        }
        else {
            request_post("/api/obtener_trueques", query, move |respuesta: ResponseTruequesFiltrados| {
                let trades = respuesta;
                let trades = trades.iter().rev().take(10).cloned().collect::<Vec<usize>>();
                log::info!("ids de todos los trades: {trades:?}");
                trade_list_state_cloned.set(Some(trades));
            });
            || {}
        }
    });

    html!{
        <div class="trueque-grid">
            if trade_list_state.is_some() {
                <ul>
                    {
                        (trade_list_state).as_ref().unwrap().iter().map(|id| {
                            html! {
                                <li><TradeThumbnail id_trade={id}/></li>
                            }
                        }).collect::<Html>()
                    }
                </ul>
            }
            else {
                <h2>{"No se han realizado trueques"}</h2>
            }
        </div>
    }
}