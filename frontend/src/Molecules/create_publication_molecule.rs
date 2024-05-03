use crate::Components::{generic_input_field::GenericInputField, generic_button::GenericButton};
use web_sys::console::log;
use yew::prelude::*;

#[function_component(CreatePublicationMolecule)]
pub fn create_publication_molecule() -> Html {

    let title_state = use_state(|| "No title yet".to_owned());
    let cloned_title_state = title_state.clone();
    let title_changed = Callback::from(move |title:String|{
            cloned_title_state.set(title.clone());
    });

    let description_state = use_state(|| "No description yet".to_owned());
    let cloned_description_state = description_state.clone();
    let description_changed = Callback::from(move |description:String|{
            cloned_description_state.set(description.clone());
    });

    let post_clicked = Callback::from(move |()| {
        ()
    });

    html!(
        <form class="create-publication">
            <h1>{"Crea tu publicación!"}</h1>
            <div class="text-prompts">
                <GenericInputField name="Titulo" label="Ingrese el titulo de la publicación" tipo="text" handle_on_change={title_changed}/>
                <GenericInputField name="Descripción" label="Ingrese una descripción para la publicación" tipo="text" handle_on_change={description_changed}/>
            </div>
            <div class="image-prompts">
                {"PROMPTS DE IMAGENES"}
            </div>
            <div class="image-preview">
                {"PREVIEW DE IMAGENES"}
            </div>
            <div class="submit_button">
                <GenericButton text="Publicar" onclick_event={post_clicked}/>
            </div>
        </form>
    )
}