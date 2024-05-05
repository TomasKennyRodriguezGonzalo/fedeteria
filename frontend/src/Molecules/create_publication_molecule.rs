use std::borrow::Borrow;
use std::clone;
use std::ops::Deref;

use crate::Components::{generic_input_field::GenericInputField, generic_button::GenericButton};
use yew::html::IntoPropValue;
use yew::prelude::*;
use web_sys::FileReader;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

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

    let submit_clicked = Callback::from(move |()| {
        ()
    });

    let onsubmit = Callback::from(move |event:SubmitEvent|{
        event.prevent_default();
    });

    let image_path: UseStateHandle<Option<String>>  = use_state(|| None);
    
    let image_path_clone = image_path.clone();
        
    let oninput = move |Event : InputEvent| {
        Event.prevent_default();

        let image_path_clone = image_path_clone.clone();
        
        let data_transfer = Event.data_transfer().unwrap();
            
        let file_list = data_transfer.files().unwrap();

        let file = file_list.get(0).unwrap();

        let reader = FileReader::new().unwrap();
        
        let reader_clone = reader.clone();
        
        let onloaded = Closure::once_into_js(move || {
            let path = reader_clone.result().unwrap().as_string();
            image_path_clone.set(path);
        });
        
        reader.set_onload(Some(onloaded.as_ref().unchecked_ref()));
        
        reader.read_as_data_url(&file).unwrap();
    };
    

    html!(
        <div class="create-publication">
            <form {onsubmit}>
                <h1>{"Crea tu publicación!"}</h1>
                <div class="text-prompts">
                    <GenericInputField name="Titulo" label="Ingrese el titulo de la publicación" tipo="text" handle_on_change={title_changed}/>
                    <GenericInputField name="Descripción" label="Ingrese una descripción para la publicación" tipo="text" handle_on_change={description_changed}/>
                </div>
                <div class="image-prompts">
                    <input oninput={oninput} type="file" id="file" name="publication_img" multiple={true}/>
                </div>
                <div class="image-preview">
                    if image_path.is_some() {
                        <h2>{"Aqui se previsualizan tus imágenes:"}</h2>
                        <p><img src={image_path.as_ref().unwrap().clone()}/></p>
                    }
                </div>
                <div class="submit_button">
                    <GenericButton text="Publicar" onclick_event={submit_clicked}/>
                </div>
            </form>
        </div>
    )
}