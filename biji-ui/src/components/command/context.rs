use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

static COMMAND_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub(crate) fn next_command_id() -> String {
    let id = COMMAND_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("biji-command-list-{id}")
}

use leptos::{
    html::{Div, Input},
    prelude::*,
};
use wasm_bindgen::JsCast;

use crate::items::{
    FilterActiveItems, Focus, GetIndex, IsActive, ManageFocus, NavigateItems, filter_active,
    next_item, previous_item,
};

#[derive(Copy, Clone)]
pub struct CommandContext {
    pub root_ref: NodeRef<Div>,
    pub input_ref: NodeRef<Input>,
    pub query: RwSignal<String>,
    pub item_focus: RwSignal<Option<usize>>,
    pub items: RwSignal<HashMap<usize, CommandItemContext>>,
    /// Generated ID shared between Input (aria-controls) and List (id).
    pub list_id: StoredValue<String>,
    pub(crate) next_id: StoredValue<AtomicUsize>,
}

impl Default for CommandContext {
    fn default() -> Self {
        Self {
            root_ref: NodeRef::default(),
            input_ref: NodeRef::default(),
            query: RwSignal::new(String::new()),
            item_focus: RwSignal::new(None),
            items: RwSignal::new(HashMap::new()),
            list_id: StoredValue::new(next_command_id()),
            next_id: StoredValue::new(AtomicUsize::new(0)),
        }
    }
}

impl CommandContext {
    pub fn next_index(&self) -> usize {
        self.next_id.with_value(|c| c.fetch_add(1, Ordering::Relaxed))
    }

    pub fn upsert_item(&self, index: usize, item: CommandItemContext) {
        self.items.update(|m| {
            *m.entry(index).or_insert(item) = item;
        });
    }

    pub fn remove_item(&self, index: usize) {
        self.items.update(|m| {
            m.remove(&index);
        });
    }

    pub fn is_item_visible(&self, index: usize) -> bool {
        let q = self.query.get().to_lowercase();
        if q.is_empty() {
            return true;
        }
        self.items.with(|m| {
            m.get(&index)
                .map(|item| {
                    !item.disabled && item.label.with_value(|l| l.to_lowercase().contains(&q))
                })
                .unwrap_or(false)
        })
    }

    pub fn visible_items(&self) -> Vec<CommandItemContext> {
        let q = self.query.get().to_lowercase();
        let all = filter_active(self.items.get());
        if q.is_empty() {
            return all;
        }
        all.into_iter()
            .filter(|item| item.label.with_value(|l| l.to_lowercase().contains(&q)))
            .collect()
    }
}

impl FilterActiveItems<CommandItemContext> for CommandContext {
    fn filter_active_items(&self) -> Vec<CommandItemContext> {
        filter_active(self.items.get())
    }
}

impl ManageFocus for CommandContext {
    fn set_focus(&self, index: Option<usize>) {
        self.item_focus.set(index);
    }

    fn item_in_focus(&self, index: usize) -> bool {
        self.item_focus.get() == Some(index)
    }
}

impl NavigateItems<CommandItemContext> for CommandContext {
    fn navigate_first_item(&self) -> Option<CommandItemContext> {
        self.visible_items().into_iter().next()
    }

    fn navigate_last_item(&self) -> Option<CommandItemContext> {
        self.visible_items().into_iter().last()
    }

    fn navigate_next_item(&self) -> Option<CommandItemContext> {
        let items = self.visible_items();
        next_item(items, self.item_focus.get(), true)
    }

    fn navigate_previous_item(&self) -> Option<CommandItemContext> {
        let items = self.visible_items();
        previous_item(items, self.item_focus.get(), true)
    }
}

#[derive(Copy, Clone)]
pub struct CommandItemContext {
    pub index: usize,
    pub value: StoredValue<String>,
    pub label: StoredValue<String>,
    pub disabled: bool,
    pub item_ref: NodeRef<Div>,
}

impl GetIndex<usize> for CommandItemContext {
    fn get_index(&self) -> usize {
        self.index
    }
}

impl IsActive for CommandItemContext {
    fn is_active(&self) -> bool {
        !self.disabled
    }
}

impl Focus for CommandItemContext {
    fn focus(&self) -> bool {
        let Some(el) = self.item_ref.get() else {
            return false;
        };
        let el_node: &web_sys::Node = el.as_ref();
        if let Some(html_el) = el_node.dyn_ref::<web_sys::HtmlElement>() {
            let _ = html_el.focus();
            return true;
        }
        false
    }
}

/// Nested context provided by `Group`. Tracks how many of its children are currently visible.
#[derive(Copy, Clone)]
pub struct CommandGroupContext {
    pub visible_count: RwSignal<usize>,
}
