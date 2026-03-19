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

/// Reactive state for a toggle group. Available via [`use_toggle_group`](super::root::use_toggle_group)
/// or the `let:` binding on [`RootWith`](super::root::RootWith).
///
/// All fields are `Copy`, so it is safe to pass this struct to child components as a prop.
#[derive(Copy, Clone)]
pub struct ToggleGroupState {
    pub value: RwSignal<Vec<String>>,
    pub group_type: ToggleGroupType,
    pub disabled: bool,
    pub(crate) item_focus: RwSignal<Option<usize>>,
    pub(crate) items: RwSignal<HashMap<usize, ToggleItemContext>>,
    pub(crate) next_id: StoredValue<AtomicUsize>,
}

impl ToggleGroupState {
    pub(crate) fn new(
        value: Option<RwSignal<Vec<String>>>,
        default_value: Option<String>,
        default_values: Option<Vec<String>>,
        group_type: ToggleGroupType,
        disabled: bool,
    ) -> Self {
        let value_sig = value.unwrap_or_else(|| {
            let initial = match group_type {
                ToggleGroupType::Single => default_value.map(|v| vec![v]).unwrap_or_default(),
                ToggleGroupType::Multiple => default_values.unwrap_or_default(),
            };
            RwSignal::new(initial)
        });
        Self {
            value: value_sig,
            group_type,
            disabled,
            item_focus: RwSignal::new(None),
            items: RwSignal::new(Default::default()),
            next_id: StoredValue::new(AtomicUsize::new(0)),
        }
    }

    pub(crate) fn next_index(&self) -> usize {
        self.next_id.with_value(|c| c.fetch_add(1, Ordering::Relaxed))
    }

    pub(crate) fn upsert_item(&self, index: usize, item: ToggleItemContext) {
        self.items.update(|m| {
            *m.entry(index).or_insert(item) = item;
        });
    }

    pub(crate) fn remove_item(&self, index: usize) {
        self.items.update(|m| {
            m.remove(&index);
        });
    }

    pub(crate) fn is_pressed(&self, val: &str) -> bool {
        self.value.with(|v| v.iter().any(|x| x == val))
    }

    pub(crate) fn toggle_value(&self, val: String) {
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

impl FilterActiveItems<ToggleItemContext> for ToggleGroupState {
    fn filter_active_items(&self) -> Vec<ToggleItemContext> {
        filter_active(self.items.get())
    }
}

impl ManageFocus for ToggleGroupState {
    fn set_focus(&self, index: Option<usize>) {
        self.item_focus.set(index);
    }

    fn item_in_focus(&self, index: usize) -> bool {
        self.item_focus.get() == Some(index)
    }
}

impl NavigateItems<ToggleItemContext> for ToggleGroupState {
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
