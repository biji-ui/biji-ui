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

/// Reactive state for a checkbox. Available via [`use_checkbox`](super::root::use_checkbox)
/// or the `let:` binding on [`RootWith`](super::root::RootWith).
///
/// All fields are `Copy`, so it is safe to pass this struct to child components as a prop.
#[derive(Copy, Clone)]
pub struct CheckboxState {
    pub checked: RwSignal<CheckedState>,
    pub disabled: bool,
    /// `"checked"` | `"unchecked"` | `"indeterminate"`, derived from `checked`.
    pub data_state: Signal<&'static str>,
    pub(crate) trigger_ref: NodeRef<Button>,
}

impl CheckboxState {
    pub(crate) fn new(
        checked: Option<RwSignal<CheckedState>>,
        default_checked: bool,
        indeterminate: bool,
        disabled: bool,
    ) -> Self {
        let checked_sig = checked.unwrap_or_else(|| {
            let initial = if indeterminate {
                CheckedState::Indeterminate
            } else if default_checked {
                CheckedState::Checked
            } else {
                CheckedState::Unchecked
            };
            RwSignal::new(initial)
        });
        let data_state = Signal::derive(move || checked_sig.get().as_str());
        Self { checked: checked_sig, disabled, data_state, trigger_ref: NodeRef::new() }
    }
}
