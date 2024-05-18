use yew::prelude::*;


#[derive(Properties,PartialEq)]
pub struct Props{
    pub text : String,
    pub confirm_func : Callback<MouseEvent>,
    pub reject_func : Callback<MouseEvent>,
}


#[function_component(ConfirmPromptButtonMolecule)]
pub fn log_in_button(props: &Props)-> Html{
 
    let confirm_func = (props.confirm_func).clone();
    let reject_func = (props.reject_func).clone();

    html! {
            <>
            <button onclick={move |e: MouseEvent|  confirm_func.emit(e)}>  {"confirmar"} </button>
            <button onclick={move |e: MouseEvent|  reject_func.emit(e)}>  {"rechazar"} </button>
            </>

    }

}