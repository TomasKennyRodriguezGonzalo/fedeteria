use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[derive(Properties,PartialEq)]
pub struct Props{
    pub name:String,
    pub label:String,
    pub tipo:String,
    pub handle_on_change:Callback<String>,

}

#[function_component(GenericInputField)]
pub fn generic_field(props: &Props)-> Html{
    let handle_on_change = props.handle_on_change.clone();
    let name = props.name.clone();

    let onchange: Callback<Event> = Callback::from(move |event:Event|{
        let target = event.target().unwrap();
        let input:HtmlInputElement = target.unchecked_into();
        let input_value = input.value();
        log::info!("Onchange: name=[{}] input_value=[{}]", name, input_value);
        handle_on_change.emit(input_value);
    });

    let id= props.label.to_lowercase().replace(" ", "-");

    html! {
        <>
            <div>
                <label for={id.clone()}>{&props.label}</label>
            </div>
            <div>
                <input type = {props.tipo.clone()} name = {props.name.clone()} onchange={onchange} />
            </div>
        </>
    }

}