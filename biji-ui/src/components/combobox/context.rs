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
    utils::positioning::{AvoidCollisions, Positioning},
};

#[derive(Copy, Clone)]
pub struct ComboboxContext {
    pub trigger_ref: NodeRef<Button>,
    pub content_ref: NodeRef<Div>,
    pub input_ref: NodeRef<Input>,
    pub open: RwSignal<bool>,
    pub value: RwSignal<Option<String>>,
    /// Display label of the currently selected item, cached at selection time.
    pub selected_label: RwSignal<Option<String>>,
    /// Current search/filter text typed in the Input.
    pub query: RwSignal<String>,
    pub item_focus: RwSignal<Option<usize>>,
    pub items: RwSignal<HashMap<usize, ComboboxItemContext>>,
    pub hide_delay: Duration,
    pub positioning: Positioning,
    pub arrow_size: i32,
    pub combobox_id: StoredValue<String>,
    pub avoid_collisions: AvoidCollisions,
    pub(crate) on_value_change: Option<Callback<String>>,
    pub(crate) next_id: StoredValue<AtomicUsize>,
    /// True when using `InputTrigger` (the input IS the trigger, positioned above the dropdown).
    pub inline_mode: bool,
    /// Suppresses the next focus-triggered open (used after programmatic focus returns to the input
    /// post-selection, to avoid immediately re-opening the dropdown).
    pub suppress_next_open: StoredValue<bool>,
}

impl Default for ComboboxContext {
    fn default() -> Self {
        Self {
            trigger_ref: NodeRef::default(),
            content_ref: NodeRef::default(),
            input_ref: NodeRef::default(),
            open: RwSignal::new(false),
            value: RwSignal::new(None),
            selected_label: RwSignal::new(None),
            query: RwSignal::new(String::new()),
            item_focus: RwSignal::new(None),
            items: RwSignal::new(HashMap::new()),
            hide_delay: Duration::from_millis(200),
            positioning: Positioning::BottomStart,
            arrow_size: 0,
            combobox_id: StoredValue::new(String::new()),
            avoid_collisions: AvoidCollisions::Flip,
            on_value_change: None,
            next_id: StoredValue::new(AtomicUsize::new(0)),
            inline_mode: false,
            suppress_next_open: StoredValue::new(false),
        }
    }
}

impl ComboboxContext {
    pub fn next_index(&self) -> usize {
        self.next_id.with_value(|c| c.fetch_add(1, Ordering::Relaxed))
    }

    pub fn upsert_item(&self, index: usize, item: ComboboxItemContext) {
        self.items.update(|m| {
            *m.entry(index).or_insert(item) = item;
        });
    }

    pub fn remove_item(&self, index: usize) {
        self.items.update(|m| {
            m.remove(&index);
        });
    }

    pub fn select(&self, value: String, label: String) {
        self.value.set(Some(value.clone()));
        self.selected_label.set(Some(label));
        if let Some(cb) = self.on_value_change {
            cb.run(value);
        }
        self.close();
    }

    pub fn open(&self) {
        self.query.set(String::new());
        self.open.set(true);
    }

    pub fn close(&self) {
        self.open.set(false);
        self.item_focus.set(None);
        // query is intentionally NOT reset here — resetting it during close would cause
        // all filtered-out items to flash back before the hide animation completes.
        // It is reset in open() instead, so the list is fresh on the next open.
    }

    pub fn toggle(&self) {
        if self.open.get() {
            self.close();
        } else {
            self.open();
        }
    }

    /// Items that are active (not disabled) and match the current query.
    pub fn visible_items(&self) -> Vec<ComboboxItemContext> {
        let q = self.query.get().to_lowercase();
        let all = filter_active(self.items.get());
        if q.is_empty() {
            return all;
        }
        all.into_iter()
            .filter(|item| item.label.with_value(|l| l.to_lowercase().contains(&q)))
            .collect()
    }
}

impl FilterActiveItems<ComboboxItemContext> for ComboboxContext {
    fn filter_active_items(&self) -> Vec<ComboboxItemContext> {
        filter_active(self.items.get())
    }
}

impl ManageFocus for ComboboxContext {
    fn set_focus(&self, index: Option<usize>) {
        self.item_focus.set(index);
    }

    fn item_in_focus(&self, index: usize) -> bool {
        self.item_focus.get() == Some(index)
    }
}

// Navigation moves through visible (filtered) items only.
impl NavigateItems<ComboboxItemContext> for ComboboxContext {
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
    pub label: StoredValue<String>,
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
