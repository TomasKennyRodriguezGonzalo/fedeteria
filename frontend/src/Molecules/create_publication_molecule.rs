use crate::Components::{generic_button::GenericButton, generic_input_field::GenericInputField};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{FileReader, HtmlInputElement};
use yew::prelude::*;

#[function_component(CreatePublicationMolecule)]
pub fn create_publication_molecule() -> Html {
    let title_state = use_state(|| "".to_owned());
    let title_changed = {
        let title_state = title_state.clone();
        Callback::from(move |title: String| title_state.set(title))
    };

    let description_state = use_state(|| "".to_owned());
    let description_changed = {
        let description_state = description_state.clone();
        Callback::from(move |description: String| description_state.set(description))
    };

    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        log::info!("Form submitted!")
    });

    let image_path = use_state(|| None);
    let image_list = use_state(Vec::new);

    let oninput = {
        let image_path = image_path.clone();
        let image_list = image_list.clone();
        Callback::from(move |event: InputEvent| {
            event.prevent_default();
            let input: HtmlInputElement = event.target_dyn_into().unwrap();
            let file_list = input.files().unwrap();
            input.set_disabled(true);

            let file = file_list.get(0).unwrap();

            let reader = FileReader::new().unwrap();

            let onload = {
                let image_path = image_path.clone();
                let image_list = image_list.clone();
                let reader = reader.clone(); // Clona el FileReader si es posible (FileReader no implementa Clone, ver nota abajo)
                Closure::once_into_js(Box::new(move || {
                    if let Ok(result) = reader.result() {
                        if let Some(url) = result.as_string() {
                            image_path.set(Some(url.clone()));
                            image_list.set({
                                let mut list = (*image_list).clone();
                                list.push(html! { <img src={url} height="200px" width="300px" /> });
                                list
                            });
                        }
                    }
                    input.set_disabled(false);
                }) as Box<dyn FnMut()>)
            };

            reader.set_onload(Some(onload.as_ref().unchecked_ref()));
            reader.read_as_data_url(&file).unwrap();
        })
    };

    let delete_last_image = {
        let image_path = image_path.clone();
        let image_list = image_list.clone();
        Callback::from(move |event: MouseEvent| {
            image_list.set({
                let mut list = (*image_list).clone();
                list.pop();
                list
            });
            log::info!("{:?}", (&*image_list))
        })
    };

    html!(
        <div class="create-publication-box">
            <form {onsubmit}>
                <h1>{"Crea tu publicación!"}</h1>
                <div class="text-prompts">
                    <GenericInputField name="Titulo" label="Ingrese el titulo de la publicación" tipo="text" handle_on_change={title_changed}/>
                    <GenericInputField name="Descripción" label="Ingrese una descripción para la publicación" tipo="text" handle_on_change={description_changed}/>
                </div>
                <div class="image-prompts">
                    <input oninput={oninput} type="file" id="file" name="publication_img" accept="image/*"/>
                </div>
                <div class="image-preview">
                    if !(&*image_list).is_empty() {
                        <h2>{"Aqui se previsualizan tus imágenes:"}</h2>
                        <ul class="image-list">
                            {(&*image_list).iter().map(|image| html!(<li class="image-item">{(&*image).clone()}</li>)).collect::<Html>()}
                        </ul>
                        <button type="button" onclick={delete_last_image}>{"Eliminar última imagen"}</button>
                    }
                </div>
                <div class="submit_button">
                    if !((&*title_state).is_empty()) && !((&*description_state).is_empty()) && !((&*image_list).is_empty()) {
                        <input type="submit" value="Confirmar"/>
                    } else { 
                        <button class="disabled-dyn-element">{"Confirmar"}</button>
                    }
                </div>
            </form>
        </div>
)
}