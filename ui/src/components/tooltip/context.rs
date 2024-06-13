use std::time::Duration;

use leptos::*;

use super::tooltip::Positioning;

#[derive(Copy, Clone)]
pub struct TooltipContext {
    pub trigger_ref: NodeRef<html::Button>,
    pub content_ref: NodeRef<html::Div>,
    pub open: RwSignal<bool>,
    pub pointer_inside_trigger: RwSignal<bool>,
    pub pointer_inside_content: RwSignal<bool>,
    pub hide_delay: Duration,
    pub positioning: Positioning,
    pub arrow_size: i32,
}

impl Default for TooltipContext {
    fn default() -> Self {
        Self {
            trigger_ref: NodeRef::default(),
            content_ref: NodeRef::default(),
            open: create_rw_signal(false),
            pointer_inside_trigger: create_rw_signal(false),
            pointer_inside_content: create_rw_signal(false),
            hide_delay: Duration::from_millis(200),
            positioning: Positioning::default(),
            arrow_size: 8,
        }
    }
}

impl TooltipContext {
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
