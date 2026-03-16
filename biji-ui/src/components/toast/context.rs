use std::collections::HashSet;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

use leptos::prelude::*;

/// Where toasts are anchored in the viewport.
#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub enum ToastPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    #[default]
    BottomRight,
}

/// Controls which toasts pause their countdown timer when hovered.
#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub enum PauseOnHover {
    /// Only the toast being hovered pauses. Default.
    #[default]
    Single,
    /// All toasts pause whenever any toast is hovered.
    All,
    /// Hover has no effect on timers.
    Disable,
}

/// Data for a single toast notification.
#[derive(Clone)]
pub struct ToastItem {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    /// Arbitrary user-defined type string, emitted as `data-type`. E.g. `"success"`, `"error"`.
    pub toast_type: Option<String>,
    pub duration: Duration,
}

/// Context provided by [`Toaster`]. Access via `use_context::<ToasterContext>()`.
#[derive(Copy, Clone)]
pub struct ToasterContext {
    /// All active toasts (includes ones currently animating out).
    pub toasts: RwSignal<Vec<ToastItem>>,
    /// IDs of toasts currently in the exit-animation state.
    pub dismissed: RwSignal<HashSet<u32>>,
    /// Duration of the exit animation (used to delay DOM removal).
    pub hide_delay: Duration,
    pub(crate) default_duration: StoredValue<Duration>,
    pub(crate) next_id: StoredValue<Arc<AtomicU32>>,
    pub(crate) pause_on_hover: StoredValue<PauseOnHover>,
    /// Number of toasts currently being hovered. Used by `PauseOnHover::All`.
    pub hover_count: RwSignal<u32>,
}

impl ToasterContext {
    pub fn new(
        default_duration: Duration,
        hide_delay: Duration,
        pause_on_hover: PauseOnHover,
    ) -> Self {
        Self {
            toasts: RwSignal::new(vec![]),
            dismissed: RwSignal::new(HashSet::new()),
            hide_delay,
            default_duration: StoredValue::new(default_duration),
            next_id: StoredValue::new(Arc::new(AtomicU32::new(1))),
            pause_on_hover: StoredValue::new(pause_on_hover),
            hover_count: RwSignal::new(0u32),
        }
    }

    fn alloc_id(&self) -> u32 {
        self.next_id.with_value(|c| c.fetch_add(1, Ordering::Relaxed))
    }

    /// Add a toast. `toast_type` is an optional string emitted as `data-type` — use it
    /// to drive styling via `data-[type=your-value]:…` selectors. `duration` overrides the
    /// default when provided.
    pub fn add(
        &self,
        title: impl Into<String>,
        description: Option<String>,
        toast_type: Option<String>,
        duration: Option<Duration>,
    ) {
        let id = self.alloc_id();
        let duration = duration.unwrap_or_else(|| self.default_duration.get_value());
        self.toasts.update(|v| {
            v.push(ToastItem {
                id,
                title: title.into(),
                description,
                toast_type,
                duration,
            })
        });
    }

    /// Shorthand: add a toast with no type string.
    pub fn toast(&self, title: impl Into<String>) {
        self.add(title, None, None, None);
    }

    /// Dismiss a specific toast by id. Triggers the exit animation, then removes
    /// the toast from the list after `hide_delay`.
    pub fn dismiss(&self, id: u32) {
        self.dismissed.update(|s| {
            s.insert(id);
        });
        let toasts = self.toasts;
        let dismissed = self.dismissed;
        let hide_delay = self.hide_delay;
        set_timeout(
            move || {
                toasts.update(|v| v.retain(|t| t.id != id));
                dismissed.update(|s| {
                    s.remove(&id);
                });
            },
            hide_delay,
        );
    }

    /// Dismiss all active toasts.
    pub fn dismiss_all(&self) {
        let ids: Vec<u32> = self.toasts.with(|v| v.iter().map(|t| t.id).collect());
        for id in ids {
            self.dismiss(id);
        }
    }
}
