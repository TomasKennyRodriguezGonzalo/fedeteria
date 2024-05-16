use serde::{Deserialize, Serialize};
use yewdux::prelude::*;
use std::rc::Rc;
use yewdux::{
    log::{log, Level},
    Context,
};


#[derive(Debug, Serialize, Deserialize, Clone,Default, PartialEq,Store)]
#[store(storage = "local", listener(LogListener))]
pub struct InformationStore{
    pub messages : Vec<String>,
}

struct LogListener;
impl Listener for LogListener {
    type Store = InformationStore;
    
    fn on_change(&mut self, _cx: &Context, state: Rc<Self::Store>) {
        if !(state.messages.is_empty()) {
            log!(Level::Info, "Information message changed to {:?}", state.messages);
        } else{
            log!(Level::Info, "Information message is empty");
        }
    }
}
