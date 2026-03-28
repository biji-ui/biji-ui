use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

use leptos::{html::Button, prelude::*};

use crate::items::{
    FilterActiveItems, Focus, GetIndex, IsActive, ManageFocus, NavigateItems, filter_active,
    next_item, previous_item,
};

/// Reactive state for a radio group. Available via [`use_radio_group`](super::root::use_radio_group)
/// or the `let:` binding on [`RootWith`](super::root::RootWith).
///
/// All fields are `Copy`, so it is safe to pass this struct to child components as a prop.
#[derive(Copy, Clone)]
pub struct RadioGroupState {
    pub value: RwSignal<Option<String>>,
    pub disabled: bool,
    pub(crate) item_focus: RwSignal<Option<usize>>,
    pub(crate) items: RwSignal<HashMap<usize, RadioItemContext>>,
    pub(crate) next_id: StoredValue<AtomicUsize>,
}

impl RadioGroupState {
    pub(crate) fn new(
        value: Option<RwSignal<Option<String>>>,
        default_value: Option<String>,
        disabled: bool,
    ) -> Self {
        let value_sig = value.unwrap_or_else(|| RwSignal::new(default_value));
        Self {
            value: value_sig,
            disabled,
            item_focus: RwSignal::new(None),
            items: RwSignal::new(Default::default()),
            next_id: StoredValue::new(AtomicUsize::new(0)),
        }
    }

    pub(crate) fn next_index(&self) -> usize {
        self.next_id.with_value(|c| c.fetch_add(1, Ordering::Relaxed))
    }

    pub(crate) fn upsert_item(&self, index: usize, item: RadioItemContext) {
        self.items.update(|m| {
            *m.entry(index).or_insert(item) = item;
        });
    }

    pub(crate) fn remove_item(&self, index: usize) {
        self.items.update(|m| {
            m.remove(&index);
        });
    }

    pub(crate) fn select(&self, value: String) {
        self.value.set(Some(value));
    }
}

impl FilterActiveItems<RadioItemContext> for RadioGroupState {
    fn filter_active_items(&self) -> Vec<RadioItemContext> {
        filter_active(self.items.get())
    }
}

impl ManageFocus for RadioGroupState {
    fn set_focus(&self, index: Option<usize>) {
        self.item_focus.set(index);
    }

    fn item_in_focus(&self, index: usize) -> bool {
        self.item_focus.get() == Some(index)
    }
}

impl NavigateItems<RadioItemContext> for RadioGroupState {
    fn navigate_first_item(&self) -> Option<RadioItemContext> {
        self.filter_active_items().into_iter().next()
    }

    fn navigate_last_item(&self) -> Option<RadioItemContext> {
        self.filter_active_items().into_iter().last()
    }

    fn navigate_next_item(&self) -> Option<RadioItemContext> {
        let items = self.filter_active_items();
        next_item(items, self.item_focus.get(), true)
    }

    fn navigate_previous_item(&self) -> Option<RadioItemContext> {
        let items = self.filter_active_items();
        previous_item(items, self.item_focus.get(), true)
    }
}

#[derive(Copy, Clone)]
pub struct RadioItemContext {
    pub index: usize,
    pub value: StoredValue<String>,
    pub disabled: bool,
    pub trigger_ref: NodeRef<Button>,
}

impl RadioItemContext {
    pub fn is_checked(&self, group_value: Option<String>) -> bool {
        group_value.map(|gv| self.value.with_value(|v| gv == *v)).unwrap_or(false)
    }

    pub fn data_state(&self, group_value: Option<String>) -> &'static str {
        if self.is_checked(group_value) { "checked" } else { "unchecked" }
    }
}

impl GetIndex<usize> for RadioItemContext {
    fn get_index(&self) -> usize {
        self.index
    }
}

impl IsActive for RadioItemContext {
    fn is_active(&self) -> bool {
        !self.disabled
    }
}

impl Focus for RadioItemContext {
    fn focus(&self) -> bool {
        let Some(el) = self.trigger_ref.get() else {
            return false;
        };
        let _ = el.focus();
        true
    }
}
