use yew::prelude::*;


#[derive(Properties,PartialEq)]
pub struct Props{
    pub text : String,
    pub onclick_event : Callback<()>,
}


#[function_component(GenericButton)]
pub fn log_in_button(props: &Props)-> Html{
 
    let button_clicked = props.onclick_event.clone();

    let on_click = Callback::from(move |_| {
        button_clicked.emit(())
    });

    html! {
        <button onclick = {on_click} >
            {props.text.clone()}
        </button>
    }

}