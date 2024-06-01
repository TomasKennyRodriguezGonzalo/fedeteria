use _Props::id;
use datos_comunes::{QueryObtenerTrueque, ResponseObtenerTrueque, Trueque};
use web_sys::window;
use crate::request_post;
use crate::{router::Route, store::UserStore};
use yew_router::hooks::use_navigator;
use yewdux::use_store;
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use crate::information_store::InformationStore;
use crate::molecules::confirm_prompt_button_molecule::ConfirmPromptButtonMolecule;

#[derive(Properties,PartialEq)]
pub struct Props {
    pub id: usize,
}

#[function_component(TruequeMolecule)]
pub fn trueque_molecule (props : &Props) -> Html {

    let id_trueque = props.id;
    let trueque_state:UseStateHandle<Option<Trueque>> = use_state(||None);
    let cloned_trueque_state = trueque_state.clone();

    use_effect_once(move ||{
        let trueque_state = cloned_trueque_state.clone();
        let query = QueryObtenerTrueque{
            id : id_trueque,  
        };

        request_post("/api/obtener_trueque", query, move |respuesta:ResponseObtenerTrueque|{
            let trueque_state = trueque_state.clone();
            match respuesta {
                Ok(trueque) =>{
                    trueque_state.set(Some(trueque));
                }
                Err(error) =>{
                    log::error!("Error al obtener el trueque {:?}", error);
                }
            }
        });

        ||{}
    });





    html! {
        <>
            if let Some(trueque) = (&*trueque_state){

            }

            <h1>{format!("Trueque número {}", props.id)}</h1>
        </>
    }
}