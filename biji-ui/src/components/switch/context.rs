use leptos::{html::Button, prelude::*};

#[derive(Copy, Clone)]
pub struct SwitchContext {
    pub checked: RwSignal<bool>,
    pub disabled: bool,
    pub trigger_ref: NodeRef<Button>,
}

impl SwitchContext {
    pub fn data_state(&self) -> &'static str {
        if self.checked.get() { "checked" } else { "unchecked" }
    }
}
