use std::time::Duration;

use leptos::{
    html::{Button, Div},
    prelude::*,
};

#[derive(Copy, Clone)]
pub struct AlertDialogState {
    pub open: RwSignal<bool>,
    pub data_state: Signal<&'static str>,
    pub(crate) trigger_ref: NodeRef<Button>,
    pub(crate) content_ref: NodeRef<Div>,
    pub(crate) cancel_ref: NodeRef<Button>,
    pub(crate) prevent_scroll: bool,
    pub(crate) hide_delay: Duration,
    pub(crate) title_id: StoredValue<String>,
    pub(crate) desc_id: StoredValue<String>,
    pub(crate) on_open_change: Option<Callback<bool>>,
}

impl AlertDialogState {
    pub(crate) fn open(&self) {
        self.open.set(true);
        if let Some(cb) = self.on_open_change {
            cb.run(true);
        }
    }

    pub(crate) fn close(&self) {
        self.open.set(false);
        if let Some(cb) = self.on_open_change {
            cb.run(false);
        }
    }
}
