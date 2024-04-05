use std::collections::HashMap;

use leptos::{
    html::{Button, Div},
    *,
};

#[derive(Copy, Clone)]
pub struct AccordionContext {
    pub items: RwSignal<HashMap<usize, AccordionItemContext>>,
    pub accordion_ref: NodeRef<Div>,
    pub current_focus: RwSignal<Option<usize>>,
}

impl AccordionContext {
    pub fn filter_active_accordion_items(&self) -> Vec<AccordionItemContext> {
        let mut items = self
            .items
            .get()
            .values()
            .filter(|item| !item.disabled)
            .cloned()
            .collect::<Vec<AccordionItemContext>>();

        items.sort_by(|a, b| a.index.cmp(&b.index));

        items
    }

    pub fn next_active_accordion_item(&self) -> Option<AccordionItemContext> {
        let active_accordion_items = self.filter_active_accordion_items();

        let Some(current_focus) = self.current_focus.get() else {
            if let Some(first) = active_accordion_items.get(0) {
                return Some(first.clone());
            }
            return None;
        };

        active_accordion_items
            .iter()
            .position(|item| item.index == current_focus)
            .map(|i| {
                if i < active_accordion_items.len() - 1 {
                    active_accordion_items[i + 1].clone()
                } else {
                    active_accordion_items[0].clone()
                }
            })
    }

    pub fn prev_active_accordion_item(&self) -> Option<AccordionItemContext> {
        let active_accordion_items = self.filter_active_accordion_items();

        let Some(current_focus) = self.current_focus.get() else {
            if let Some(last) = active_accordion_items.last() {
                return Some(last.clone());
            }
            return None;
        };

        active_accordion_items
            .iter()
            .position(|item| item.index == current_focus)
            .map(|i| {
                if i > 0 {
                    active_accordion_items[i - 1].clone()
                } else {
                    active_accordion_items[active_accordion_items.len() - 1].clone()
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
        if let Some(item) = self.next_active_accordion_item() {
            return self.focus_item(item.index);
        }
        false
    }

    pub fn focus_previous(&self) -> bool {
        if let Some(item) = self.prev_active_accordion_item() {
            return self.focus_item(item.index);
        }
        false
    }

    pub fn upsert_item(&self, index: usize, item: AccordionItemContext) {
        self.items.update(|items| {
            *items.entry(index).or_insert(item) = item;
        });
    }
}

#[derive(Copy, Clone)]
pub struct AccordionItemContext {
    pub index: usize,
    pub open: RwSignal<bool>,
    pub disabled: bool,
    pub trigger_ref: NodeRef<Button>,
}
