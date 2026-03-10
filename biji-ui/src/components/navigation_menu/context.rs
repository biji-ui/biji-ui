use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use leptos::{
    html::{Button, Div},
    leptos_dom::helpers::TimeoutHandle,
    prelude::*,
};

use crate::{
    items::{
        FilterActiveItems, Focus, GetIndex, IsActive, ManageFocus, NavigateItems, filter_active,
        next_item, previous_item,
    },
    utils::positioning::{AvoidCollisions, Positioning},
};

#[derive(Copy, Clone)]
pub struct NavMenuContext {
    /// Value of the currently-open item, or `None` when all items are closed.
    pub open_value: RwSignal<Option<String>>,
    /// Roving-tabindex focus index (mirrors Tabs pattern).
    pub item_focus: RwSignal<Option<usize>>,
    pub items: RwSignal<HashMap<usize, NavMenuItemContext>>,
    pub positioning: Positioning,
    pub avoid_collisions: AvoidCollisions,
    /// Hover-close delay: how long after leaving a trigger/content before closing.
    pub close_delay: Duration,
    /// Animation unmount delay passed to `CustomAnimatedShow`.
    pub hide_delay: Duration,
    /// Shared close timer.  `Arc<Mutex<>>` so every `Copy` clone of the context
    /// can cancel / restart the same underlying handle — same pattern used in
    /// `CustomAnimatedShow` to guard against stale arena-slot reuse.
    pub close_timer: StoredValue<Arc<Mutex<Option<TimeoutHandle>>>>,
    pub root_id: usize,
    pub(crate) next_id: StoredValue<AtomicUsize>,
}

impl NavMenuContext {
    pub fn next_index(&self) -> usize {
        self.next_id.with_value(|c| c.fetch_add(1, Ordering::Relaxed))
    }

    pub fn upsert_item(&self, index: usize, item: NavMenuItemContext) {
        self.items.update(|m| {
            *m.entry(index).or_insert(item) = item;
        });
    }

    pub fn remove_item(&self, index: usize) {
        self.items.update(|m| {
            m.remove(&index);
        });
    }

    /// Cancel any in-flight close timer.
    pub fn cancel_close_timer(&self) {
        self.close_timer.with_value(|arc| {
            if let Some(h) = arc.lock().unwrap().take() {
                h.clear();
            }
        });
    }

    /// Schedule `open_value → None` after `close_delay`.  Cancels any existing timer first.
    pub fn schedule_close(&self) {
        self.cancel_close_timer();
        let open_value = self.open_value;
        let delay = self.close_delay;
        let handle = leptos::leptos_dom::helpers::set_timeout_with_handle(
            move || {
                open_value.set(None);
            },
            delay,
        )
        .expect("nav menu close timer");
        self.close_timer.with_value(|arc| {
            *arc.lock().unwrap() = Some(handle);
        });
    }

    /// Immediately open an item, cancelling any pending close.
    pub fn open(&self, value: String) {
        self.cancel_close_timer();
        self.open_value.set(Some(value));
    }

    /// Immediately close all items, cancelling any pending close timer.
    pub fn close_immediate(&self) {
        self.cancel_close_timer();
        self.open_value.set(None);
    }

    pub fn is_open(&self, value: &str) -> bool {
        self.open_value.get().as_deref() == Some(value)
    }

    pub fn any_open(&self) -> bool {
        self.open_value.get().is_some()
    }
}

impl FilterActiveItems<NavMenuItemContext> for NavMenuContext {
    fn filter_active_items(&self) -> Vec<NavMenuItemContext> {
        filter_active(self.items.get())
    }
}

impl ManageFocus for NavMenuContext {
    fn set_focus(&self, index: Option<usize>) {
        self.item_focus.set(index);
    }

    fn item_in_focus(&self, index: usize) -> bool {
        self.item_focus.get() == Some(index)
    }
}

impl NavigateItems<NavMenuItemContext> for NavMenuContext {
    fn navigate_first_item(&self) -> Option<NavMenuItemContext> {
        self.filter_active_items().into_iter().next()
    }

    fn navigate_last_item(&self) -> Option<NavMenuItemContext> {
        self.filter_active_items().into_iter().last()
    }

    fn navigate_next_item(&self) -> Option<NavMenuItemContext> {
        let items = self.filter_active_items();
        next_item(items, self.item_focus.get(), true)
    }

    fn navigate_previous_item(&self) -> Option<NavMenuItemContext> {
        let items = self.filter_active_items();
        previous_item(items, self.item_focus.get(), true)
    }
}

#[derive(Copy, Clone)]
pub struct NavMenuItemContext {
    pub index: usize,
    pub value: StoredValue<String>,
    pub disabled: bool,
    pub trigger_ref: NodeRef<Button>,
    pub content_ref: NodeRef<Div>,
    pub trigger_id: StoredValue<String>,
    pub content_id: StoredValue<String>,
    /// Set to `true` by the `Content` sub-component on mount.
    /// Controls whether `aria-controls` / `aria-haspopup` are emitted on the trigger.
    pub has_content: RwSignal<bool>,
}

impl GetIndex<usize> for NavMenuItemContext {
    fn get_index(&self) -> usize {
        self.index
    }
}

impl IsActive for NavMenuItemContext {
    fn is_active(&self) -> bool {
        !self.disabled
    }
}

impl Focus for NavMenuItemContext {
    fn focus(&self) -> bool {
        let Some(el) = self.trigger_ref.get() else {
            return false;
        };
        let _ = el.focus();
        true
    }
}
