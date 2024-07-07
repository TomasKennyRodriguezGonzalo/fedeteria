use datos_comunes::{Descuento, QueryEliminarDescuento, QueryObtenerDescuentos, ResponseEliminarDescuento, ResponseObtenerDescuentos};
use yew::prelude::*;
use yew_hooks::use_effect_once;
use crate::components::indexed_button::IndexedButton;
use crate::molecules::confirm_prompt_button_molecule::ConfirmPromptButtonMolecule;
use web_sys::window;

use crate::request_post;

#[function_component(DiscountsPage)]
pub fn trueque_page() -> Html {
    let discounts:UseStateHandle<Vec<Descuento>> = use_state(||Vec::new());
    let cloned_discounts = discounts.clone();
    use_effect_once(move || {
        let query = QueryObtenerDescuentos{
            nada : true,
        };
        request_post("/api/obtener_descuentos", query, move |respuesta:ResponseObtenerDescuentos|{
            cloned_discounts.set(respuesta.descuentos);
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
        <>
        {
            (&*discounts).iter().enumerate().map(|(index, descuento)| {
                html! {
                    <>
                        <div> {"codigo: "}{(descuento.codigo).clone()}</div>
                        <div> {"porcentaje: "}{(descuento.porcentaje).clone()}</div>
                        <div> {"reintegro maximo: "}{(descuento.reintegro_maximo).clone()}</div>
                        <div> {"nivel minimo: "}{(descuento.nivel_minimo).clone()}</div>
                        <div> {"fecha de caducidad: "}{(descuento.fecha_vencimiento).clone()}</div>
                        if descuento.vigente{
                            <div> {"estado: Vigente"}</div>
                            <IndexedButton text="Cancelar Descuento" index={index} onclick_event={(show_confirm_button).clone()}/>
                        } else{
                            <div> {"estado: No Vigente"}</div>
                        }
                        if *confirm_button{
                            <ConfirmPromptButtonMolecule text="¿Seguro que quiere eliminar el descuento?" confirm_func={(eliminar_descuento).clone()} reject_func={(hide_confirm_button).clone()} />
                        }
                    </>
                }
            }).collect::<Html>()
        }
        </>
    )
}