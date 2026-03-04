use chrono::{Datelike, NaiveDate};
use leptos::{context::Provider, prelude::*};

use super::{
    context::CalendarContext,
    types::{CalendarValue, CalendarView, SelectionType, WeekStartsOn},
};

fn today() -> NaiveDate {
    chrono::Local::now().date_naive()
}

#[component]
pub fn Root(
    #[prop(into, optional)] class: String,
    /// Controlled external signal. When provided, the calendar mirrors its initial value and
    /// calls `on_change` on every mutation so the caller can sync it back.
    #[prop(optional)]
    value: Option<RwSignal<CalendarValue>>,
    /// Initial value for uncontrolled mode.
    #[prop(optional)]
    default_value: Option<CalendarValue>,
    #[prop(default = SelectionType::Single)] selection_type: SelectionType,
    /// The month to display initially. Defaults to the current month.
    /// Pass this explicitly from the server to avoid SSR timezone-boundary hydration mismatches.
    #[prop(optional)]
    placeholder: Option<NaiveDate>,
    /// Number of month grids to render side-by-side.
    #[prop(default = 1usize)]
    months: usize,
    #[prop(optional)] min_date: Option<NaiveDate>,
    #[prop(optional)] max_date: Option<NaiveDate>,
    /// Return `true` to disable a specific date. Must be `Send + Sync`.
    #[prop(optional)]
    is_date_disabled: Option<Box<dyn Fn(NaiveDate) -> bool + Send + Sync>>,
    #[prop(default = WeekStartsOn::Sunday)] week_starts_on: WeekStartsOn,
    #[prop(optional)] on_change: Option<Callback<CalendarValue>>,
    children: Children,
) -> impl IntoView {
    // Determine the initial value (controlled takes precedence over default_value).
    let initial_value = value
        .map(|s| s.get_untracked())
        .or(default_value)
        .unwrap_or_else(|| CalendarValue::default_for(selection_type));

    // Determine anchor month, clamped to the first day of that month.
    let anchor = {
        let d = placeholder.unwrap_or_else(today);
        NaiveDate::from_ymd_opt(d.year(), d.month(), 1).unwrap_or(d)
    };

    let ctx = CalendarContext {
        placeholder: RwSignal::new(anchor),
        view: RwSignal::new(CalendarView::Day),
        value: RwSignal::new(initial_value),
        selection_type,
        months,
        min_date,
        max_date,
        is_date_disabled: is_date_disabled.map(StoredValue::new),
        week_starts_on,
        hover_date: RwSignal::new(None),
        focused_date: RwSignal::new(None),
        on_change,
    };

    view! {
        <Provider value={ctx}>
            <div
                class={class}
                data-view={move || match ctx.view.get() {
                    CalendarView::Day   => "day",
                    CalendarView::Month => "month",
                    CalendarView::Year  => "year",
                }}
            >
                {children()}
            </div>
        </Provider>
    }
}
