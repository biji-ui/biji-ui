use std::time::Duration;

use leptos::{
    html::{Button, Div},
    prelude::*,
};

use crate::{
    components::tooltip::singleton,
    utils::positioning::{AvoidCollisions, Positioning},
};

#[derive(Copy, Clone)]
pub struct TooltipState {
    pub open: RwSignal<bool>,
    pub data_state: Signal<&'static str>,
    pub(crate) trigger_ref: NodeRef<Button>,
    pub(crate) content_ref: NodeRef<Div>,
    pub(crate) pointer_inside_trigger: RwSignal<bool>,
    pub(crate) pointer_inside_content: RwSignal<bool>,
    pub(crate) hide_delay: Duration,
    pub(crate) positioning: Positioning,
    pub(crate) arrow_size: i32,
    pub(crate) tooltip_id: StoredValue<String>,
    pub(crate) numeric_id: usize,
    pub(crate) avoid_collisions: AvoidCollisions,
}

impl TooltipState {
    pub(crate) fn open(&self) {
        singleton::activate(self.numeric_id);
        self.open.set(true);
    }

    pub(crate) fn close(&self) {
        singleton::deactivate(self.numeric_id);
        self.open.set(false);
    }
}
