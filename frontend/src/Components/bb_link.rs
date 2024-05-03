use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;


#[derive(PartialEq,Clone,Properties)]
pub struct Props{
    pub text:String,
    pub route:Route,

}


#[function_component(BBLink)]
pub fn bb_link(props:&Props)-> Html{





    html!{
            <Link<Route> to={props.route.clone()}  >{props.text.clone()}</Link<Route>>
    }
}