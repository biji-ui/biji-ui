use std::time::Duration;

use leptos::{
    html::{Button, Div},
    prelude::*,
};

#[derive(Copy, Clone)]
pub struct AlertDialogContext {
    pub open: RwSignal<bool>,
    pub trigger_ref: NodeRef<Button>,
    pub content_ref: NodeRef<Div>,
    pub cancel_ref: NodeRef<Button>,
    pub prevent_scroll: bool,
    pub hide_delay: Duration,
    pub title_id: StoredValue<String>,
    pub desc_id: StoredValue<String>,
    pub(crate) on_open_change: Option<Callback<bool>>,
}

impl Default for AlertDialogContext {
    fn default() -> Self {
        Self {
            open: RwSignal::new(false),
            trigger_ref: NodeRef::default(),
            content_ref: NodeRef::default(),
            cancel_ref: NodeRef::default(),
            prevent_scroll: true,
            hide_delay: Duration::from_millis(200),
            title_id: StoredValue::new(String::new()),
            desc_id: StoredValue::new(String::new()),
            on_open_change: None,
        }
    }
}

impl AlertDialogContext {
    pub fn open(&self) {
        self.open.set(true);
        if let Some(cb) = self.on_open_change {
            cb.run(true);
        }
    }

    pub fn close(&self) {
        self.open.set(false);
        if let Some(cb) = self.on_open_change {
            cb.run(false);
        }
    }

    pub fn data_state(&self) -> &'static str {
        if self.open.get() { "open" } else { "closed" }
    }
}
