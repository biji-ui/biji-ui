use std::time::Duration;

use leptos::{
    html::{Button, Div},
    prelude::*,
};

#[derive(Copy, Clone)]
pub struct DialogState {
    pub open: RwSignal<bool>,
    pub data_state: Signal<&'static str>,
    pub(crate) trigger_ref: NodeRef<Button>,
    pub(crate) root: RwSignal<RootContext>,
    pub(crate) prevent_scroll: bool,
    pub(crate) hide_delay: Duration,
}

impl DialogState {
    pub fn open(&self) {
        self.open.set(true);
    }

    pub fn close(&self) {
        self.open.set(false);
    }

    pub fn toggle(&self) {
        if self.open.get() {
            self.close();
        } else {
            self.open();
        }
    }
}

#[derive(Copy, Clone)]
pub struct RootContext {
    pub close_ref: NodeRef<Button>,
    pub overlay_ref: NodeRef<Div>,
    pub content_ref: NodeRef<Div>,
}

impl Default for RootContext {
    fn default() -> Self {
        Self {
            overlay_ref: NodeRef::default(),
            close_ref: NodeRef::default(),
            content_ref: NodeRef::default(),
        }
    }
}
