use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[derive(Properties,PartialEq)]
pub struct Props{
    #[prop_or_default]
    pub placeholder : String,
    pub dni:String,
    #[prop_or_default]
    pub label:String,
    pub tipo:String,
    pub handle_on_change:Callback<String>,

}

#[function_component(DniInputField)]
pub fn generic_field(props: &Props)-> Html{
    let handle_on_change = props.handle_on_change.clone();
    let name = props.dni.clone();

    let onchange: Callback<Event> = Callback::from(move |event:Event|{
        let target = event.target().unwrap();
        let input:HtmlInputElement = target.unchecked_into();
        let input_value = input.value();
        log::info!("Onchange: name=[{}] input_value=[{}]", name, input_value);
        handle_on_change.emit(input_value);
    });

    let id= props.label.to_lowercase().replace(' ', "-");

    html! {
        <>
            <div>
                <label for={id.clone()}>{&props.label}</label>
            </div>
            <div>
                <input placeholder={props.placeholder.clone()} type = {props.tipo.clone()} min="0" name = {props.dni.clone()} onchange={onchange} />
            </div>
        </>
    }

}