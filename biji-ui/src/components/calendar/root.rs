use chrono::{Datelike, NaiveDate};
use leptos::{context::Provider, prelude::*};

use super::{
    context::CalendarState,
    types::{CalendarValue, CalendarView, SelectionType, WeekStartsOn},
};

fn today() -> NaiveDate {
    chrono::Local::now().date_naive()
}

fn build_state(
    value: Option<RwSignal<CalendarValue>>,
    default_value: Option<CalendarValue>,
    selection_type: SelectionType,
    placeholder: Option<NaiveDate>,
    months: usize,
    min_date: Option<NaiveDate>,
    max_date: Option<NaiveDate>,
    is_date_disabled: Option<Box<dyn Fn(NaiveDate) -> bool + Send + Sync>>,
    week_starts_on: WeekStartsOn,
) -> CalendarState {
    let value_signal = value.unwrap_or_else(|| {
        let init = default_value.unwrap_or_else(|| CalendarValue::default_for(selection_type));
        RwSignal::new(init)
    });
    let anchor = {
        let d = placeholder.unwrap_or_else(today);
        NaiveDate::from_ymd_opt(d.year(), d.month(), 1).unwrap_or(d)
    };
    CalendarState {
        placeholder: RwSignal::new(anchor),
        view: RwSignal::new(CalendarView::Day),
        value: value_signal,
        selection_type,
        months,
        min_date,
        max_date,
        is_date_disabled: is_date_disabled.map(StoredValue::new),
        week_starts_on,
        hover_date: RwSignal::new(None),
        focused_date: RwSignal::new(None),
    }
}

/// Returns the [`CalendarState`] from the nearest [`Root`] or [`RootWith`] ancestor.
///
/// Call this inside any child component that needs access to calendar state.
pub fn use_calendar() -> CalendarState {
    expect_context::<CalendarState>()
}

/// The render-prop variant of [`Root`]. Use this when you need access to [`CalendarState`]
/// directly inside the children via the `let:` binding.
///
/// ```rust
/// <calendar::RootWith let:cal>
///     <p class="text-sm mb-2">
///         {move || cal.value.with(|v| match v {
///             CalendarValue::Single(Some(d)) => d.format("%B %-d, %Y").to_string(),
///             _ => "No date selected".to_string(),
///         })}
///     </p>
///     <calendar::Header ...>/* nav */</calendar::Header>
///     <calendar::Grid><calendar::GridHead /><calendar::GridBody ... /></calendar::Grid>
/// </calendar::RootWith>
/// ```
///
/// The `cal: CalendarState` binding is `Copy`, so it can be passed to child components as a prop.
#[component]
pub fn RootWith<IV: IntoView + 'static>(
    children: impl Fn(CalendarState) -> IV + Send + Sync + 'static,
    #[prop(into, optional)] class: String,
    /// Controlled external signal. When provided, the calendar writes into this signal on every
    /// selection change so the caller can react via effects or `RootWith let:`.
    #[prop(into, default = None)]
    value: Option<RwSignal<CalendarValue>>,
    /// Initial value for uncontrolled mode.
    #[prop(into, default = None)]
    default_value: Option<CalendarValue>,
    #[prop(default = SelectionType::Single)] selection_type: SelectionType,
    /// The month to display initially. Defaults to the current month.
    #[prop(into, default = None)]
    placeholder: Option<NaiveDate>,
    #[prop(default = 1usize)] months: usize,
    #[prop(into, default = None)] min_date: Option<NaiveDate>,
    #[prop(into, default = None)] max_date: Option<NaiveDate>,
    #[prop(into, default = None)]
    is_date_disabled: Option<Box<dyn Fn(NaiveDate) -> bool + Send + Sync>>,
    #[prop(default = WeekStartsOn::Sunday)] week_starts_on: WeekStartsOn,
) -> impl IntoView {
    let state = build_state(
        value,
        default_value,
        selection_type,
        placeholder,
        months,
        min_date,
        max_date,
        is_date_disabled,
        week_starts_on,
    );

    view! {
        <Provider value={state}>
            <div
                class={class}
                data-view={move || match state.view.get() {
                    CalendarView::Day   => "day",
                    CalendarView::Month => "month",
                    CalendarView::Year  => "year",
                }}
            >
                {children(state)}
            </div>
        </Provider>
    }
}

/// The standard calendar root. Renders a wrapper `<div>` with a `data-view` attribute and
/// provides [`CalendarState`] to all descendants via context.
///
/// Use [`RootWith`] instead when you need to access [`CalendarState`] inline via `let:cal`.
#[component]
pub fn Root(
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
    #[prop(into, default = None)] value: Option<RwSignal<CalendarValue>>,
    #[prop(into, default = None)] default_value: Option<CalendarValue>,
    #[prop(default = SelectionType::Single)] selection_type: SelectionType,
    #[prop(into, default = None)] placeholder: Option<NaiveDate>,
    #[prop(default = 1usize)] months: usize,
    #[prop(into, default = None)] min_date: Option<NaiveDate>,
    #[prop(into, default = None)] max_date: Option<NaiveDate>,
    #[prop(optional)] is_date_disabled: Option<Box<dyn Fn(NaiveDate) -> bool + Send + Sync>>,
    #[prop(default = WeekStartsOn::Sunday)] week_starts_on: WeekStartsOn,
) -> impl IntoView {
    view! {
        <RootWith
            value={value}
            default_value={default_value}
            selection_type={selection_type}
            placeholder={placeholder}
            months={months}
            min_date={min_date}
            max_date={max_date}
            is_date_disabled={is_date_disabled}
            week_starts_on={week_starts_on}
            class={class}
            let:_
        >
            {children()}
        </RootWith>
    }
}
