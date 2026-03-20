use leptos::{html::Button, prelude::*};

/// Reactive state for a collapsible. Available via [`use_collapsible`](super::root::use_collapsible)
/// or the `let:` binding on [`RootWith`](super::root::RootWith).
///
/// All fields are `Copy`, so it is safe to pass this struct to child components as a prop.
#[derive(Copy, Clone)]
pub struct CollapsibleState {
    pub open: RwSignal<bool>,
    pub disabled: bool,
    /// `"open"` or `"closed"`, derived from `open`.
    pub data_state: Signal<&'static str>,
    pub(crate) trigger_ref: NodeRef<Button>,
}

impl CollapsibleState {
    pub(crate) fn new(open: Option<RwSignal<bool>>, disabled: bool) -> Self {
        let open = open.unwrap_or_else(|| RwSignal::new(false));
        let data_state = Signal::derive(move || if open.get() { "open" } else { "closed" });
        Self {
            open,
            disabled,
            data_state,
            trigger_ref: NodeRef::new(),
        }
    }

    pub fn toggle(&self) {
        if !self.disabled {
            self.open.update(|o| *o = !*o);
        }
    }
}
