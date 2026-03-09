use std::time::Duration;

use leptos::{
    html::{Button, Div},
    prelude::*,
};

use crate::utils::positioning::{AvoidCollisions, Positioning};

#[derive(Copy, Clone)]
pub struct PopoverContext {
    pub trigger_ref: NodeRef<Button>,
    pub content_ref: NodeRef<Div>,
    pub open: RwSignal<bool>,
    pub hide_delay: Duration,
    pub positioning: Positioning,
    pub arrow_size: i32,
    pub popover_id: StoredValue<String>,
    pub avoid_collisions: AvoidCollisions,
    pub auto_focus: bool,
    pub(crate) on_open_change: Option<Callback<bool>>,
}

impl Default for PopoverContext {
    fn default() -> Self {
        Self {
            trigger_ref: NodeRef::default(),
            content_ref: NodeRef::default(),
            open: RwSignal::new(false),
            hide_delay: Duration::from_millis(200),
            positioning: Positioning::Bottom,
            arrow_size: 8,
            popover_id: StoredValue::new(String::new()),
            avoid_collisions: AvoidCollisions::Flip,
            auto_focus: true,
            on_open_change: None,
        }
    }
}

impl PopoverContext {
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

    pub fn toggle(&self) {
        if self.open.get() {
            self.close();
        } else {
            self.open();
        }
    }

    pub fn data_state(&self) -> &'static str {
        if self.open.get() { "open" } else { "closed" }
    }
}
