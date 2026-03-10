use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

use leptos::{
    html::{Button, Div},
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
pub struct SelectContext {
    pub trigger_ref: NodeRef<Button>,
    pub content_ref: NodeRef<Div>,
    pub open: RwSignal<bool>,
    pub value: RwSignal<Option<String>>,
    /// The display label of the currently selected item. Cached at selection
    /// time so it survives the dropdown unmounting.
    pub selected_label: RwSignal<Option<String>>,
    pub item_focus: RwSignal<Option<usize>>,
    pub items: RwSignal<HashMap<usize, SelectItemContext>>,
    pub hide_delay: Duration,
    pub positioning: Positioning,
    pub arrow_size: i32,
    pub select_id: StoredValue<String>,
    pub avoid_collisions: AvoidCollisions,
    pub(crate) on_value_change: Option<Callback<String>>,
    pub(crate) next_id: StoredValue<AtomicUsize>,
}

impl Default for SelectContext {
    fn default() -> Self {
        Self {
            trigger_ref: NodeRef::default(),
            content_ref: NodeRef::default(),
            open: RwSignal::new(false),
            value: RwSignal::new(None),
            selected_label: RwSignal::new(None),
            item_focus: RwSignal::new(None),
            items: RwSignal::new(HashMap::new()),
            hide_delay: Duration::from_millis(200),
            positioning: Positioning::BottomStart,
            arrow_size: 0,
            select_id: StoredValue::new(String::new()),
            avoid_collisions: AvoidCollisions::Flip,
            on_value_change: None,
            next_id: StoredValue::new(AtomicUsize::new(0)),
        }
    }
}

impl SelectContext {
    pub fn next_index(&self) -> usize {
        self.next_id
            .with_value(|c| c.fetch_add(1, Ordering::Relaxed))
    }

    pub fn upsert_item(&self, index: usize, item: SelectItemContext) {
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
        self.open.set(false);
    }

    pub fn open(&self) {
        self.open.set(true);
    }

    pub fn close(&self) {
        self.open.set(false);
        self.item_focus.set(None);
    }

    pub fn toggle(&self) {
        if self.open.get() {
            self.close();
        } else {
            self.open();
        }
    }
}

impl FilterActiveItems<SelectItemContext> for SelectContext {
    fn filter_active_items(&self) -> Vec<SelectItemContext> {
        filter_active(self.items.get())
    }
}

impl ManageFocus for SelectContext {
    fn set_focus(&self, index: Option<usize>) {
        self.item_focus.set(index);
    }

    fn item_in_focus(&self, index: usize) -> bool {
        self.item_focus.get() == Some(index)
    }
}

impl NavigateItems<SelectItemContext> for SelectContext {
    fn navigate_first_item(&self) -> Option<SelectItemContext> {
        self.filter_active_items().into_iter().next()
    }

    fn navigate_last_item(&self) -> Option<SelectItemContext> {
        self.filter_active_items().into_iter().last()
    }

    fn navigate_next_item(&self) -> Option<SelectItemContext> {
        let items = self.filter_active_items();
        next_item(items, self.item_focus.get(), true)
    }

    fn navigate_previous_item(&self) -> Option<SelectItemContext> {
        let items = self.filter_active_items();
        previous_item(items, self.item_focus.get(), true)
    }
}

#[derive(Copy, Clone)]
pub struct SelectItemContext {
    pub index: usize,
    pub value: StoredValue<String>,
    /// Display label shown in the trigger when this item is selected.
    /// Defaults to `value` if not explicitly provided.
    pub label: StoredValue<String>,
    pub disabled: bool,
    pub item_ref: NodeRef<Div>,
}

impl GetIndex<usize> for SelectItemContext {
    fn get_index(&self) -> usize {
        self.index
    }
}

impl IsActive for SelectItemContext {
    fn is_active(&self) -> bool {
        !self.disabled
    }
}

impl Focus for SelectItemContext {
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
