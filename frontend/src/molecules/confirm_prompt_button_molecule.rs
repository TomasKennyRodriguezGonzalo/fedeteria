use yew::prelude::*;


#[derive(Properties,PartialEq)]
pub struct Props{
    pub text : String,
    pub confirm_func : Callback<MouseEvent>,
    pub reject_func : Callback<MouseEvent>,
}


#[function_component(ConfirmPromptButtonMolecule)]
pub fn confirm_prompt_button(props: &Props)-> Html{
 
    let confirm_func = (props.confirm_func).clone();
    let reject_func = (props.reject_func).clone();

    html! {
            <div class="confirm-prompt">
                <div class="solid-background">
                    <div>
                    <h1 class="title">{props.text.clone()}</h1>
                    </div>
                    <div>
                    <button onclick={move |e: MouseEvent|  confirm_func.emit(e)}>  {"confirmar"} </button>
                    <button style="background-color : red" onclick={move |e: MouseEvent|  reject_func.emit(e)}>  {"rechazar"} </button>
                    </div>
                </div>
            </div>

    }

}