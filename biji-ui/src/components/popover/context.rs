use std::time::Duration;

use leptos::{
    html::{Button, Div},
    prelude::*,
};

use crate::utils::positioning::{AvoidCollisions, Positioning};

#[derive(Copy, Clone)]
pub struct PopoverState {
    pub open: RwSignal<bool>,
    pub data_state: Signal<&'static str>,
    pub(crate) trigger_ref: NodeRef<Button>,
    pub(crate) content_ref: NodeRef<Div>,
    pub(crate) hide_delay: Duration,
    pub(crate) positioning: Positioning,
    pub(crate) arrow_size: i32,
    pub(crate) popover_id: StoredValue<String>,
    pub(crate) avoid_collisions: AvoidCollisions,
    pub(crate) auto_focus: bool,
}

impl PopoverState {
    pub(crate) fn open(&self) {
        self.open.set(true);
    }

    pub(crate) fn close(&self) {
        self.open.set(false);
    }

    pub(crate) fn toggle(&self) {
        if self.open.get() {
            self.close();
        } else {
            self.open();
        }
    }
}
