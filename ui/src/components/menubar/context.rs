use std::collections::HashMap;

use leptos::{html::Div, *};

pub trait IsActive {
    fn is_active(&self) -> bool;
}

pub trait GetIndex<T> {
    fn get_index(&self) -> T;
}

pub trait Toggle {
    fn toggle(&self);

    fn open(&self);

    fn close(&self);
}

pub trait Focus {
    /// Focus on element
    fn focus(&self) -> bool;
}

pub trait ManageFocus {
    fn set_focus(&self, index: Option<usize>);

    /// Check if item is in focus
    fn item_in_focus(&self, index: usize) -> bool;
}

pub trait NavigateActiveItems<T> {
    fn active_items(&self) -> Vec<T>;
    fn first_active(&self) -> Option<T>;
    fn last_active(&self) -> Option<T>;
    fn next_active_item(&self) -> Option<T>;
    fn previous_active_item(&self) -> Option<T>;
}

pub fn filter_active<T>(items: HashMap<usize, T>) -> Vec<T>
where
    T: GetIndex<usize> + IsActive + Clone,
{
    let mut items = items
        .values()
        .filter(|item| item.is_active())
        .cloned()
        .collect::<Vec<T>>();

    items.sort_by(|a, b| a.get_index().cmp(&b.get_index()));

    items
}

pub fn next_item<T>(items: Vec<T>, current_focus: Option<usize>) -> Option<T>
where
    T: GetIndex<usize> + Clone,
{
    let Some(item_focus) = current_focus else {
        if let Some(first) = items.get(0) {
            return Some(first.clone());
        }
        return None;
    };

    items
        .iter()
        .position(|item| item.get_index() == item_focus)
        .map(|i| {
            if i < items.len() - 1 {
                items[i + 1].clone()
            } else {
                items[0].clone()
            }
        })
}

pub fn previous_item<T>(items: Vec<T>, current_focus: Option<usize>) -> Option<T>
where
    T: GetIndex<usize> + Clone,
{
    let Some(item_focus) = current_focus else {
        if let Some(last) = items.last() {
            return Some(last.clone());
        }
        return None;
    };

    items
        .iter()
        .position(|item| item.get_index() == item_focus)
        .map(|i| {
            if i > 0 {
                items[i - 1].clone()
            } else {
                items[items.len() - 1].clone()
            }
        })
}

#[derive(Copy, Clone)]
pub struct MenubarContext {
    pub menubar_ref: NodeRef<Div>,
    pub root: RwSignal<RootContext>,
}

#[derive(Copy, Clone)]
pub struct RootContext {
    pub item_focus: RwSignal<Option<usize>>,
    pub items: RwSignal<HashMap<usize, MenuContext>>,
}

impl Default for RootContext {
    fn default() -> Self {
        Self {
            item_focus: create_rw_signal(None),
            items: create_rw_signal(HashMap::new()),
        }
    }
}

impl RootContext {
    pub fn upsert_item(&self, index: usize, item: MenuContext) {
        self.items.update(|items| {
            *items.entry(index).or_insert(item) = item;
        });
    }
}

impl NavigateActiveItems<MenuContext> for RootContext {
    fn active_items(&self) -> Vec<MenuContext> {
        filter_active(self.items.get())
    }

    fn first_active(&self) -> Option<MenuContext> {
        let active_items = self.active_items();

        if let Some(first) = active_items.get(0) {
            return Some(first.clone());
        }
        None
    }

    fn last_active(&self) -> Option<MenuContext> {
        let active_items = self.active_items();

        if let Some(last) = active_items.last() {
            return Some(last.clone());
        }
        None
    }

    fn next_active_item(&self) -> Option<MenuContext> {
        let active_items = self.active_items();

        next_item(active_items, self.item_focus.get())
    }

    fn previous_active_item(&self) -> Option<MenuContext> {
        let active_items = self.active_items();

        previous_item(active_items, self.item_focus.get())
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
}

impl Default for MenuContext {
    fn default() -> Self {
        Self {
            index: 0,
            disabled: false,
            open: create_rw_signal(false),
            menu_ref: NodeRef::default(),
            trigger_ref: NodeRef::default(),
            item_focus: create_rw_signal(None),
            items: create_rw_signal(HashMap::new()),
        }
    }
}

impl MenuContext {
    pub fn upsert_item(&self, index: usize, item: ItemData) {
        self.items.update(|items| {
            *items.entry(index).or_insert(item) = item;
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

impl NavigateActiveItems<ItemData> for MenuContext {
    fn active_items(&self) -> Vec<ItemData> {
        filter_active(self.items.get())
    }

    fn first_active(&self) -> Option<ItemData> {
        let active_items = self.active_items();

        if let Some(first) = active_items.get(0) {
            return Some(first.clone());
        }
        None
    }

    fn last_active(&self) -> Option<ItemData> {
        let active_items = self.active_items();

        if let Some(last) = active_items.last() {
            return Some(last.clone());
        }
        None
    }

    fn next_active_item(&self) -> Option<ItemData> {
        let active_items = self.active_items();

        next_item(active_items, self.item_focus.get())
    }

    fn previous_active_item(&self) -> Option<ItemData> {
        let active_items = self.active_items();

        previous_item(active_items, self.item_focus.get())
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
