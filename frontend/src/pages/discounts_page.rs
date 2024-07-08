use datos_comunes::{Descuento, QueryEliminarDescuento, QueryObtenerDescuentos, ResponseEliminarDescuento, ResponseObtenerDescuentos};
use yew::prelude::*;
use yew_hooks::use_effect_once;
use crate::components::indexed_button::IndexedButton;
use crate::molecules::confirm_prompt_button_molecule::ConfirmPromptButtonMolecule;
use web_sys::window;

use crate::request_post;

#[function_component(DiscountsPage)]
pub fn trueque_page() -> Html {
    let discounts = use_state(||Vec::new());
    let cloned_discounts = discounts.clone();
    use_effect_once(move || {
        let query = QueryObtenerDescuentos{
            nada : true,
        };
        request_post("/api/obtener_descuentos", query, move |respuesta:ResponseObtenerDescuentos|{
            let mut aux = respuesta.descuentos;
            aux.sort_by(|d1, d2| d1.1.vigente.cmp(&d2.1.vigente));
            cloned_discounts.set(aux);
        });

        || {}
    });

    let index_cochino:UseStateHandle<usize> = use_state(|| 0);
    let index_cochino_cloned = index_cochino.clone();

    let eliminar_descuento = Callback::from(move |_|{
        let query = QueryEliminarDescuento{
            index : *index_cochino_cloned,
        };
        request_post("/api/eliminar_descuento", query, move|_:ResponseEliminarDescuento|{

        });
        if let Some(window) = window() {
            window.location().reload().unwrap();
        }
    });
    let confirm_button = use_state(||false);
    let cloned_confirm_button = confirm_button.clone();

    let index_cochino_cloned = index_cochino.clone();

    let show_confirm_button = Callback::from(move |index:usize|{
        cloned_confirm_button.set(true);
        index_cochino_cloned.set(index);
    });

    let cloned_confirm_button = confirm_button.clone();
    let hide_confirm_button = Callback::from(move |_|{
        cloned_confirm_button.set(false);
    });

    html!(
        <ul class="discount-list">
       
        {
            (&*discounts).iter().filter(|d| d.1.vigente).map(|(index,descuento)| {
                html!{
                    <li class="discount-item">
                        <h1> {"Codigo: "}{(descuento.codigo).clone()}</h1>
                        <h1> {"Porcentaje: "}{(descuento.porcentaje * 100.0).clone()}{"%"}</h1>
                        <h1> {"Reintegro maximo: "}{(descuento.reintegro_maximo).clone()}</h1>
                        <h1> {"Fecha de caducidad: "}{(descuento.fecha_vencimiento).clone()}</h1>
                        <h1> {"Nivel minimo: "}{(descuento.nivel_minimo).clone()}</h1>
                        <h1> {"Estado: No Vigente"}</h1>
                        <IndexedButton text="Eliminar" index={index} onclick_event={(show_confirm_button).clone()}/>
                    </li>
                }
            }).collect::<Html>()
        }
        {
            (&*discounts).iter().filter(|d| !d.1.vigente).map(|(index, descuento)| 
                html! {
                    <li class="discount-item log-down">
                        <h1> {"Codigo: "}{(descuento.codigo).clone()}</h1>
                        <h1> {"Porcentaje: "}{(descuento.porcentaje * 100.0).clone()}{"%"}</h1>
                        <h1> {"Reintegro maximo: "}{(descuento.reintegro_maximo).clone()}</h1>
                        <h1> {"Fecha de caducidad: "}{(descuento.fecha_vencimiento).clone()}</h1>
                        <h1> {"Nivel minimo: "}{(descuento.nivel_minimo).clone()}</h1>
                        <h1> {"Estado: No Vigente"}</h1>
                    </li>
                }
            ).collect::<Html>()
        }
            if *confirm_button{
                <ConfirmPromptButtonMolecule text="¿Seguro que quiere eliminar el descuento?" confirm_func={(eliminar_descuento).clone()} reject_func={(hide_confirm_button).clone()} />
            }                               
        </ul>
    )
}