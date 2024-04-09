use std::collections::HashMap;

use leptos::{
    html::{Button, Div},
    *,
};

#[derive(Copy, Clone)]
pub struct MenubarContext {
    pub in_focus: RwSignal<bool>,
    pub menubar_ref: NodeRef<Div>,
    pub close_on_outside_click: bool,
    pub items: RwSignal<HashMap<usize, MenubarMenuContext>>,
    pub current_focus: RwSignal<Option<usize>>,
}

impl MenubarContext {
    pub fn filter_active_menubar_items(&self) -> Vec<MenubarMenuContext> {
        let mut items = self
            .items
            .get()
            .values()
            .filter(|item| !item.disabled)
            .cloned()
            .collect::<Vec<MenubarMenuContext>>();

        items.sort_by(|a, b| a.index.cmp(&b.index));

        items
    }

    pub fn next_active_menubar_item(&self) -> Option<MenubarMenuContext> {
        let active_menubar_items = self.filter_active_menubar_items();

        let Some(current_focus) = self.current_focus.get() else {
            if let Some(first) = active_menubar_items.get(0) {
                return Some(first.clone());
            }
            return None;
        };

        active_menubar_items
            .iter()
            .position(|item| item.index == current_focus)
            .map(|i| {
                if i < active_menubar_items.len() - 1 {
                    active_menubar_items[i + 1].clone()
                } else {
                    active_menubar_items[0].clone()
                }
            })
    }

    pub fn previous_active_menubar_item(&self) -> Option<MenubarMenuContext> {
        let active_menubar_items = self.filter_active_menubar_items();

        let Some(current_focus) = self.current_focus.get() else {
            if let Some(last) = active_menubar_items.last() {
                return Some(last.clone());
            }
            return None;
        };

        active_menubar_items
            .iter()
            .position(|item| item.index == current_focus)
            .map(|i| {
                if i > 0 {
                    active_menubar_items[i - 1].clone()
                } else {
                    active_menubar_items[active_menubar_items.len() - 1].clone()
                }
            })
    }

    pub fn focus_item(&self, i: usize, open: bool) -> bool {
        if let Some(item) = self.items.get().get(&i) {
            if !item.disabled {
                self.current_focus.set(Some(i));
                if let Some(trigger_ref) = item.trigger_ref.get() {
                    let _ = trigger_ref.focus();
                    if open {
                        item.open();
                    }
                    return true;
                }
            }
        }

        false
    }

    pub fn focus_current(&self) -> bool {
        let Some(current_focus) = self.current_focus.get() else {
            return false;
        };

        self.focus_item(current_focus, false)
    }

    pub fn get_current_menubar_menu(&self) -> Option<MenubarMenuContext> {
        if let Some(current_focus) = self.current_focus.get() {
            return self.items.get().get(&current_focus).cloned();
        }

        None
    }

    pub fn close_current_focus(&self) -> bool {
        if let Some(current_focus) = self.current_focus.get() {
            if let Some(item) = self.items.get().get(&current_focus) {
                let was_open = item.open.get();
                item.close();
                return was_open;
            }
        }
        false
    }

    pub fn focus_next(&self) -> bool {
        let was_open = self.close_current_focus();
        if let Some(item) = self.next_active_menubar_item() {
            return self.focus_item(item.index, was_open);
        }
        false
    }

    pub fn focus_previous(&self) -> bool {
        let was_open = self.close_current_focus();
        if let Some(item) = self.previous_active_menubar_item() {
            return self.focus_item(item.index, was_open);
        }
        false
    }

    pub fn upsert_item(&self, index: usize, item: MenubarMenuContext) {
        self.items.update(|items| {
            *items.entry(index).or_insert(item) = item;
        });
    }
}

#[derive(Copy, Clone)]
pub struct MenubarMenuContext {
    pub index: usize,
    pub disabled: bool,
    pub close_on_outside_click: bool,
    pub menu_ref: NodeRef<Div>,
    pub trigger_ref: NodeRef<Button>,
    pub open: RwSignal<bool>,
    pub in_focus: RwSignal<bool>,
    pub items: RwSignal<HashMap<usize, MenubarMenuItemContext>>,
    pub current_focus: RwSignal<Option<usize>>,
}

impl MenubarMenuContext {
    pub fn open(&self) {
        self.open.set(true);
        self.in_focus.set(true);
    }

    pub fn close(&self) {
        self.open.set(false);
        self.in_focus.set(false);
    }

    pub fn filter_active_menubar_menu_items(&self) -> Vec<MenubarMenuItemContext> {
        let mut items = self
            .items
            .get()
            .values()
            .filter(|item| !item.disabled)
            .cloned()
            .collect::<Vec<MenubarMenuItemContext>>();

        items.sort_by(|a, b| a.index.cmp(&b.index));

        items
    }

    pub fn next_active_item(&self) -> Option<MenubarMenuItemContext> {
        let active_items = self.filter_active_menubar_menu_items();

        let Some(current_focus) = self.current_focus.get() else {
            if let Some(first) = active_items.get(0) {
                return Some(first.clone());
            }
            return None;
        };

        active_items
            .iter()
            .position(|item| item.index == current_focus)
            .map(|i| {
                if i < active_items.len() - 1 {
                    active_items[i + 1].clone()
                } else {
                    active_items[0].clone()
                }
            })
    }

    pub fn previous_active_item(&self) -> Option<MenubarMenuItemContext> {
        let active_items = self.filter_active_menubar_menu_items();

        let Some(current_focus) = self.current_focus.get() else {
            if let Some(last) = active_items.last() {
                return Some(last.clone());
            }
            return None;
        };

        active_items
            .iter()
            .position(|item| item.index == current_focus)
            .map(|i| {
                if i > 0 {
                    active_items[i - 1].clone()
                } else {
                    active_items[active_items.len() - 1].clone()
                }
            })
    }

    pub fn focus_item(&self, i: usize) -> bool {
        if let Some(item) = self.items.get().get(&i) {
            if !item.disabled {
                if let Some(trigger_ref) = item.trigger_ref.get() {
                    self.current_focus.set(Some(i));
                    let _ = trigger_ref.focus();
                    return true;
                }
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
        if let Some(item) = self.next_active_item() {
            return self.focus_item(item.index);
        }
        false
    }

    pub fn focus_previous(&self) -> bool {
        if let Some(item) = self.previous_active_item() {
            return self.focus_item(item.index);
        }
        false
    }

    pub fn upsert_item(&self, index: usize, item: MenubarMenuItemContext) {
        self.items.update(|items| {
            *items.entry(index).or_insert(item) = item;
        });
    }
}

#[derive(Copy, Clone)]
pub struct MenubarMenuItemContext {
    pub index: usize,
    pub disabled: bool,
    pub trigger_ref: NodeRef<Div>,
}
