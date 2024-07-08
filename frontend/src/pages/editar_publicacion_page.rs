use std::{cell::RefCell, rc::Rc};

use crate::{components::checked_input_field::CheckedInputField, convenient_request::request_get, information_store::InformationStore, router::Route};
use datos_comunes::ResponsePublicacion;
use serde::Serialize;
use yew_hooks::use_effect_once;
use yew_router::hooks::use_navigator;
use yewdux::use_store;
use reqwasm::http::Request;
use web_sys::{File, FormData, HtmlFormElement, HtmlInputElement};
use yew::{platform::spawn_local, prelude::*, virtual_dom::VNode};
use wasm_bindgen::JsCast;
use crate::store::UserStore;

#[derive(Properties,PartialEq)]
pub struct Props {
    pub id : usize
}

#[derive(Debug, Clone)]
struct DatosImagen {
    vnode: VNode,
    // Si no hay "File" es porque es una imagen ya subida.
    file: Option<File>,
    url: String,
    url_original: Option<String>,
}

impl DatosImagen {
    pub fn new(file: Option<File>, url: String, image_list: UseStateHandle<Rc<RefCell<Vec<DatosImagen>>>>, url_original: Option<String>) -> DatosImagen {

        let on_borrar = {
            let url_c = url.clone();
            Callback::from(move |_| {
                let image_list = image_list.clone();
                let pos = image_list.borrow()
                    .iter().enumerate()
                    .find(|(_, datos)| {
                        datos.url == url_c}
                    )
                    .unwrap().0;
                let datos = image_list.borrow_mut().remove(pos);
                web_sys::Url::revoke_object_url(&datos.url).unwrap();
                image_list.set((*image_list).clone());
            })
        };
        let node = html! {<>
            <img src={url.clone()} height="200px" width="300px" />
            <button onclick={on_borrar}> {"Borrar"} </button>
        </>};
        DatosImagen {
            vnode: node, url, file, url_original
        }
    } 
}
#[function_component(EditarPublicacionPage)]
pub fn editar_publicacion_page(props : &Props) -> Html {
    let id = props.id;

    let (store, _dispatch) = use_store::<UserStore>();
    let (_information_store, information_dispatch) = use_store::<InformationStore>();
    let navigator = use_navigator().unwrap();


    // IMPORTANTE: Para que se actualice tras cambiar los datos internos, hay que hacer image_list.set((*image_list).clone());
    let image_list = use_state(|| Rc::new(RefCell::new(Vec::<DatosImagen>::new())));

    let form_ref = use_node_ref();

    let form_ref_c = form_ref.clone();
    let image_list_c = image_list.clone();
    // Nos traemos los datos de la publicación.
    use_effect_once(move || {
        let image_list = image_list_c;
        let form_ref = form_ref_c;
        let url = format!("/api/datos_publicacion?id={id}");
        request_get(&url, move |respuesta: ResponsePublicacion| {
            let respuesta = respuesta.unwrap();
            let form = form_ref.cast::<HtmlFormElement>().unwrap();
            let elementos = form.elements();
            
            let mut i = 0;
            while let Some(elem) = elementos.item(i) {
                if let Ok(elem) = elem.dyn_into::<HtmlInputElement>() {
                    log::info!("elemento: {}", elem.name());
                    if elem.name() == "Titulo" {
                        elem.set_value(&respuesta.titulo);
                    } else if elem.name() == "Descripción" {
                        elem.set_value(&respuesta.descripcion);
                    }
                }
                i += 1;
            }
            for url_original in respuesta.imagenes {
                let url = format!("/publication_images/{url_original}");
                image_list.borrow_mut().push(DatosImagen::new(None, url, image_list.clone(), Some(url_original)));
            }
            image_list.set((*image_list).clone());
        });

        || {}
    });

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
                let image_list = image_list.clone();

                let result = web_sys::Url::create_object_url_with_blob(&file);
                if let Ok(url) = result {
                    image_list.borrow_mut().push(DatosImagen::new(Some(file), url, image_list.clone(), None));
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

    let on_confirmar = {
        let image_list = image_list.clone();
        let form_ref = form_ref.clone();
        Callback::from(move |event: MouseEvent| {
            log::info!("enviando publicacion!!!");
            event.prevent_default();
            let navigator = navigator.clone();

            let form = form_ref.cast::<HtmlFormElement>().unwrap();

            let form_data = FormData::new_with_form(&form).unwrap();

            form_data.append_with_str("id", &id.to_string()).unwrap();
            form_data.append_with_str("dni", &store.dni.unwrap().to_string()).unwrap();
            for datos in image_list.borrow().iter() {
                if let Some(blob) = &datos.file.clone() {
                    form_data.append_with_blob("blob", blob).unwrap();
                } else if let Some(url_original) = &datos.url_original {
                    form_data.append_with_str("url_original", url_original).unwrap();
                }
            }

            spawn_local(async move {
                let res = Request::post("/api/editar_publicacion")
                    .body(form_data)
                    .send().await;
                let res = res.unwrap();
                let res = res.text().await.unwrap();
                if res == "OK" {
                    navigator.push(&Route::Publication { id });
                }
            });
        })
    };

    html!(
        <div class="create-publication-box">
            <form onsubmit={onsubmit} ref={form_ref}>
                <h1>{"¡Hora de hacer tremendo edit de tu publicación!"}</h1>
                <div class="text-prompts">
                    <CheckedInputField name="Titulo" maxlength={40} label="Ingrese el titulo de la publicación" tipo="text"/>
                    <CheckedInputField name="Descripción" label="Ingrese una descripción para la publicación" tipo="text"/>
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
                                        <li class="image-item">{image.vnode.clone()}</li>
                                    </>)
                                }).collect::<Html>()
                            }
                        </ul>
                    }
                </div>
                <div class="submit_button">
                    // if !(title_state.is_empty()) && !(description_state.is_empty()) && !(image_list.borrow().is_empty()) {
                    <button onclick={on_confirmar}>{"Confirmar"}</button>
                    // } else { 
                    //     <button class="disabled-dyn-element">{"Confirmar"}</button>
                    // }
                </div>
            </form>
        </div>
)
}