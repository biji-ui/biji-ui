use std::sync::atomic::{AtomicUsize, Ordering};

use leptos::{html::Input, prelude::*};

use crate::utils::props::StringProp;

static PIN_INPUT_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub(crate) fn next_pin_input_id() -> String {
    let id = PIN_INPUT_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("biji-pin-{id}")
}

/// Reactive state for a PIN input. Available via [`use_pin_input`](super::root::use_pin_input)
/// or the `let:` binding on [`RootWith`](super::root::RootWith).
///
/// All fields are `Copy`, so it is safe to pass this struct to child components as a prop.
#[derive(Copy, Clone)]
pub struct PinInputState {
    pub values: RwSignal<Vec<String>>,
    pub length: usize,
    pub disabled: bool,
    /// Full PIN string — concatenation of all filled cells.
    pub value: Signal<String>,
    /// `true` when every cell has a character.
    pub is_complete: Signal<bool>,
    pub(crate) cell_refs: StoredValue<Vec<NodeRef<Input>>>,
    pub(crate) placeholder: StoredValue<StringProp>,
    pub(crate) root_id: StoredValue<String>,
    pub(crate) on_complete: Option<Callback<String>>,
}

impl PinInputState {
    pub(crate) fn new(
        values_signal: Option<RwSignal<Vec<String>>>,
        length: usize,
        disabled: bool,
        placeholder: StringProp,
        on_complete: Option<Callback<String>>,
    ) -> Self {
        let cell_refs: Vec<NodeRef<Input>> = (0..length).map(|_| NodeRef::new()).collect();
        let values = values_signal.unwrap_or_else(|| RwSignal::new(vec![String::new(); length]));
        let value = Signal::derive(move || values.with(|v| v.join("")));
        let is_complete =
            Signal::derive(move || values.with(|v| v.iter().all(|s| !s.is_empty())));
        Self {
            values,
            length,
            disabled,
            value,
            is_complete,
            cell_refs: StoredValue::new(cell_refs),
            placeholder: StoredValue::new(placeholder),
            root_id: StoredValue::new(next_pin_input_id()),
            on_complete,
        }
    }

    pub fn set_cell(&self, index: usize, val: String) {
        self.values.update(|v| {
            if index < v.len() {
                v[index] = val;
            }
        });
        if self.is_complete.get() {
            if let Some(cb) = self.on_complete {
                cb.run(self.value.get());
            }
        }
    }

    pub fn focus_cell(&self, index: usize) {
        self.cell_refs.with_value(|refs| {
            if let Some(nr) = refs.get(index) {
                if let Some(el) = nr.get() {
                    let _ = el.focus();
                }
            }
        });
    }
}
