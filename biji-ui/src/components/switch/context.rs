use leptos::{html::Button, prelude::*};

/// Reactive state for a switch. Available via [`use_switch`](super::root::use_switch)
/// or the `let:` binding on [`RootWith`](super::root::RootWith).
///
/// All fields are `Copy`, so it is safe to pass this struct to child components as a prop.
#[derive(Copy, Clone)]
pub struct SwitchState {
    pub checked: RwSignal<bool>,
    pub disabled: bool,
    /// `"checked"` or `"unchecked"`, derived from `checked`.
    pub data_state: Signal<&'static str>,
    pub(crate) trigger_ref: NodeRef<Button>,
}

impl SwitchState {
    pub(crate) fn new(
        checked: Option<RwSignal<bool>>,
        default_checked: bool,
        disabled: bool,
    ) -> Self {
        let checked_sig = checked.unwrap_or_else(|| RwSignal::new(default_checked));
        let data_state =
            Signal::derive(move || if checked_sig.get() { "checked" } else { "unchecked" });
        Self { checked: checked_sig, disabled, data_state, trigger_ref: NodeRef::new() }
    }
}
