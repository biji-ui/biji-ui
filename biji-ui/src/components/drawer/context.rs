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
pub struct DrawerState {
    pub open: RwSignal<bool>,
    pub data_state: Signal<&'static str>,
    pub side: DrawerSide,
    pub(crate) trigger_ref: NodeRef<Button>,
    pub(crate) overlay_ref: NodeRef<Div>,
    pub(crate) content_ref: NodeRef<Div>,
    pub(crate) prevent_scroll: bool,
    pub(crate) hide_delay: Duration,
    pub(crate) drawer_id: StoredValue<String>,
    pub(crate) title_id: StoredValue<String>,
    pub(crate) description_id: StoredValue<String>,
    pub(crate) on_open_change: Option<Callback<bool>>,
}

impl DrawerState {
    pub(crate) fn open(&self) {
        self.open.set(true);
        if let Some(cb) = self.on_open_change {
            cb.run(true);
        }
    }

    pub(crate) fn close(&self) {
        self.open.set(false);
        if let Some(cb) = self.on_open_change {
            cb.run(false);
        }
    }

    pub(crate) fn toggle(&self) {
        if self.open.get() {
            self.close();
        } else {
            self.open();
        }
    }
}
