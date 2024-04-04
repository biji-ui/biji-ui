use std::collections::HashMap;

use leptos::{
    html::{Button, Div},
    *,
};

#[derive(Copy, Clone)]
pub struct MenuContext {
    pub open: RwSignal<bool>,
    pub in_focus: RwSignal<bool>,
    pub close_on_outside_click: bool,
    pub menu_ref: NodeRef<Div>,
    pub trigger_ref: NodeRef<Button>,
    pub items: RwSignal<HashMap<usize, MenuItemContext>>,
    pub current_focus: RwSignal<Option<usize>>,
}

impl MenuContext {
    pub fn open(&self) {
        self.open.set(true);
        self.in_focus.set(true);
    }

    pub fn close(&self) {
        self.open.set(false);
        self.in_focus.set(false);
    }

    pub fn filter_active_menu_items(&self) -> Vec<MenuItemContext> {
        let mut items = self
            .items
            .get()
            .values()
            .filter(|item| !item.disabled)
            .cloned()
            .collect::<Vec<MenuItemContext>>();

        items.sort_by(|a, b| a.index.cmp(&b.index));

        items
    }

    pub fn next_active_menu_item(&self) -> Option<MenuItemContext> {
        let active_menu_items = self.filter_active_menu_items();

        let Some(current_focus) = self.current_focus.get() else {
            if let Some(first) = active_menu_items.get(0) {
                return Some(first.clone());
            }
            return None;
        };

        active_menu_items
            .iter()
            .position(|item| item.index == current_focus)
            .map(|i| {
                if i < active_menu_items.len() - 1 {
                    active_menu_items[i + 1].clone()
                } else {
                    active_menu_items[0].clone()
                }
            })
    }

    pub fn prev_active_menu_item(&self) -> Option<MenuItemContext> {
        let active_menu_items = self.filter_active_menu_items();

        let Some(current_focus) = self.current_focus.get() else {
            if let Some(last) = active_menu_items.last() {
                return Some(last.clone());
            }
            return None;
        };

        active_menu_items
            .iter()
            .position(|item| item.index == current_focus)
            .map(|i| {
                if i > 0 {
                    active_menu_items[i - 1].clone()
                } else {
                    active_menu_items[active_menu_items.len() - 1].clone()
                }
            })
    }

    pub fn focus_item(&self, i: usize) -> bool {
        if let Some(item) = self.items.get().get(&i) {
            if let Some(trigger_ref) = item.trigger_ref.get() {
                let _ = trigger_ref.focus();
                self.current_focus.set(Some(i));
                return true;
            }
        }
        false
    }

    pub fn focus_current(&self) -> bool {
        let Some(current_focus) = self.current_focus.get() else {
            return false;
        };

        self.focus_item(current_focus)
    }

    pub fn focus_next(&self) -> bool {
        if let Some(item) = self.next_active_menu_item() {
            return self.focus_item(item.index);
        }
        false
    }

    pub fn focus_previous(&self) -> bool {
        if let Some(item) = self.prev_active_menu_item() {
            return self.focus_item(item.index);
        }
        false
    }
}

#[derive(Copy, Clone)]
pub struct MenuItemContext {
    pub index: usize,
    pub disabled: bool,
    pub trigger_ref: NodeRef<Div>,
}
