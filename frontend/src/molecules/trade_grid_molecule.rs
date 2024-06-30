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
                filtro_fecha_pactada: None,
                filtro_fecha_trueque: None,
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
            if let Some(trade_list) = &*trade_list_state {
                if trade_list.is_empty() {
                    {
                        if let Some(query) = props.query.clone() {
                            if let Some(estado) = query.filtro_estado {
                                match estado {
                                    EstadoTrueque::Oferta => {
                                        html! {
                                            <h1>{"No se encontraron ofertas!"}</h1>
                                        }
                                    },
                                    EstadoTrueque::Definido | EstadoTrueque::Pendiente | EstadoTrueque::Finalizado | EstadoTrueque::Rechazado | EstadoTrueque::Cancelado => {
                                        html! {
                                            <h1>{"No se encontraron trueques!"}</h1>
                                        }
                                    }
                                }
                            } else {
                                html! {
                                    <h1>{"No se encontraron resultados!"}</h1>
                                }
                            }
                        } else {
                            html! {
                                <h1>{"No existen trueques concretados aun!"}</h1>
                            }
                        }
                    }
                } else {
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
            }
            else {
                <h2>{"No se han realizado trueques"}</h2>
            }
        </div>
    }
}