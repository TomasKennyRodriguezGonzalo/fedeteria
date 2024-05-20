use std::borrow::BorrowMut;

use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[derive(Properties,PartialEq)]
pub struct Props {
    #[prop_or(0)]
    pub maxlength : usize,
    #[prop_or_default]
    pub placeholder : String,
    pub name: String,
    #[prop_or_default]
    pub label: String,
    pub tipo: String,
    #[prop_or_default]
    pub on_change: Option<Callback<String>>,
    #[prop_or_default]
    /// El valor Err() NO contiene el error, sino el valor que da error, por si igual lo necesitas
    pub on_change_checked: Option<Callback<Result<String, String>>>,
    #[prop_or_default]
    pub on_change_validity: Option<Callback<bool>>,

}

#[function_component(CheckedInputField)]
pub fn generic_field(props: &Props)-> Html{
    let name = props.name.clone();
    let on_change = props.on_change.clone();
    let on_change_checked = props.on_change_checked.clone();
    let on_change_validity= props.on_change_validity.clone();

    let onchange = Callback::from(move |event:Event|{
        let target = event.target().unwrap();
        let input:HtmlInputElement = target.unchecked_into();
        let input_value = input.value();
        log::info!("Onchange: name=[{}] input_value=[{}] valid:{}", name, input_value, input.check_validity());
        // "Returns true if the element's value has no validity problems; otherwise, returns false."
        if let Some(on_change_validity) = on_change_validity.clone() {
            on_change_validity.emit(input.check_validity());
        }
        if let Some(on_change_checked) = on_change_checked.clone() {
            if input.report_validity() {
                on_change_checked.emit(Ok(input_value.clone()));
            } else {
                on_change_checked.emit(Err(input_value.clone()))
            }
        }
        if let Some(on_change) = on_change.clone() {
            on_change.emit(input_value);
        }
    });

    let id= props.label.to_lowercase().replace(' ', "-");

    html! {
        <>
            <div>
                <label for={id.clone()}>{&props.label}</label>
            </div>
            <div>
                if props.maxlength == 0 {
                    <input type = {props.tipo.clone()} placeholder={props.placeholder.clone()} name = {props.name.clone()} onchange={onchange} />
                } else {
                    <input type = {props.tipo.clone()} placeholder={props.placeholder.clone()} name = {props.name.clone()} maxlength = {props.maxlength.to_string()} onchange={onchange} />
                }
            </div>
        </>
    }

}