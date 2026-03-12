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
pub struct HoverCardContext {
    /// Wraps the trigger children — used for bounding rect + event listeners.
    pub trigger_ref: NodeRef<Span>,
    pub content_ref: NodeRef<Div>,
    pub open: RwSignal<bool>,
    /// Delay before showing the card on hover.
    pub open_delay: Duration,
    /// Delay before hiding the card after the pointer leaves.
    pub close_delay: Duration,
    /// Animation unmount delay — should match your CSS transition duration.
    pub hide_delay: Duration,
    pub positioning: Positioning,
    pub arrow_size: i32,
    pub hover_card_id: StoredValue<String>,
    pub avoid_collisions: AvoidCollisions,
    pub on_open_change: Option<Callback<bool>>,
    pub(crate) open_timer: StoredValue<Arc<Mutex<Option<TimeoutHandle>>>>,
    pub(crate) close_timer: StoredValue<Arc<Mutex<Option<TimeoutHandle>>>>,
}

impl HoverCardContext {
    pub fn cancel_open_timer(&self) {
        self.open_timer.with_value(|arc| {
            if let Some(h) = arc.lock().unwrap().take() {
                h.clear();
            }
        });
    }

    pub fn cancel_close_timer(&self) {
        self.close_timer.with_value(|arc| {
            if let Some(h) = arc.lock().unwrap().take() {
                h.clear();
            }
        });
    }

    /// Schedule open after `open_delay`, cancelling any existing open timer.
    pub fn schedule_open(&self) {
        self.cancel_open_timer();
        let open_signal = self.open;
        let on_change = self.on_open_change;
        let delay = self.open_delay;
        let handle = leptos::leptos_dom::helpers::set_timeout_with_handle(
            move || {
                open_signal.set(true);
                if let Some(cb) = on_change {
                    cb.run(true);
                }
            },
            delay,
        )
        .expect("hover card open timer");
        self.open_timer.with_value(|arc| {
            *arc.lock().unwrap() = Some(handle);
        });
    }

    /// Schedule close after `close_delay`, cancelling any existing close timer.
    pub fn schedule_close(&self) {
        self.cancel_close_timer();
        let open_signal = self.open;
        let on_change = self.on_open_change;
        let delay = self.close_delay;
        let handle = leptos::leptos_dom::helpers::set_timeout_with_handle(
            move || {
                open_signal.set(false);
                if let Some(cb) = on_change {
                    cb.run(false);
                }
            },
            delay,
        )
        .expect("hover card close timer");
        self.close_timer.with_value(|arc| {
            *arc.lock().unwrap() = Some(handle);
        });
    }

    /// Immediately close, cancelling any pending timers.
    pub fn close_immediate(&self) {
        self.cancel_open_timer();
        self.cancel_close_timer();
        self.open.set(false);
        if let Some(cb) = self.on_open_change {
            cb.run(false);
        }
    }

    pub fn data_state(&self) -> &'static str {
        if self.open.get() { "open" } else { "closed" }
    }
}
