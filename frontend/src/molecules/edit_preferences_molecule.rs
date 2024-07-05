use datos_comunes::{QueryObtenerPreferencias, ResponseObtenerPreferencias, QueryActualizarPreferencias, ResponseActualizarPreferencias};
use gloo_net::http::Response;
use web_sys::window;
use crate::components::{checked_input_field::CheckedInputField, generic_button::GenericButton};
use crate::molecules::{confirm_prompt_button_molecule::ConfirmPromptButtonMolecule};
use crate::request_post;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yewdux::use_store;

use crate::store::UserStore;

#[function_component(EditPreferencesMolecule)]
pub fn edit_preferences_molecule() -> Html {
    let (store, _dispatch) = use_store::<UserStore>();
    let dni = store.dni.unwrap();

    let preferences_state: UseStateHandle<(Option<String>, Option<String>)> = use_state(|| (None, None));
    let new_preference_a_state = use_state(|| "".to_string());
    let new_preference_b_state = use_state(|| "".to_string());

    let update_confirmation_state = use_state(|| false);
    
    let preference_a_delete_confirmation_state = use_state(|| false);
    let preference_b_delete_confirmation_state = use_state(|| false);

    let cloned_preferences_state = preferences_state.clone();
    use_effect_once(move || {
        let cloned_preferences_state = cloned_preferences_state.clone();
        request_post("/api/obtener_preferencias", QueryObtenerPreferencias{dni}, move |response: ResponseObtenerPreferencias| {
            cloned_preferences_state.set(response.preferencias);
        });

        || {}
    });

    let cloned_new_preference_a_state = new_preference_a_state.clone();
    let cloned_new_preference_b_state = new_preference_b_state.clone();
    let update_preferences = Callback::from(move |_event| {
        let preferencias = (Some((*cloned_new_preference_a_state).clone()), Some((*cloned_new_preference_b_state).clone()));
        request_post("/api/actualizar_preferencias", QueryActualizarPreferencias{dni, preferencias}, move |_response: ResponseActualizarPreferencias| {});
        if let Some(window) = window() {
            window.location().reload().unwrap();
        }
    });

    let delete_preference_a = Callback::from(move |_event| {
        let preferencias = (None, Some("".to_string()));
        request_post("/api/actualizar_preferencias", QueryActualizarPreferencias{dni, preferencias}, move |_response: ResponseActualizarPreferencias| {});
        if let Some(window) = window() {
            window.location().reload().unwrap();
        }
    });

    let delete_preference_b = Callback::from(move |_event| {
        let preferencias = (Some("".to_string()), None);
        request_post("/api/actualizar_preferencias", QueryActualizarPreferencias{dni, preferencias}, move |_response: ResponseActualizarPreferencias| {});
        if let Some(window) = window() {
            window.location().reload().unwrap();
        }
    });

    let cloned_update_confirmation_state = update_confirmation_state.clone();
    let show_update_confirmation = Callback::from(move |_event| {
        cloned_update_confirmation_state.set(true)
    });

    let cloned_update_confirmation_state = update_confirmation_state.clone();
    let hide_update_confirmation = Callback::from(move |_event| {
        cloned_update_confirmation_state.set(false)
    });

    let cloned_preference_a_delete_confirmation_state = preference_a_delete_confirmation_state.clone();
    let show_preference_a_delete_confirmation = Callback::from(move |_event| {
        cloned_preference_a_delete_confirmation_state.set(true)
    });

    let cloned_preference_a_delete_confirmation_state = preference_a_delete_confirmation_state.clone();
    let hide_preference_a_delete_confirmation = Callback::from(move |_event| {
        cloned_preference_a_delete_confirmation_state.set(false)
    });

    let cloned_preference_b_delete_confirmation_state = preference_b_delete_confirmation_state.clone();
    let show_preference_b_delete_confirmation = Callback::from(move |_event| {
        cloned_preference_b_delete_confirmation_state.set(true)
    });

    let cloned_preference_b_delete_confirmation_state = preference_b_delete_confirmation_state.clone();
    let hide_preference_b_delete_confirmation = Callback::from(move |_event| {
        cloned_preference_b_delete_confirmation_state.set(false)
    });

    let cloned_new_preference_a_state = new_preference_a_state.clone();
    let on_change_preference_a = Callback::from(move |value| {
        cloned_new_preference_a_state.set(value);
    });

    let cloned_new_preference_b_state = new_preference_b_state.clone();
    let on_change_preference_b = Callback::from(move |value| {
        cloned_new_preference_b_state.set(value);
    });

    html! {
        <div class="preferences-box">
            <h1 class="title">{"Preferencias"}</h1>
            <div class="info">
                <ul>
                    <h1>{"Preferencia A"}</h1>        
                    <li>
                        if let Some(preference_a) = preferences_state.0.clone() {
                                <h1 class="preference-value">{preference_a}</h1>
                                <GenericButton text="X" onclick_event={show_preference_a_delete_confirmation}/>
                        } else {
                            <h1 class="preference-value">{"No definiste esta preferencia."}</h1>
                        }
                    </li>
                    <h1>{"Preferencia B"}</h1>                        
                    <li>
                        if let Some(preference_b) = preferences_state.1.clone() {
                                <h1 class="preference-value">{preference_b}</h1>
                                <GenericButton text="X" onclick_event={show_preference_b_delete_confirmation}/>
                        } else {
                            <h1 class="preference-value">{"No definiste esta preferencia."}</h1>
                        }
                    </li>
                </ul>
            </div>
            <div class="edit-prompts">
                <h1>{"Editar Preferencias"}</h1>
                <div class="edit-inputs">
                    <CheckedInputField name="preferencia-a" placeholder="preferencia-a" tipo="text" on_change={on_change_preference_a}/>
                    <CheckedInputField name="preferencia-b" placeholder="preferencia-b" tipo="text" on_change={on_change_preference_b}/>
                </div>
                    <GenericButton text="Guardar" onclick_event={show_update_confirmation}/>
            </div>
            if *update_confirmation_state {
                <ConfirmPromptButtonMolecule text="¿Confirma la edición de sus preferencias?" confirm_func={update_preferences} reject_func={hide_update_confirmation} />
            }
            if *preference_a_delete_confirmation_state {
                <ConfirmPromptButtonMolecule text="¿Confirma la eliminación de de la preferencia a?" confirm_func={delete_preference_a} reject_func={hide_preference_a_delete_confirmation} />
            }
            if *preference_b_delete_confirmation_state {
                <ConfirmPromptButtonMolecule text="¿Confirma la eliminación de de la preferencia b?" confirm_func={delete_preference_b} reject_func={hide_preference_b_delete_confirmation} />
            }
        </div>
    }
}