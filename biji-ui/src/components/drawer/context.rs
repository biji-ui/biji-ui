use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

use leptos::{
    html::{Button, Div},
    prelude::*,
};

static DRAWER_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn next_drawer_id() -> String {
    let id = DRAWER_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("biji-drawer-{id}")
}

/// Which edge the drawer panel slides in from.
#[derive(Copy, Clone, PartialEq, Default)]
pub enum DrawerSide {
    Top,
    #[default]
    Right,
    Bottom,
    Left,
}

impl DrawerSide {
    pub fn as_str(self) -> &'static str {
        match self {
            DrawerSide::Top => "top",
            DrawerSide::Right => "right",
            DrawerSide::Bottom => "bottom",
            DrawerSide::Left => "left",
        }
    }
}

#[derive(Copy, Clone)]
pub struct DrawerContext {
    pub trigger_ref: NodeRef<Button>,
    pub overlay_ref: NodeRef<Div>,
    pub content_ref: NodeRef<Div>,
    pub open: RwSignal<bool>,
    pub prevent_scroll: bool,
    pub hide_delay: Duration,
    pub side: DrawerSide,
    pub drawer_id: StoredValue<String>,
    pub title_id: StoredValue<String>,
    pub description_id: StoredValue<String>,
    pub on_open_change: Option<Callback<bool>>,
}

impl DrawerContext {
    pub fn open(&self) {
        self.open.set(true);
        if let Some(cb) = self.on_open_change {
            cb.run(true);
        }
    }

    pub fn close(&self) {
        self.open.set(false);
        if let Some(cb) = self.on_open_change {
            cb.run(false);
        }
    }

    pub fn toggle(&self) {
        if self.open.get() {
            self.close();
        } else {
            self.open();
        }
    }

    pub fn data_state(&self) -> &'static str {
        if self.open.get() { "open" } else { "closed" }
    }
}
