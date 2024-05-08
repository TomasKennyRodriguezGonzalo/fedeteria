use std::vec;

use crate::Components::{generic_input_field::GenericInputField, generic_button::GenericButton};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use web_sys::{FileReader, HtmlInputElement};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use yew::ContextProvider;

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
    
    let image_path = use_state(|| "".to_string());
    let image_path_clone = image_path.clone();
    
    let image_list = use_state(|| Vec::new());
    let image_list_clone = image_list.clone();
    use_effect(move || {
        let image_list_clone = image_list_clone.clone();

        || {}
    });
    
    
    let image_list_clone = image_list.clone();
    let oninput = move |event : InputEvent| {
        event.prevent_default();
        
        let image_path_clone = image_path_clone.clone();
        
        let image_list_clone = image_list_clone.clone();
        
        let data_transfer : HtmlInputElement = event.target_dyn_into().unwrap();
        
        let file_list = data_transfer.files().unwrap();
        
        let file = file_list.get(0).unwrap();
        
        let reader = FileReader::new().unwrap();
        
        let reader_clone = reader.clone();
        let onloaded = Closure::once_into_js(move || {
            let path = reader_clone.result().unwrap().as_string().unwrap();
            image_path_clone.set(path);
            let new_vec = &*image_list_clone;
            let mut mutable_vec = new_vec.clone();
            let image_path_clone = image_path_clone.clone();
            mutable_vec.push(html!(<img src={(&*image_path_clone).to_string()} height="200px" width="300px"/>));
            image_list_clone.set(mutable_vec);
            log::info!("la imagen es: {:?}",image_path_clone.clone())
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
                <div class="submit_button">
                <GenericButton text="Publicar" onclick_event={submit_clicked}/>
                </div>
                </form>
                <div class="image-preview">
                    if !(&*image_list).is_empty() {
                        <h2>{"Aqui se previsualizan tus imágenes:"}</h2>
                        <ul>
                            {(&*image_list).iter().map(|image| html!({(&*image).clone()})).collect::<Html>()}
                        </ul>
                    }
                </div>
            </div>
    )
}