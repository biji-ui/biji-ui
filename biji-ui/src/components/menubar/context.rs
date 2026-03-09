use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

use leptos::{html::Div, prelude::*};

use crate::{
    items::{
        FilterActiveItems, Focus, GetIndex, IsActive, ManageFocus, NavigateItems, Toggle,
        filter_active, next_item, previous_item,
    },
    utils::positioning::{AvoidCollisions, Positioning},
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
    pub(crate) next_id: StoredValue<AtomicUsize>,
}

impl Default for RootContext {
    fn default() -> Self {
        Self {
            item_focus: RwSignal::new(None),
            items: RwSignal::new(HashMap::new()),
            allow_menu_loop: false,
            allow_item_loop: false,
            prevent_scroll: false,
            next_id: StoredValue::new(AtomicUsize::new(0)),
        }
    }
}

impl RootContext {
    pub fn upsert_item(&self, index: usize, item: MenuContext) {
        self.items.update(|items| {
            items.insert(index, item);
        });
    }

    pub fn remove_item(&self, index: usize) {
        self.items.update(|items| {
            items.remove(&index);
        });
    }

    pub fn close_all(&self) {
        self.items.with_untracked(|items| {
            for item in items.values() {
                item.close_all(); // cascade into submenus before hiding the menu
                item.close();
            }
        });
    }

    pub fn next_index(&self) -> usize {
        self.next_id
            .with_value(|counter| counter.fetch_add(1, Ordering::Relaxed))
    }

    pub fn any_open(&self) -> bool {
        self.items.with_untracked(|items| {
            items.values().any(|m| m.open.get_untracked())
        })
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
        active_items.first().copied()
    }

    fn navigate_last_item(&self) -> Option<MenuContext> {
        let active_items = self.filter_active_items();
        active_items.last().copied()
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
    pub avoid_collisions: AvoidCollisions,
    pub hide_delay: Duration,
    /// When `true`, the next `focus` event on this menu's trigger will not
    /// auto-open the submenu. Used by the ArrowLeft handler to return focus to
    /// a SubMenuItem trigger without the focus listener re-opening it.
    pub skip_open_on_focus: RwSignal<bool>,
    pub(crate) next_id: StoredValue<AtomicUsize>,
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
            avoid_collisions: AvoidCollisions::Flip,
            hide_delay: Duration::from_millis(200),
            skip_open_on_focus: RwSignal::new(false),
            next_id: StoredValue::new(AtomicUsize::new(0)),
        }
    }
}

impl MenuContext {
    pub fn upsert_item(&self, index: usize, item: ItemData) {
        self.items.update(|items| {
            items.insert(index, item);
        });
    }

    pub fn remove_item(&self, index: usize) {
        self.items.update(|items| {
            items.remove(&index);
        });
    }

    pub fn next_index(&self) -> usize {
        self.next_id
            .with_value(|counter| counter.fetch_add(1, Ordering::Relaxed))
    }

    pub fn close_all(&self) {
        self.close_all_except(usize::MAX);
    }

    /// Closes every sibling submenu except the one with `except_index`.
    /// Use this instead of `close_all` when you are about to open `except_index`
    /// so that you never close-then-reopen the same submenu in the same event.
    pub fn close_all_except(&self, except_index: usize) {
        self.items.with_untracked(|items| {
            for item in items.values() {
                if let ItemData::SubMenuItem { child_context, index, .. } = item {
                    if *index != except_index {
                        child_context.close_all(); // recursively close nested submenus
                        child_context.close();
                    }
                }
            }
        });
    }

    /// Closes all nested submenus and then closes this menu.
    /// Use instead of `close()` whenever leaving a menu that may have open
    /// submenus, so their `open` signals are reset and hover works correctly
    /// the next time the menu is opened.
    pub fn close_with_submenus(&self) {
        self.close_all();
        self.close();
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
        active_items.first().copied()
    }

    fn navigate_last_item(&self) -> Option<ItemData> {
        let active_items = self.filter_active_items();
        active_items.last().copied()
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
        let next = !self.open.get_untracked();
        self.open.set(next);
    }

    fn open(&self) {
        // Guard: skip set (and the resulting reactive notification) if already open.
        if self.open.get_untracked() {
            return;
        }
        self.open.set(true);
    }

    fn close(&self) {
        // Guard: skip set (and the resulting reactive notification) if already closed.
        if !self.open.get_untracked() {
            return;
        }
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
            ItemData::Item { is_submenu, .. } | ItemData::SubMenuItem { is_submenu, .. } => {
                *is_submenu
            }
        }
    }

    pub fn get_disabled(&self) -> bool {
        match self {
            ItemData::Item { disabled, .. } | ItemData::SubMenuItem { disabled, .. } => *disabled,
        }
    }
}

impl IsActive for ItemData {
    fn is_active(&self) -> bool {
        !self.get_disabled()
    }
}

impl GetIndex<usize> for ItemData {
    fn get_index(&self) -> usize {
        match self {
            ItemData::Item { index, .. } | ItemData::SubMenuItem { index, .. } => *index,
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
