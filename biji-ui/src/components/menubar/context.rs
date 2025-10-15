use std::{collections::HashMap, time::Duration};

use leptos::{html::Div, prelude::*};

use crate::{
    items::{
        filter_active, next_item, previous_item, FilterActiveItems, Focus, GetIndex, IsActive,
        ManageFocus, NavigateItems, Toggle,
    },
    utils::positioning::Positioning,
};

#[derive(Copy, Clone)]
pub struct MenubarContext {
    pub menubar_ref: NodeRef<Div>,
    pub root: RwSignal<RootContext>,
}

#[derive(Copy, Clone)]
pub struct RootContext {
    pub item_focus: RwSignal<Option<usize>>,
    pub items: RwSignal<HashMap<usize, MenuContext>>,
    pub allow_menu_loop: bool,
    pub allow_item_loop: bool,
    pub prevent_scroll: bool,
}

impl Default for RootContext {
    fn default() -> Self {
        Self {
            item_focus: RwSignal::new(None),
            items: RwSignal::new(HashMap::new()),
            allow_menu_loop: false,
            allow_item_loop: false,
            prevent_scroll: false,
        }
    }
}

impl RootContext {
    pub fn upsert_item(&self, index: usize, item: MenuContext) {
        self.items.update(|items| {
            *items.entry(index).or_insert(item) = item;
        });
    }

    pub fn remove_item(&self, index: usize) {
        self.items.update(|items| {
            items.remove(&index);
        });
    }

    pub fn close_all(&self) {
        self.items.try_update(|items| {
            for item in items.values() {
                item.open.set(false);
            }
        });
    }

    pub fn next_index(&self) -> usize {
        self.items.get_untracked().len()
    }

    pub fn focus_active_item(&self) -> bool {
        if let Some(Some(item_focus)) = self.item_focus.try_get_untracked() {
            if let Some(item) = self.items.get_untracked().get(&item_focus) {
                return item.focus();
            }
        }
        false
    }
}

impl FilterActiveItems<MenuContext> for RootContext {
    fn filter_active_items(&self) -> Vec<MenuContext> {
        filter_active(self.items.get())
    }
}

impl NavigateItems<MenuContext> for RootContext {
    fn navigate_first_item(&self) -> Option<MenuContext> {
        let active_items = self.filter_active_items();

        if let Some(first) = active_items.get(0) {
            return Some(first.clone());
        }
        None
    }

    fn navigate_last_item(&self) -> Option<MenuContext> {
        let active_items = self.filter_active_items();

        if let Some(last) = active_items.last() {
            return Some(last.clone());
        }
        None
    }

    fn navigate_next_item(&self) -> Option<MenuContext> {
        let active_items = self.filter_active_items();

        next_item(active_items, self.item_focus.get(), self.allow_menu_loop)
    }

    fn navigate_previous_item(&self) -> Option<MenuContext> {
        let active_items = self.filter_active_items();

        previous_item(active_items, self.item_focus.get(), self.allow_menu_loop)
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

#[derive(Copy, Clone)]
pub struct MenuContext {
    pub index: usize,
    pub disabled: bool,
    pub open: RwSignal<bool>,
    pub menu_ref: NodeRef<Div>,
    pub trigger_ref: NodeRef<Div>,
    pub item_focus: RwSignal<Option<usize>>,
    pub items: RwSignal<HashMap<usize, ItemData>>,
    pub allow_loop: bool,
    pub positioning: Positioning,
    pub hide_delay: Duration,
}

impl Default for MenuContext {
    fn default() -> Self {
        Self {
            index: 0,
            disabled: false,
            open: RwSignal::new(false),
            menu_ref: NodeRef::default(),
            trigger_ref: NodeRef::default(),
            item_focus: RwSignal::new(None),
            items: RwSignal::new(HashMap::new()),
            allow_loop: false,
            positioning: Positioning::BottomStart,
            hide_delay: Duration::from_millis(200),
        }
    }
}

impl MenuContext {
    pub fn upsert_item(&self, index: usize, item: ItemData) {
        self.items.update(|items| {
            *items.entry(index).or_insert(item) = item;
        });
    }

    pub fn remove_item(&self, index: usize) {
        self.items.update(|items| {
            items.remove(&index);
        });
    }

    pub fn next_index(&self) -> usize {
        self.items.get_untracked().len()
    }

    pub fn close_all(&self) {
        self.items.try_update(|items| {
            for item in items.values() {
                if let ItemData::SubMenuItem { child_context, .. } = item {
                    child_context.close();
                }
            }
        });
    }
}

impl IsActive for MenuContext {
    fn is_active(&self) -> bool {
        !self.disabled
    }
}

impl GetIndex<usize> for MenuContext {
    fn get_index(&self) -> usize {
        self.index
    }
}

impl Focus for MenuContext {
    fn focus(&self) -> bool {
        let Some(trigger_ref) = self.trigger_ref.get() else {
            return false;
        };

        let _ = trigger_ref.focus();
        true
    }
}

impl ManageFocus for MenuContext {
    fn set_focus(&self, index: Option<usize>) {
        self.item_focus.set(index);
    }

    fn item_in_focus(&self, index: usize) -> bool {
        self.item_focus.get() == Some(index)
    }
}

impl FilterActiveItems<ItemData> for MenuContext {
    fn filter_active_items(&self) -> Vec<ItemData> {
        filter_active(self.items.get())
    }
}

impl NavigateItems<ItemData> for MenuContext {
    fn navigate_first_item(&self) -> Option<ItemData> {
        let active_items = self.filter_active_items();

        if let Some(first) = active_items.get(0) {
            return Some(first.clone());
        }
        None
    }

    fn navigate_last_item(&self) -> Option<ItemData> {
        let active_items = self.filter_active_items();

        if let Some(last) = active_items.last() {
            return Some(last.clone());
        }
        None
    }

    fn navigate_next_item(&self) -> Option<ItemData> {
        let active_items = self.filter_active_items();

        next_item(active_items, self.item_focus.get(), self.allow_loop)
    }

    fn navigate_previous_item(&self) -> Option<ItemData> {
        let active_items = self.filter_active_items();

        previous_item(active_items, self.item_focus.get(), self.allow_loop)
    }
}

impl Toggle for MenuContext {
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

#[derive(Copy, Clone)]
pub enum ItemData {
    Item {
        index: usize,
        disabled: bool,
        trigger_ref: NodeRef<Div>,
        is_submenu: bool,
    },
    SubMenuItem {
        index: usize,
        disabled: bool,
        is_submenu: bool,
        parent_context: MenuContext,
        child_context: MenuContext,
    },
}

impl ItemData {
    pub fn get_trigger_ref(&self) -> NodeRef<Div> {
        match self {
            ItemData::Item { trigger_ref, .. } => trigger_ref.clone(),
            ItemData::SubMenuItem {
                child_context: context,
                ..
            } => context.trigger_ref.clone(),
        }
    }

    pub fn is_submenu(&self) -> bool {
        match self {
            ItemData::Item { is_submenu, .. } => *is_submenu,
            ItemData::SubMenuItem { is_submenu, .. } => *is_submenu,
        }
    }

    pub fn get_disabled(&self) -> bool {
        match self {
            ItemData::Item { disabled, .. } => *disabled,
            ItemData::SubMenuItem { disabled, .. } => *disabled,
        }
    }
}

impl IsActive for ItemData {
    fn is_active(&self) -> bool {
        match self {
            ItemData::Item { disabled, .. } => !disabled,
            ItemData::SubMenuItem { disabled, .. } => !disabled,
        }
    }
}

impl GetIndex<usize> for ItemData {
    fn get_index(&self) -> usize {
        match self {
            ItemData::Item { index, .. } => *index,
            ItemData::SubMenuItem { index, .. } => *index,
        }
    }
}

impl Focus for ItemData {
    fn focus(&self) -> bool {
        match self {
            ItemData::Item { trigger_ref, .. } => {
                let Some(trigger_ref) = trigger_ref.get() else {
                    return false;
                };

                let _ = trigger_ref.focus();
                true
            }
            ItemData::SubMenuItem {
                child_context: context,
                ..
            } => {
                let Some(trigger_ref) = context.trigger_ref.get() else {
                    return false;
                };

                let _ = trigger_ref.focus();
                true
            }
        }
    }
}
