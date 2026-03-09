use std::time::Duration;

use leptos::{
    html::{Button, Div},
    prelude::*,
};

use crate::{components::tooltip::singleton, utils::positioning::{AvoidCollisions, Positioning}};

#[derive(Copy, Clone)]
pub struct TooltipContext {
    pub trigger_ref: NodeRef<Button>,
    pub content_ref: NodeRef<Div>,
    pub open: RwSignal<bool>,
    pub pointer_inside_trigger: RwSignal<bool>,
    pub pointer_inside_content: RwSignal<bool>,
    pub hide_delay: Duration,
    pub positioning: Positioning,
    pub arrow_size: i32,
    pub tooltip_id: StoredValue<String>,
    /// Numeric ID used by the singleton registry to enforce one-at-a-time.
    pub numeric_id: usize,
    pub avoid_collisions: AvoidCollisions,
}

impl Default for TooltipContext {
    fn default() -> Self {
        Self {
            trigger_ref: NodeRef::default(),
            content_ref: NodeRef::default(),
            open: RwSignal::new(false),
            pointer_inside_trigger: RwSignal::new(false),
            pointer_inside_content: RwSignal::new(false),
            hide_delay: Duration::from_millis(200),
            positioning: Positioning::default(),
            arrow_size: 8,
            tooltip_id: StoredValue::new(String::new()),
            numeric_id: 0,
            avoid_collisions: AvoidCollisions::Flip,
        }
    }
}

impl TooltipContext {
    pub fn open(&self) {
        singleton::activate(self.numeric_id);
        self.open.set(true);
    }

    pub fn close(&self) {
        singleton::deactivate(self.numeric_id);
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
