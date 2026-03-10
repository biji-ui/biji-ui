use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

use leptos::{html::Button, prelude::*};

use crate::items::{
    FilterActiveItems, Focus, GetIndex, IsActive, ManageFocus, NavigateItems, filter_active,
    next_item, previous_item,
};

#[derive(Copy, Clone, PartialEq, Default)]
pub enum ToggleGroupType {
    #[default]
    Single,
    Multiple,
}

#[derive(Copy, Clone)]
pub struct ToggleGroupContext {
    /// Current selected value(s). Always a Vec; Single mode keeps at most one entry.
    pub value: RwSignal<Vec<String>>,
    pub group_type: ToggleGroupType,
    pub disabled: bool,
    pub item_focus: RwSignal<Option<usize>>,
    pub items: RwSignal<HashMap<usize, ToggleItemContext>>,
    pub(crate) next_id: StoredValue<AtomicUsize>,
    pub(crate) on_value_change: Option<Callback<String>>,
    pub(crate) on_values_change: Option<Callback<Vec<String>>>,
}

impl ToggleGroupContext {
    pub fn next_index(&self) -> usize {
        self.next_id.with_value(|c| c.fetch_add(1, Ordering::Relaxed))
    }

    pub fn upsert_item(&self, index: usize, item: ToggleItemContext) {
        self.items.update(|m| {
            *m.entry(index).or_insert(item) = item;
        });
    }

    pub fn remove_item(&self, index: usize) {
        self.items.update(|m| {
            m.remove(&index);
        });
    }

    pub fn is_pressed(&self, val: &str) -> bool {
        self.value.with(|v| v.iter().any(|x| x == val))
    }

    pub fn toggle_value(&self, val: String) {
        match self.group_type {
            ToggleGroupType::Single => {
                self.value.update(|v| {
                    if v.first().map(|x| x == &val).unwrap_or(false) {
                        v.clear();
                    } else {
                        *v = vec![val];
                    }
                });
            }
            ToggleGroupType::Multiple => {
                self.value.update(|v| {
                    if let Some(pos) = v.iter().position(|x| x == &val) {
                        v.remove(pos);
                    } else {
                        v.push(val);
                    }
                });
            }
        }
    }
}

impl FilterActiveItems<ToggleItemContext> for ToggleGroupContext {
    fn filter_active_items(&self) -> Vec<ToggleItemContext> {
        filter_active(self.items.get())
    }
}

impl ManageFocus for ToggleGroupContext {
    fn set_focus(&self, index: Option<usize>) {
        self.item_focus.set(index);
    }

    fn item_in_focus(&self, index: usize) -> bool {
        self.item_focus.get() == Some(index)
    }
}

impl NavigateItems<ToggleItemContext> for ToggleGroupContext {
    fn navigate_first_item(&self) -> Option<ToggleItemContext> {
        self.filter_active_items().into_iter().next()
    }

    fn navigate_last_item(&self) -> Option<ToggleItemContext> {
        self.filter_active_items().into_iter().last()
    }

    fn navigate_next_item(&self) -> Option<ToggleItemContext> {
        let items = self.filter_active_items();
        next_item(items, self.item_focus.get(), true)
    }

    fn navigate_previous_item(&self) -> Option<ToggleItemContext> {
        let items = self.filter_active_items();
        previous_item(items, self.item_focus.get(), true)
    }
}

#[derive(Copy, Clone)]
pub struct ToggleItemContext {
    pub index: usize,
    pub value: StoredValue<String>,
    pub disabled: bool,
    pub trigger_ref: NodeRef<Button>,
}

impl GetIndex<usize> for ToggleItemContext {
    fn get_index(&self) -> usize {
        self.index
    }
}

impl IsActive for ToggleItemContext {
    fn is_active(&self) -> bool {
        !self.disabled
    }
}

impl Focus for ToggleItemContext {
    fn focus(&self) -> bool {
        let Some(el) = self.trigger_ref.get() else {
            return false;
        };
        let _ = el.focus();
        true
    }
}
