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
    pub user:String,
    pub token:String,
}


/* 
impl Store for UserStore {
    fn new(_cx: &yewdux::Context) -> Self {
        Self {
            user: Default::default(),
            token: Default::default(),
        }
    }

    fn should_notify(&self, old: &Self) -> bool {
        // When this returns true, all components are notified and consequently re-render.
        self != old
    }

}
*/

struct LogListener;
impl Listener for LogListener {
    type Store = UserStore;

    fn on_change(&mut self, _cx: &Context, state: Rc<Self::Store>) {
        log!(Level::Info, "Username changed to {}", state.user);
    }
}