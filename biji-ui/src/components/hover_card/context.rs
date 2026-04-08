use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use leptos::{
    html::{Div, Span},
    leptos_dom::helpers::TimeoutHandle,
    prelude::*,
};

use crate::utils::positioning::{AvoidCollisions, Positioning};

static HOVER_CARD_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn next_hover_card_id() -> String {
    let id = HOVER_CARD_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("biji-hover-card-{id}")
}

#[derive(Copy, Clone)]
pub struct HoverCardState {
    pub open: RwSignal<bool>,
    pub data_state: Signal<&'static str>,
    pub(crate) trigger_ref: NodeRef<Span>,
    pub(crate) content_ref: NodeRef<Div>,
    pub(crate) open_delay: Duration,
    pub(crate) close_delay: Duration,
    pub(crate) hide_delay: Duration,
    pub(crate) positioning: Positioning,
    pub(crate) arrow_size: i32,
    pub(crate) hover_card_id: StoredValue<String>,
    pub(crate) avoid_collisions: AvoidCollisions,
    pub(crate) open_timer: StoredValue<Arc<Mutex<Option<TimeoutHandle>>>>,
    pub(crate) close_timer: StoredValue<Arc<Mutex<Option<TimeoutHandle>>>>,
}

impl HoverCardState {
    pub(crate) fn cancel_open_timer(&self) {
        self.open_timer.with_value(|arc| {
            if let Some(h) = arc.lock().unwrap().take() {
                h.clear();
            }
        });
    }

    pub(crate) fn cancel_close_timer(&self) {
        self.close_timer.with_value(|arc| {
            if let Some(h) = arc.lock().unwrap().take() {
                h.clear();
            }
        });
    }

    pub(crate) fn schedule_open(&self) {
        self.cancel_open_timer();
        let open_signal = self.open;
        let delay = self.open_delay;
        let handle = leptos::leptos_dom::helpers::set_timeout_with_handle(
            move || {
                if !open_signal.get_untracked() {
                    open_signal.set(true);
                }
            },
            delay,
        )
        .expect("hover card open timer");
        self.open_timer.with_value(|arc| {
            *arc.lock().unwrap() = Some(handle);
        });
    }

    pub(crate) fn schedule_close(&self) {
        self.cancel_close_timer();
        let open_signal = self.open;
        let delay = self.close_delay;
        let handle = leptos::leptos_dom::helpers::set_timeout_with_handle(
            move || {
                if open_signal.get_untracked() {
                    open_signal.set(false);
                }
            },
            delay,
        )
        .expect("hover card close timer");
        self.close_timer.with_value(|arc| {
            *arc.lock().unwrap() = Some(handle);
        });
    }

    pub(crate) fn close_immediate(&self) {
        self.cancel_open_timer();
        self.cancel_close_timer();
        if self.open.get_untracked() {
            self.open.set(false);
        }
    }
}
