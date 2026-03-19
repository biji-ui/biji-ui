use chrono::NaiveDate;
use leptos::prelude::*;

use super::types::{CalendarValue, CalendarView, SelectionType, WeekStartsOn};

/// Reactive state for a calendar. Available via [`use_calendar`](super::root::use_calendar)
/// or the `let:` binding on [`RootWith`](super::root::RootWith).
///
/// All fields are `Copy`, so it is safe to pass this struct to child components as a prop.
#[derive(Copy, Clone)]
pub struct CalendarState {
    /// First day of the first displayed month (anchor for navigation).
    pub placeholder: RwSignal<NaiveDate>,
    /// Current view mode.
    pub view: RwSignal<CalendarView>,
    /// The current selection value.
    pub value: RwSignal<CalendarValue>,
    /// Controls toggle behaviour (Single / Multiple / Range).
    pub selection_type: SelectionType,
    /// How many month grids are displayed simultaneously.
    pub months: usize,
    pub min_date: Option<NaiveDate>,
    pub max_date: Option<NaiveDate>,
    pub week_starts_on: WeekStartsOn,
    /// Optional user-supplied predicate. `Send + Sync` required by Leptos 0.8 arena allocator.
    pub(crate) is_date_disabled:
        Option<StoredValue<Box<dyn Fn(NaiveDate) -> bool + Send + Sync>>>,
    /// Hovered date — set while the pointer is over a day cell.
    pub hover_date: RwSignal<Option<NaiveDate>>,
    /// Currently keyboard-focused date.
    pub(crate) focused_date: RwSignal<Option<NaiveDate>>,
    /// Called whenever the selection changes.
    pub(crate) on_change: Option<Callback<CalendarValue>>,
}

impl CalendarState {
    /// Returns true if the given date should be rendered as disabled.
    pub(crate) fn date_is_disabled(&self, date: NaiveDate) -> bool {
        if let Some(min) = self.min_date {
            if date < min {
                return true;
            }
        }
        if let Some(max) = self.max_date {
            if date > max {
                return true;
            }
        }
        if let Some(pred) = self.is_date_disabled {
            return pred.with_value(|f| f(date));
        }
        false
    }

    /// Returns true if `date` falls in the hover-preview range (Range mode, start set, end not yet set).
    pub(crate) fn date_in_hover_range(&self, date: NaiveDate) -> bool {
        if self.selection_type != SelectionType::Range {
            return false;
        }
        let val = self.value.get();
        let CalendarValue::Range {
            start: Some(start),
            end: None,
        } = val
        else {
            return false;
        };
        let Some(hover) = self.hover_date.get() else {
            return false;
        };
        let (lo, hi) = if hover >= start {
            (start, hover)
        } else {
            (hover, start)
        };
        date > lo && date < hi
    }

    /// Update the selection value and fire the `on_change` callback.
    pub(crate) fn emit_change(&self, new_val: CalendarValue) {
        self.value.set(new_val.clone());
        if let Some(cb) = self.on_change {
            cb.run(new_val);
        }
    }
}

/// Per-Grid context: holds the specific month this grid renders.
/// Uses `RwSignal` so it stays reactive when `CalendarState.placeholder` changes.
#[derive(Copy, Clone)]
pub struct GridContext {
    pub month: RwSignal<NaiveDate>,
}
