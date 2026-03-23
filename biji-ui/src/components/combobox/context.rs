use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

use leptos::{
    html::{Button, Div, Input},
    prelude::*,
};
use wasm_bindgen::JsCast;

use crate::{
    items::{
        FilterActiveItems, Focus, GetIndex, IsActive, ManageFocus, NavigateItems, filter_active,
        next_item, previous_item,
    },
    utils::{positioning::{AvoidCollisions, Positioning}, props::StringProp},
};

#[derive(Copy, Clone)]
pub struct ComboboxState {
    pub(crate) trigger_ref: NodeRef<Button>,
    pub(crate) content_ref: NodeRef<Div>,
    pub(crate) input_ref: NodeRef<Input>,
    pub open: RwSignal<bool>,
    pub value: RwSignal<Option<String>>,
    /// Display label of the currently selected item, cached at selection time.
    pub selected_label: RwSignal<Option<String>>,
    /// Current search/filter text typed in the Input.
    pub query: RwSignal<String>,
    pub data_state: Signal<&'static str>,
    pub(crate) item_focus: RwSignal<Option<usize>>,
    pub(crate) items: RwSignal<HashMap<usize, ComboboxItemContext>>,
    pub(crate) hide_delay: Duration,
    pub(crate) positioning: Positioning,
    pub(crate) arrow_size: i32,
    pub(crate) combobox_id: StoredValue<String>,
    pub(crate) avoid_collisions: AvoidCollisions,
    pub(crate) next_id: StoredValue<AtomicUsize>,
    /// True when using `InputTrigger` (the input IS the trigger, positioned above the dropdown).
    pub(crate) inline_mode: bool,
    /// Suppresses the next focus-triggered open (used after programmatic focus returns to the input
    /// post-selection, to avoid immediately re-opening the dropdown).
    pub(crate) suppress_next_open: StoredValue<bool>,
}

impl ComboboxState {
    pub(crate) fn next_index(&self) -> usize {
        self.next_id
            .with_value(|c| c.fetch_add(1, Ordering::Relaxed))
    }

    pub(crate) fn upsert_item(&self, index: usize, item: ComboboxItemContext) {
        self.items.update(|m| {
            *m.entry(index).or_insert(item) = item;
        });
    }

    pub(crate) fn remove_item(&self, index: usize) {
        self.items.update(|m| {
            m.remove(&index);
        });
    }

    pub(crate) fn select(&self, value: String, label: String) {
        self.value.set(Some(value));
        self.selected_label.set(Some(label));
        self.close();
    }

    pub(crate) fn open(&self) {
        self.query.set(String::new());
        self.open.set(true);
    }

    pub(crate) fn close(&self) {
        self.open.set(false);
        self.item_focus.set(None);
        // query is intentionally NOT reset here — resetting it during close would cause
        // all filtered-out items to flash back before the hide animation completes.
        // It is reset in open() instead, so the list is fresh on the next open.
    }

    pub(crate) fn toggle(&self) {
        if self.open.get() {
            self.close();
        } else {
            self.open();
        }
    }

    /// Items that are active (not disabled) and match the current query.
    pub(crate) fn visible_items(&self) -> Vec<ComboboxItemContext> {
        let q = self.query.get().to_lowercase();
        let all = filter_active(self.items.get());
        if q.is_empty() {
            return all;
        }
        all.into_iter()
            .filter(|item| item.label.with_value(|l| l.get().to_lowercase().contains(&q)))
            .collect()
    }
}

impl FilterActiveItems<ComboboxItemContext> for ComboboxState {
    fn filter_active_items(&self) -> Vec<ComboboxItemContext> {
        filter_active(self.items.get())
    }
}

impl ManageFocus for ComboboxState {
    fn set_focus(&self, index: Option<usize>) {
        self.item_focus.set(index);
    }

    fn item_in_focus(&self, index: usize) -> bool {
        self.item_focus.get() == Some(index)
    }
}

// Navigation moves through visible (filtered) items only.
impl NavigateItems<ComboboxItemContext> for ComboboxState {
    fn navigate_first_item(&self) -> Option<ComboboxItemContext> {
        self.visible_items().into_iter().next()
    }

    fn navigate_last_item(&self) -> Option<ComboboxItemContext> {
        self.visible_items().into_iter().last()
    }

    fn navigate_next_item(&self) -> Option<ComboboxItemContext> {
        let items = self.visible_items();
        next_item(items, self.item_focus.get(), true)
    }

    fn navigate_previous_item(&self) -> Option<ComboboxItemContext> {
        let items = self.visible_items();
        previous_item(items, self.item_focus.get(), true)
    }
}

#[derive(Copy, Clone)]
pub struct ComboboxItemContext {
    pub index: usize,
    pub value: StoredValue<String>,
    pub label: StoredValue<StringProp>,
    pub disabled: bool,
    pub item_ref: NodeRef<Div>,
}

impl GetIndex<usize> for ComboboxItemContext {
    fn get_index(&self) -> usize {
        self.index
    }
}

impl IsActive for ComboboxItemContext {
    fn is_active(&self) -> bool {
        !self.disabled
    }
}

impl Focus for ComboboxItemContext {
    fn focus(&self) -> bool {
        let Some(el) = self.item_ref.get() else {
            return false;
        };
        let el_node: &web_sys::Node = el.as_ref();
        if let Some(html_el) = el_node.dyn_ref::<web_sys::HtmlElement>() {
            let _ = html_el.focus();
            return true;
        }
        false
    }
}
