use std::time::Duration;

use leptos::*;

#[derive(Copy, Clone)]
pub struct TooltipContext {
    pub trigger_ref: NodeRef<html::Button>,
    pub open: RwSignal<bool>,
    pub hide_delay: Duration,
}

impl Default for TooltipContext {
    fn default() -> Self {
        Self {
            trigger_ref: NodeRef::default(),
            open: create_rw_signal(false),
            hide_delay: Duration::from_millis(200),
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
