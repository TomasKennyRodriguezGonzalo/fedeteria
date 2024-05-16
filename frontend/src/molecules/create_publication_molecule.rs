use crate::components::generic_input_field::GenericInputField;
use reqwasm::http::Request;
use web_sys::{FormData, HtmlFormElement, HtmlImageElement, HtmlInputElement};
use yew::{platform::spawn_local, prelude::*};
use wasm_bindgen::JsCast;

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

    let image_list = use_state(Vec::new);

    let on_image_selected = {
        let image_list = image_list.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_dyn_into().unwrap();
            let file_list: web_sys::FileList = input.files().unwrap();

            let file = file_list.get(0).unwrap();

            let result = web_sys::Url::create_object_url_with_blob(&file);
            if let Ok(url) = result {
                image_list.set({
                    let mut list = (*image_list).clone();

                    let node = html! {<img src={url} height="200px" width="300px" />};
                    list.push((node, file));
                    list
                });
            }
            // Esto hace que el ultimo archivo seleccionado no figure en la parte de arriba.
            input.set_value("");
        })
    };

    let delete_last_image = {
        let image_list = image_list.clone();
        Callback::from(move |_| {
            let mut list = (*image_list).clone();
            let last = list.pop();
            image_list.set(list);
            if let Some(last) = last {
                if let yew::virtual_dom::VNode::VTag(t) = last.0 {
                    let img = t.node_ref.cast::<HtmlImageElement>().unwrap();
                    web_sys::Url::revoke_object_url(&img.src()).unwrap();
                } else {
                    panic!()
                }
            }
        })
    };

    let onsubmit = {
        
        let image_list = image_list.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let target = event.target();
            let form = target.and_then(|t| t.dyn_into::<HtmlFormElement>().ok()).unwrap();

            let form_data = FormData::new_with_form(&form).unwrap();

            for (_, blob) in image_list.iter() {
                form_data.append_with_blob("", blob).unwrap();
            }
            // HtmlFormElement::submit(&self)
            // // let req: Multipart;

            spawn_local(async move {
            
                let res = Request::post("/api/crear_publicacion")
                    // .header("Content-Type", "multipart/form-data")
                    .body(form_data)
                    .send().await;
                let res = res.unwrap();
                let res = res.text().await.unwrap();
                assert_eq!(res, "HOLO");
            });
        })
    };

    html!(
        <div class="create-publication-box">
            <form {onsubmit} enctype="multipart/form-data" action="/api/crear_publicacion" method="POST">
                <h1>{"Crea tu publicación!"}</h1>
                <div class="text-prompts">
                    <GenericInputField name="Titulo" label="Ingrese el titulo de la publicación" tipo="text" handle_on_change={title_changed}/>
                    <GenericInputField name="Descripción" label="Ingrese una descripción para la publicación" tipo="text" handle_on_change={description_changed}/>
                </div>
                if image_list.len() < 5 {
                    <div class="image-prompts">
                        <input oninput={on_image_selected} type="file" id="file" name="publication_img" accept="image/*"/>
                    </div>
                } 
                <div class="image-preview">
                    if !image_list.is_empty() {
                        <h2>{format!("Aqui se previsualizan tus imágenes {}/5:", image_list.len())}</h2>
                        <ul class="image-list">
                            {
                                image_list.iter().map(|image| {
                                    html!( <>
                                        <li class="image-item">{image.0.clone()}</li>
                                    </>)
                                }).collect::<Html>()
                            }
                        </ul>
                        <button type="button" onclick={delete_last_image}>{"Eliminar última imagen"}</button>
                    }
                </div>
                <div class="submit_button">
                    if !(title_state.is_empty()) && !(description_state.is_empty()) && !(image_list.is_empty()) {
                        <input type="submit" value="Confirmar"/>
                    } else { 
                        <button class="disabled-dyn-element">{"Confirmar"}</button>
                    }
                </div>
            </form>
        </div>
)
}