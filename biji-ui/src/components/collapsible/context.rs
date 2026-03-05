use leptos::{html::Button, prelude::*};

#[derive(Copy, Clone)]
pub struct CollapsibleContext {
    pub open: RwSignal<bool>,
    pub disabled: bool,
    pub trigger_ref: NodeRef<Button>,
}

impl CollapsibleContext {
    pub fn data_state(&self) -> &'static str {
        if self.open.get() {
            "open"
        } else {
            "closed"
        }
    }

    pub fn toggle(&self) {
        if !self.disabled {
            self.open.update(|o| *o = !*o);
        }
    }
}
