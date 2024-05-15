use yew::prelude::*;

use crate::components::{generic_button::GenericButton, generic_input_field::GenericInputField};

#[function_component(CreateOfficeMolecule)]
pub fn create_office_molecule() -> Html {

    let office_name_state = use_state(|| "".to_string());
    let office_name_state_cloned = office_name_state.clone();

    let office_name_changed = Callback::from(move |office_name : String| {
        office_name_state_cloned.set(office_name);
    });

    let submit_clicked = Callback::from(|()| {
        // Agregar logica para que el backend cree la sucursal
    });

    let onsubmit = Callback::from(|submit_event : SubmitEvent| {
        submit_event.prevent_default();
    });

    html!(
        <div class="create-office-box">
            <form onsubmit={onsubmit}>
                <GenericInputField name="office-name" label="Ingresa el nombre de la sucursal: " tipo="text" handle_on_change={office_name_changed}/>
                <GenericButton text="Cargar Sucursal" onclick_event={submit_clicked}/>
            </form>
        </div>
    )
}