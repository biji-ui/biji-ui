use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

use leptos::{html::Div, prelude::*};
use wasm_bindgen::JsCast;

use crate::items::{
    FilterActiveItems, Focus, GetIndex, IsActive, ManageFocus, NavigateItems, filter_active,
    next_item, previous_item,
};

#[derive(Copy, Clone)]
pub struct ContextMenuState {
    pub(crate) trigger_ref: NodeRef<Div>,
    pub(crate) content_ref: NodeRef<Div>,
    pub open: RwSignal<bool>,
    /// Viewport X coordinate where the context menu was invoked.
    pub pointer_x: RwSignal<f64>,
    /// Viewport Y coordinate where the context menu was invoked.
    pub pointer_y: RwSignal<f64>,
    pub data_state: Signal<&'static str>,
    pub(crate) item_focus: RwSignal<Option<usize>>,
    pub(crate) items: RwSignal<HashMap<usize, ContextMenuItemContext>>,
    pub(crate) allow_loop: bool,
    pub(crate) hide_delay: Duration,
    pub(crate) menu_id: StoredValue<String>,
    pub(crate) next_id: StoredValue<AtomicUsize>,
}

impl ContextMenuState {
    pub(crate) fn next_index(&self) -> usize {
        self.next_id
            .with_value(|c| c.fetch_add(1, Ordering::Relaxed))
    }

    pub(crate) fn upsert_item(&self, index: usize, item: ContextMenuItemContext) {
        self.items.update(|m| {
            *m.entry(index).or_insert(item) = item;
        });
    }

    pub(crate) fn remove_item(&self, index: usize) {
        self.items.update(|m| {
            m.remove(&index);
        });
    }

    pub(crate) fn open_at(&self, x: f64, y: f64) {
        self.pointer_x.set(x);
        self.pointer_y.set(y);
        self.open.set(true);
    }

    pub(crate) fn close(&self) {
        self.open.set(false);
        self.item_focus.set(None);
    }
}

impl FilterActiveItems<ContextMenuItemContext> for ContextMenuState {
    fn filter_active_items(&self) -> Vec<ContextMenuItemContext> {
        filter_active(self.items.get())
    }
}

impl ManageFocus for ContextMenuState {
    fn set_focus(&self, index: Option<usize>) {
        self.item_focus.set(index);
    }

    fn item_in_focus(&self, index: usize) -> bool {
        self.item_focus.get() == Some(index)
    }
}

impl NavigateItems<ContextMenuItemContext> for ContextMenuState {
    fn navigate_first_item(&self) -> Option<ContextMenuItemContext> {
        self.filter_active_items().into_iter().next()
    }

    fn navigate_last_item(&self) -> Option<ContextMenuItemContext> {
        self.filter_active_items().into_iter().last()
    }

    fn navigate_next_item(&self) -> Option<ContextMenuItemContext> {
        let items = self.filter_active_items();
        next_item(items, self.item_focus.get(), self.allow_loop)
    }

    fn navigate_previous_item(&self) -> Option<ContextMenuItemContext> {
        let items = self.filter_active_items();
        previous_item(items, self.item_focus.get(), self.allow_loop)
    }
}

#[derive(Copy, Clone)]
pub struct ContextMenuItemContext {
    pub index: usize,
    pub disabled: bool,
    pub item_ref: NodeRef<Div>,
    pub(crate) on_select: Option<Callback<()>>,
}

impl GetIndex<usize> for ContextMenuItemContext {
    fn get_index(&self) -> usize {
        self.index
    }
}

impl IsActive for ContextMenuItemContext {
    fn is_active(&self) -> bool {
        !self.disabled
    }
}

impl Focus for ContextMenuItemContext {
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
