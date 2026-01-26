use std::sync::{Arc, Mutex};

use dioxus::prelude::*;

#[derive(Clone)]
pub struct SharedSender<T>(Arc<Mutex<Option<UnboundedSender<T>>>>);

impl<T> SharedSender<T> {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(None)))
    }

    pub fn set(&self, sender: Option<UnboundedSender<T>>) {
        *self.0.lock().unwrap() = sender;
    }

    pub(super) fn get(&self) -> Option<UnboundedSender<T>> {
        self.0.lock().unwrap().clone()
    }
}
