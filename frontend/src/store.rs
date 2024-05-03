use serde::{Deserialize, Serialize};
use yew::{prelude::*, functional::*};
use yewdux::prelude::*;
use yewdux_functional::*;


#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct UserStore{
    pub user:String,
    pub token:String,
}

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


