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
pub struct SelectState {
    pub(crate) trigger_ref: NodeRef<Button>,
    pub(crate) content_ref: NodeRef<Div>,
    pub open: RwSignal<bool>,
    pub value: RwSignal<Option<String>>,
    /// The display label of the currently selected item. Cached at selection
    /// time so it survives the dropdown unmounting.
    pub selected_label: RwSignal<Option<String>>,
    pub data_state: Signal<&'static str>,
    pub(crate) item_focus: RwSignal<Option<usize>>,
    pub(crate) items: RwSignal<HashMap<usize, SelectItemContext>>,
    pub(crate) hide_delay: Duration,
    pub(crate) positioning: Positioning,
    pub(crate) arrow_size: i32,
    pub(crate) select_id: StoredValue<String>,
    pub(crate) avoid_collisions: AvoidCollisions,
    pub(crate) next_id: StoredValue<AtomicUsize>,
}

impl SelectState {
    pub(crate) fn next_index(&self) -> usize {
        self.next_id
            .with_value(|c| c.fetch_add(1, Ordering::Relaxed))
    }

    pub(crate) fn upsert_item(&self, index: usize, item: SelectItemContext) {
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
        self.open.set(false);
    }

    pub(crate) fn open(&self) {
        self.open.set(true);
    }

    pub(crate) fn close(&self) {
        self.open.set(false);
        self.item_focus.set(None);
    }

    pub(crate) fn toggle(&self) {
        if self.open.get() {
            self.close();
        } else {
            self.open();
        }
    }
}

impl FilterActiveItems<SelectItemContext> for SelectState {
    fn filter_active_items(&self) -> Vec<SelectItemContext> {
        filter_active(self.items.get())
    }
}

impl ManageFocus for SelectState {
    fn set_focus(&self, index: Option<usize>) {
        self.item_focus.set(index);
    }

    fn item_in_focus(&self, index: usize) -> bool {
        self.item_focus.get() == Some(index)
    }
}

impl NavigateItems<SelectItemContext> for SelectState {
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
