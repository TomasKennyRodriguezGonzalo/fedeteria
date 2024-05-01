use yew::prelude::*;
use gloo::console::log;


#[derive(Properties,PartialEq)]
pub struct Props{
    pub text : String,
}


#[function_component(LogInButton)]
pub fn log_in_button(props: &Props)-> Html{
    let mut button_clicked = false;

    let on_click = Callback::from(|mouse_event:MouseEvent| {
        let target=mouse_event.target();
});

    html! {
        <button onclick = {on_click} >
            {props.text.clone()}
        </button>
    }

}