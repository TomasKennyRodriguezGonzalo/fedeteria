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
        <>
        {
            (&*discounts).iter().enumerate().map(|(index, descuento)| {
                html! {
                    <>
                    if descuento.vigente{
                        <div> {"codigo: "}{(descuento.codigo).clone()}</div>
                        <div> {"porcentaje: "}{(descuento.porcentaje * 100.0).clone()}{"%"}</div>
                        <div> {"reintegro maximo: "}{(descuento.reintegro_maximo).clone()}</div>
                        <div> {"fecha de caducidad: "}{(descuento.fecha_vencimiento).clone()}</div>
                        }
                    </>
                }
            }).collect::<Html>()
        }
        </>
    )
}