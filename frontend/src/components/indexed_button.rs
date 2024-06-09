use yew::prelude::*;


#[derive(Properties,PartialEq)]
pub struct Props{
    pub text : String,
    pub index : usize,
    pub onclick_event : Callback<usize>,
    #[prop_or_default]
    pub disabled : bool,
}


#[function_component(IndexedButton)]
pub fn indexed_button(props: &Props)-> Html{
    let button_clicked = props.onclick_event.clone();
   // let button_clicked = Callback::clone(&props.onclick_event.clone());
    let index_cloned = (&props.index).clone();

    let on_click = Callback::from( move |_| {
        let index_cloned = index_cloned.clone();
        button_clicked.emit(index_cloned.clone())
    });

    html! {
        if props.disabled {
            <button onclick = {on_click} class="disabled-dyn-element">
                {props.text.clone()}
            </button>
        } else {
            <button onclick = {on_click} >
                {props.text.clone()}
            </button>
        }
    }

}