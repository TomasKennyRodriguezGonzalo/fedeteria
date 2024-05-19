use std::{cell::RefCell, rc::Rc};

use crate::{components::checked_input_field::CheckedInputField, information_store::InformationStore, router::Route};
use yew_router::hooks::use_navigator;
use yewdux::use_store;
use reqwasm::http::Request;
use web_sys::{File, FormData, HtmlFormElement, HtmlInputElement};
use yew::{platform::spawn_local, prelude::*, virtual_dom::VNode};
use wasm_bindgen::JsCast;
use crate::store::UserStore;

#[function_component(CreatePublicationMolecule)]
pub fn create_publication_molecule() -> Html {
    let (store, _dispatch) = use_store::<UserStore>();
    let (information_store, information_dispatch) = use_store::<InformationStore>();
    let navigator = use_navigator().unwrap();

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

    // IMPORTANTE: Para que se actualice tras cambiar los datos internos, hay que hacer image_list.set((*image_list).clone());
    let image_list = use_state(|| Rc::new(RefCell::new(Vec::<(VNode, File, String)>::new())));

    let on_image_selected = {
        let image_list = image_list.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_dyn_into().unwrap();
            let file_list: web_sys::FileList = input.files().unwrap();
            for i in 0..file_list.length() {
                if image_list.borrow().len() >= 5 {break; }
                let file = file_list.get(i).unwrap();

                let nombre = file.name();
                log::info!("Agregando un archivo: {}", nombre);
                let image_list_c = image_list.clone();
                let file_c = file.clone();
                let on_borrar = {
                    Callback::from(move |_| {
                        let image_list = image_list_c.clone();
                        log::info!("Borrando archivo: {}", nombre);
                        let pos = image_list.borrow()
                            .iter().enumerate()
                            .find(|(_, (_, f, _))| f == &file_c)
                            .unwrap().0;
                        let (_, _, url) = image_list.borrow_mut().remove(pos);
                        web_sys::Url::revoke_object_url(&url).unwrap();
                        image_list.set((*image_list).clone());
                    })
                };

                let result = web_sys::Url::create_object_url_with_blob(&file);
                if let Ok(url) = result {
                    let node = html! {<>
                        <img src={url.clone()} height="200px" width="300px" />
                        <button onclick={on_borrar}> {"Borrar"} </button>
                    </>};
                    image_list.borrow_mut().push((node, file, url));
                } else {
                    panic!();
                }
            }
            image_list.set((*image_list).clone());
            // Esto hace que el ultimo archivo seleccionado no figure en la parte de arriba.
            input.set_value("");
        })
    };

    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default()
    });
    let form_ref = use_node_ref();

    let on_confirmar = {
        let image_list = image_list.clone();
        let form_ref = form_ref.clone();
        Callback::from(move |event: MouseEvent| {
            log::info!("enviando publicacion!!!");
            event.prevent_default();
            let navigator = navigator.clone();
            let information_dispatch = information_dispatch.clone();

            let form = form_ref.cast::<HtmlFormElement>().unwrap();

            let form_data = FormData::new_with_form(&form).unwrap();

            form_data.append_with_str("dni", &store.dni.unwrap().to_string()).unwrap();
            for (_, blob, _) in image_list.borrow().iter() {
                form_data.append_with_blob("", blob).unwrap();
            }

            spawn_local(async move {
            
                let res = Request::post("/api/crear_publicacion")
                    .body(form_data)
                    .send().await;
                let res = res.unwrap();
                let res = res.text().await.unwrap();
                if res == "OK" {

                    navigator.push(&Route::Home);
                    information_dispatch.reduce_mut(|store| store.messages.push("Publicación creada con éxito.".to_string()))
                }
            });
        })
    };

    html!(
        <div class="create-publication-box">
            <form onsubmit={onsubmit} ref={form_ref}>
                <h1>{"Crea tu publicación!"}</h1>
                <div class="text-prompts">
                    <CheckedInputField name="Titulo" label="Ingrese el titulo de la publicación" tipo="text" on_change={title_changed}/>
                    <CheckedInputField name="Descripción" label="Ingrese una descripción para la publicación" tipo="text" on_change={description_changed}/>
                </div>
                if image_list.borrow().len() < 5 {
                    <div class="image-prompts">
                        <input oninput={on_image_selected} type="file" id="file" name="publication_img" accept="image/*" multiple=true/>
                    </div>
                } 
                <div class="image-preview">
                    if !image_list.borrow().is_empty() {
                        <h2>{format!("Aqui se previsualizan tus imágenes {}/5:", image_list.borrow().len())}</h2>
                        <ul class="image-list">
                            {
                                image_list.borrow().iter().map(|image| {
                                    html!( <>
                                        <li class="image-item">{image.0.clone()}</li>
                                    </>)
                                }).collect::<Html>()
                            }
                        </ul>
                    }
                </div>
                <div class="submit_button">
                    if !(title_state.is_empty()) && !(description_state.is_empty()) && !(image_list.borrow().is_empty()) {
                        <button onclick={on_confirmar}>{"Confirmar"}</button>
                    } else { 
                        <button class="disabled-dyn-element">{"Confirmar"}</button>
                    }
                </div>
            </form>
        </div>
)
}