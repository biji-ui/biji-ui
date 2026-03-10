use std::sync::atomic::{AtomicUsize, Ordering};

use leptos::{html::Input, prelude::*};

static PIN_INPUT_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub(crate) fn next_pin_input_id() -> String {
    let id = PIN_INPUT_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("biji-pin-{id}")
}

#[derive(Copy, Clone)]
pub struct PinInputContext {
    pub values: RwSignal<Vec<String>>,
    pub length: usize,
    pub cell_refs: StoredValue<Vec<NodeRef<Input>>>,
    pub disabled: bool,
    pub placeholder: StoredValue<String>,
    /// Unique root ID used to generate stable `id`/`name` attributes for each cell.
    pub root_id: StoredValue<String>,
    pub(crate) on_complete: Option<Callback<String>>,
    pub(crate) on_change: Option<Callback<String>>,
    pub(crate) next_id: StoredValue<AtomicUsize>,
}

impl PinInputContext {
    pub fn next_index(&self) -> usize {
        self.next_id.with_value(|c| c.fetch_add(1, Ordering::Relaxed))
    }

    pub fn current_value(&self) -> String {
        self.values.with(|v| v.join(""))
    }

    pub fn is_complete(&self) -> bool {
        self.values.with(|v| v.iter().all(|s| !s.is_empty()))
    }

    pub fn set_cell(&self, index: usize, value: String) {
        self.values.update(|v| {
            if index < v.len() {
                v[index] = value;
            }
        });
        let full = self.current_value();
        if let Some(cb) = self.on_change {
            cb.run(full.clone());
        }
        if self.is_complete() {
            if let Some(cb) = self.on_complete {
                cb.run(full);
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
