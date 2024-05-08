use serde::{Deserialize, Serialize};
use yewdux::prelude::*;
use std::rc::Rc;
use yewdux::{
    log::{log, Level},
    Context,
};


#[derive(Debug, Serialize, Deserialize, Clone,Default, PartialEq,Store)]
#[store(storage = "local", listener(LogListener))]
pub struct UserStore{
    pub dni:Option<u64>,
    pub nombre:String,
    pub token:String,
    pub login_fail:bool,
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