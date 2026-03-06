use leptos::{html::Button, prelude::*};

#[derive(Copy, Clone, PartialEq)]
pub enum CheckedState {
    Checked,
    Unchecked,
    Indeterminate,
}

impl CheckedState {
    pub fn as_str(&self) -> &'static str {
        match self {
            CheckedState::Checked => "checked",
            CheckedState::Unchecked => "unchecked",
            CheckedState::Indeterminate => "indeterminate",
        }
    }

    pub fn aria_checked(&self) -> &'static str {
        match self {
            CheckedState::Checked => "true",
            CheckedState::Unchecked => "false",
            CheckedState::Indeterminate => "mixed",
        }
    }
}

#[derive(Copy, Clone)]
pub struct CheckboxContext {
    pub checked: RwSignal<CheckedState>,
    pub disabled: bool,
    pub trigger_ref: NodeRef<Button>,
}
