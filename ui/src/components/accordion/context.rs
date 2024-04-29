use std::collections::HashMap;

use leptos::{
    html::{Button, Div},
    logging::log,
    *,
};

use crate::items::{
    filter_active, next_item, previous_item, FilterActiveItems, Focus, GetIndex, IsActive,
    ManageFocus, NavigateItems, Toggle,
};

#[derive(Copy, Clone)]
pub struct AccordionContext {
    pub accordion_ref: NodeRef<Div>,
    pub root: RwSignal<RootContext>,
}

#[derive(Copy, Clone)]
pub struct RootContext {
    pub item_focus: RwSignal<Option<usize>>,
    pub items: RwSignal<HashMap<usize, ItemContext>>,
}

impl Default for RootContext {
    fn default() -> Self {
        Self {
            item_focus: create_rw_signal(None),
            items: create_rw_signal(HashMap::new()),
        }
    }
}

impl FilterActiveItems<ItemContext> for RootContext {
    fn filter_active_items(&self) -> Vec<ItemContext> {
        filter_active(self.items.get())
    }
}

impl ManageFocus for RootContext {
    fn set_focus(&self, index: Option<usize>) {
        self.item_focus.set(index);
    }

    fn item_in_focus(&self, index: usize) -> bool {
        self.item_focus.get() == Some(index)
    }
}

impl RootContext {
    pub fn next_index(&self) -> usize {
        self.items.get_untracked().len()
    }

    pub fn upsert_item(&self, index: usize, item: ItemContext) {
        self.items.update(|items| {
            *items.entry(index).or_insert(item) = item;
        });
    }

    pub fn remove_item(&self, index: usize) {
        self.items.update(|items| {
            items.remove(&index);
        });
    }
}

impl NavigateItems<ItemContext> for RootContext {
    fn navigate_first_item(&self) -> Option<ItemContext> {
        let active_items = self.filter_active_items();

        if let Some(first) = active_items.get(0) {
            return Some(first.clone());
        }
        None
    }

    fn navigate_last_item(&self) -> Option<ItemContext> {
        let active_items = self.filter_active_items();

        if let Some(last) = active_items.last() {
            return Some(last.clone());
        }
        None
    }

    fn navigate_next_item(&self) -> Option<ItemContext> {
        let active_items = self.filter_active_items();
        log!("Active items {}", active_items.len());

        next_item(active_items, self.item_focus.get())
    }

    fn navigate_previous_item(&self) -> Option<ItemContext> {
        let active_items = self.filter_active_items();

        previous_item(active_items, self.item_focus.get())
    }
}

#[derive(Copy, Clone)]
pub struct ItemContext {
    pub index: usize,
    pub open: RwSignal<bool>,
    pub disabled: bool,
    pub trigger_ref: NodeRef<Button>,
}

impl ItemContext {
    pub fn data_state(&self) -> String {
        if self.open.get() {
            String::from("open")
        } else {
            String::from("closed")
        }
    }
}

impl Default for ItemContext {
    fn default() -> Self {
        Self {
            index: 0,
            open: create_rw_signal(false),
            disabled: false,
            trigger_ref: NodeRef::default(),
        }
    }
}

impl GetIndex<usize> for ItemContext {
    fn get_index(&self) -> usize {
        self.index
    }
}

impl IsActive for ItemContext {
    fn is_active(&self) -> bool {
        !self.disabled
    }
}

impl Toggle for ItemContext {
    fn toggle(&self) {
        self.open.set(!self.open.get());
    }

    fn open(&self) {
        self.open.set(true);
    }

    fn close(&self) {
        self.open.set(false);
    }
}

impl Focus for ItemContext {
    fn focus(&self) -> bool {
        let Some(trigger_ref) = self.trigger_ref.get() else {
            return false;
        };
        let _ = trigger_ref.focus();
        true
    }
}
