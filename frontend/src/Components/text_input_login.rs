use yew::prelude::*;
use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[derive(Properties,PartialEq)]
pub struct Props{
    pub name:String,
    pub handle_on_change:Callback<String>,

}


#[function_component(LogInInputField)]
pub fn log_in_input_field(props: &Props)-> Html{
    let handle_on_change = props.handle_on_change.clone();

    let onchange: Callback<Event> = Callback::from(move |event:Event|{
        let target = event.target().unwrap();
        let input:HtmlInputElement = target.unchecked_into();
        let input_value = input.value();
        handle_on_change.emit(input_value);
    });

    html! {
        <input type = "text" name = {props.name.clone()} onchange={onchange} />
    }

}