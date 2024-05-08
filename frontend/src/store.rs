use serde::{Deserialize, Serialize};
use yew::{prelude::*, functional::*};
use yewdux::prelude::*;
use yewdux_functional::*;
use std::rc::Rc;
use yewdux::{
    log::{log, Level},
    prelude::*, Context,
};


#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq,Store)]
#[store(storage = "local", listener(LogListener))]
pub struct UserStore{
    pub dni:Option<u64>,
    pub nombre:String,
    pub token:String,
}




struct LogListener;
impl Listener for LogListener {
    type Store = UserStore;

    fn on_change(&mut self, _cx: &Context, state: Rc<Self::Store>) {
        if state.dni.is_some(){
            log!(Level::Info, "Dni changed to {}", state.dni.unwrap());
        } else{
            log!(Level::Info, "Dni change faliure");
        }
    }
}