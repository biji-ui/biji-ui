use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

use leptos::{html::Button, prelude::*};

use crate::items::{
    FilterActiveItems, Focus, GetIndex, IsActive, ManageFocus, NavigateItems, filter_active,
    next_item, previous_item,
};

#[derive(Copy, Clone)]
pub struct RadioGroupContext {
    pub value: RwSignal<Option<String>>,
    pub item_focus: RwSignal<Option<usize>>,
    pub items: RwSignal<HashMap<usize, RadioItemContext>>,
    pub disabled: bool,
    pub(crate) next_id: StoredValue<AtomicUsize>,
}

impl RadioGroupContext {
    pub fn next_index(&self) -> usize {
        self.next_id
            .with_value(|c| c.fetch_add(1, Ordering::Relaxed))
    }

    pub fn upsert_item(&self, index: usize, item: RadioItemContext) {
        self.items.update(|m| {
            *m.entry(index).or_insert(item) = item;
        });
    }

    pub fn remove_item(&self, index: usize) {
        self.items.update(|m| {
            m.remove(&index);
        });
    }

    pub fn select(&self, value: String) {
        self.value.set(Some(value));
    }
}

impl FilterActiveItems<RadioItemContext> for RadioGroupContext {
    fn filter_active_items(&self) -> Vec<RadioItemContext> {
        filter_active(self.items.get())
    }
}

impl ManageFocus for RadioGroupContext {
    fn set_focus(&self, index: Option<usize>) {
        self.item_focus.set(index);
    }

    fn item_in_focus(&self, index: usize) -> bool {
        self.item_focus.get() == Some(index)
    }
}

impl NavigateItems<RadioItemContext> for RadioGroupContext {
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
        if self.is_checked(group_value) {
            "checked"
        } else {
            "unchecked"
        }
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
