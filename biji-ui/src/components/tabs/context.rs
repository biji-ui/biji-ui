use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

use leptos::{html::Button, prelude::*};

use crate::items::{
    FilterActiveItems, Focus, GetIndex, IsActive, ManageFocus, NavigateItems, filter_active,
    next_item, previous_item,
};

#[derive(Copy, Clone, Default, PartialEq)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Copy, Clone, Default, PartialEq)]
pub enum ActivationMode {
    /// Arrow keys move focus AND activate the tab.
    #[default]
    Automatic,
    /// Arrow keys move focus only; Enter/Space activates.
    Manual,
}

#[derive(Copy, Clone)]
pub struct TabsContext {
    pub value: RwSignal<Option<String>>,
    pub item_focus: RwSignal<Option<usize>>,
    pub items: RwSignal<HashMap<usize, TabItemContext>>,
    pub orientation: Orientation,
    pub activation_mode: ActivationMode,
    pub(crate) on_value_change: Option<Callback<String>>,
    pub(crate) next_id: StoredValue<AtomicUsize>,
}

impl TabsContext {
    pub fn next_index(&self) -> usize {
        self.next_id
            .with_value(|c| c.fetch_add(1, Ordering::Relaxed))
    }

    pub fn upsert_item(&self, index: usize, item: TabItemContext) {
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
        self.value.set(Some(value.clone()));
        if let Some(cb) = self.on_value_change {
            cb.run(value);
        }
    }
}

impl FilterActiveItems<TabItemContext> for TabsContext {
    fn filter_active_items(&self) -> Vec<TabItemContext> {
        filter_active(self.items.get())
    }
}

impl ManageFocus for TabsContext {
    fn set_focus(&self, index: Option<usize>) {
        self.item_focus.set(index);
    }

    fn item_in_focus(&self, index: usize) -> bool {
        self.item_focus.get() == Some(index)
    }
}

impl NavigateItems<TabItemContext> for TabsContext {
    fn navigate_first_item(&self) -> Option<TabItemContext> {
        self.filter_active_items().into_iter().next()
    }

    fn navigate_last_item(&self) -> Option<TabItemContext> {
        self.filter_active_items().into_iter().last()
    }

    fn navigate_next_item(&self) -> Option<TabItemContext> {
        let items = self.filter_active_items();
        next_item(items, self.item_focus.get(), true)
    }

    fn navigate_previous_item(&self) -> Option<TabItemContext> {
        let items = self.filter_active_items();
        previous_item(items, self.item_focus.get(), true)
    }
}

#[derive(Copy, Clone)]
pub struct TabItemContext {
    pub index: usize,
    pub value: StoredValue<String>,
    pub disabled: bool,
    pub trigger_ref: NodeRef<Button>,
    pub trigger_id: StoredValue<String>,
    pub panel_id: StoredValue<String>,
}

impl GetIndex<usize> for TabItemContext {
    fn get_index(&self) -> usize {
        self.index
    }
}

impl IsActive for TabItemContext {
    fn is_active(&self) -> bool {
        !self.disabled
    }
}

impl Focus for TabItemContext {
    fn focus(&self) -> bool {
        let Some(el) = self.trigger_ref.get() else {
            return false;
        };
        let _ = el.focus();
        true
    }
}
