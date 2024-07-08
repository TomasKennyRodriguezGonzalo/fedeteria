use datos_comunes::{Descuento, QueryGetUserDiscounts, ResponseGetUserDiscounts};
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yew_router::hooks::{use_location, use_navigator};
use yewdux::prelude::*;

use crate::{request_post, store::UserStore};


#[function_component(AllowedDiscountsPage)]
pub fn allowed_discounts_page() -> Html {
    let (store, _dispatch) = use_store::<UserStore>();
    let dni = store.dni;
    let discounts:UseStateHandle<Vec<Descuento>> = use_state(||Vec::new());
    let cloned_discounts = discounts.clone();

    use_effect_once(move || {
        if let Some(dni) = dni {
            let query = QueryGetUserDiscounts { dni };
            request_post("/api/obtener_descuentos_usuario", query, move |respuesta:ResponseGetUserDiscounts|{
                cloned_discounts.set(respuesta.discounts);
            });
        }

        || {}
    });


    html!(
        <ul class="discount-list">
        {
            (&*discounts).iter().map(|descuento| {
                html! {
                    <li class="discount-item">
                        if descuento.vigente{
                            <h1> {"Codigo: "}{(descuento.codigo).clone()}</h1>
                            <h1> {"Porcentaje: "}{(descuento.porcentaje * 100.0).clone()}{"%"}</h1>
                            <h1> {"Reintegro maximo: "}{(descuento.reintegro_maximo).clone()}</h1>
                            <h1> {"Fecha de caducidad: "}{(descuento.fecha_vencimiento).clone()}</h1>
                            }
                    </li>
                }
            }).collect::<Html>()
        }
        </ul>
    )
}