use yew::prelude::*;
use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[derive(Properties,PartialEq)]
pub struct Props{
    pub name:String,
    pub handle_on_change:Callback<String>,

}


#[function_component(PasswordTextInput)]
pub fn password_text_input(props: &Props)-> Html{
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