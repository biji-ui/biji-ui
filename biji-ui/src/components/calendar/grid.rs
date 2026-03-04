use chrono::{Datelike, Duration, Months, NaiveDate};
use leptos::{
    context::Provider,
    ev::{click, keydown, mouseenter, mouseleave},
    html,
    prelude::*,
};
use leptos_use::use_event_listener;

use super::{
    context::{CalendarContext, GridContext},
    types::{CalendarValue, CalendarView, SelectionType, WeekStartsOn},
};

/// Renders one month grid. `month_offset` controls which month relative to the
/// calendar's anchor month this grid shows (0 = anchor, 1 = next month, etc.).
#[component]
pub fn Grid(
    #[prop(default = 0usize)] month_offset: usize,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let cal_ctx = expect_context::<CalendarContext>();

    // Derive this grid's month reactively.
    let month_signal = RwSignal::new(compute_grid_month(
        cal_ctx.placeholder.get_untracked(),
        month_offset,
    ));

    // Keep the signal in sync when placeholder changes.
    Effect::new(move |_| {
        let new_month = compute_grid_month(cal_ctx.placeholder.get(), month_offset);
        month_signal.set(new_month);
    });

    let grid_ctx = GridContext {
        month: month_signal,
    };

    view! {
        <Provider value={grid_ctx}>
            <div class={class}>{children()}</div>
        </Provider>
    }
}

fn compute_grid_month(placeholder: NaiveDate, offset: usize) -> NaiveDate {
    placeholder
        .checked_add_months(Months::new(offset as u32))
        .and_then(|d| NaiveDate::from_ymd_opt(d.year(), d.month(), 1))
        .unwrap_or(placeholder)
}

/// Renders a row of 7 weekday header cells ordered by `week_starts_on`.
/// Automatically hidden when the calendar is in Month or Year view.
#[component]
pub fn GridHead(#[prop(into, optional)] class: String) -> impl IntoView {
    let cal_ctx = expect_context::<CalendarContext>();
    let labels = cal_ctx.week_starts_on.ordered_labels();
    let sv_class = StoredValue::new(class);

    view! {
        <div
            class={sv_class.get_value()}
            role="row"
            style={move || {
                if cal_ctx.view.get() != CalendarView::Day { "visibility:hidden" } else { "" }
            }}
        >
            {labels
                .into_iter()
                .map(|label| {
                    view! {
                        <div role="columnheader" aria-label={label}>
                            {label}
                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
}

/// Renders the calendar body. Switches between day / month / year grids based on
/// the current `CalendarView`.
///
/// Each week in the day grid is wrapped in a `role="row"` element that uses
/// `display: contents` so the parent's `grid grid-cols-7` (set via `day_class`)
/// still controls column layout. This satisfies the ARIA grid pattern
/// (`role="grid"` → `role="row"` → `role="gridcell"`) without requiring a
/// separate row class. Apply `grid grid-cols-4` via `month_class` / `year_class`
/// for the month and year pickers.
#[component]
pub fn GridBody(
    /// Class always applied to the body container.
    #[prop(into, optional)]
    class: String,
    /// Additional class applied when showing the day grid (view = Day).
    #[prop(into, optional)]
    day_class: String,
    /// Additional class applied when showing the month picker (view = Month).
    #[prop(into, optional)]
    month_class: String,
    /// Additional class applied when showing the year picker (view = Year).
    #[prop(into, optional)]
    year_class: String,
) -> impl IntoView {
    let cal_ctx = expect_context::<CalendarContext>();
    let grid_ctx = expect_context::<GridContext>();

    // Store strings as StoredValues so they can be used inside reactive closures.
    let sv_class = StoredValue::new(class);
    let sv_day = StoredValue::new(day_class);
    let sv_month = StoredValue::new(month_class);
    let sv_year = StoredValue::new(year_class);

    view! {
        <div
            role="grid"
            class={move || {
                let view_class = match cal_ctx.view.get() {
                    CalendarView::Day => sv_day.get_value(),
                    CalendarView::Month => sv_month.get_value(),
                    CalendarView::Year => sv_year.get_value(),
                };
                let base = sv_class.get_value();
                match (base.is_empty(), view_class.is_empty()) {
                    (true, _) => view_class,
                    (_, true) => base,
                    _ => format!("{} {}", base, view_class),
                }
            }}
        >
            {move || {
                let grid_month = grid_ctx.month.get();
                let is_primary = grid_month == cal_ctx.placeholder.get();
                match cal_ctx.view.get() {
                    CalendarView::Day => render_day_grid(cal_ctx, grid_ctx).into_any(),
                    CalendarView::Month if is_primary => {
                        render_month_grid(cal_ctx, grid_ctx).into_any()
                    }
                    CalendarView::Year if is_primary => {
                        render_year_grid(cal_ctx, grid_ctx).into_any()
                    }
                    _ => ().into_any(),
                }
            }}
        </div>
    }
}

fn render_day_grid(cal_ctx: CalendarContext, grid_ctx: GridContext) -> impl IntoView {
    let today = chrono::Local::now().date_naive();
    // Untracked: the outer GridBody closure already subscribes to grid_ctx.month.
    let month = grid_ctx.month.get_untracked();

    // Each week is wrapped in role="row" with display:contents so the parent's
    // CSS grid (grid-cols-7) still controls column layout, while satisfying the
    // ARIA grid hierarchy: role="grid" > role="row" > role="gridcell".
    compute_weeks(month, cal_ctx.week_starts_on)
        .into_iter()
        .map(|week| {
            let cells = week
                .into_iter()
                .map(|day_opt| match day_opt {
                    None => view! { <div aria-hidden="true"></div> }.into_any(),
                    Some(date) => render_day_cell(cal_ctx, grid_ctx, date, today).into_any(),
                })
                .collect_view();
            view! { <div role="row" style="display:contents">{cells}</div> }
        })
        .collect_view()
}

fn render_day_cell(
    cal_ctx: CalendarContext,
    grid_ctx: GridContext,
    date: NaiveDate,
    today: NaiveDate,
) -> impl IntoView {
    let is_today = date == today;
    let is_disabled = cal_ctx.date_is_disabled(date);
    let grid_month = grid_ctx.month;

    let btn_ref = NodeRef::<html::Button>::new();

    // Move DOM focus to this button whenever it becomes the keyboard focus target.
    // Tracks both `focused_date` and the NodeRef so it fires whether the signal
    // changes first or the element mounts first.
    Effect::new(move |_| {
        if cal_ctx.focused_date.get() == Some(date) {
            if let Some(el) = btn_ref.get() {
                let _ = el.focus();
            }
        }
    });

    let _ = use_event_listener(btn_ref, mouseenter, move |_| {
        cal_ctx.hover_date.set(Some(date));
    });
    let _ = use_event_listener(btn_ref, mouseleave, move |_| {
        cal_ctx.hover_date.set(None);
    });
    let _ = use_event_listener(btn_ref, click, move |_| {
        if !is_disabled {
            // Track the clicked date so Tab-out-and-back restores focus here.
            cal_ctx.focused_date.set(Some(date));
            handle_day_click(cal_ctx, date);
        }
    });
    let _ = use_event_listener(btn_ref, keydown, move |evt| {
        handle_day_keydown(cal_ctx, date, evt);
    });

    // Roving tabindex: only this cell is in the tab order when it is the focus
    // target. When nothing has been keyboard-navigated yet (focused_date = None),
    // the first day of the month acts as the tab entry point.
    let is_tab_target = move || match cal_ctx.focused_date.get() {
        Some(fd) => fd == date,
        None => date == grid_month.get(), // grid_month is always the 1st of the month
    };

    view! {
        <button
            node_ref={btn_ref}
            role="gridcell"
            disabled={is_disabled}
            tabindex={move || if !is_disabled && is_tab_target() { 0 } else { -1 }}
            data-today={is_today}
            data-outside-month={move || {
                let m = grid_month.get();
                date.month() != m.month() || date.year() != m.year()
            }}
            data-disabled={is_disabled}
            data-selected={move || cal_ctx.value.with(|v| v.contains(date))}
            data-in-range={move || {
                match cal_ctx.value.get() {
                    CalendarValue::Range { start: Some(s), end: Some(e) } => date > s && date < e,
                    _ => cal_ctx.date_in_hover_range(date),
                }
            }}
            data-range-start={move || {
                matches!(
                    cal_ctx.value.get(),
                    CalendarValue::Range { start: Some(s), .. }
                    if s == date
                )
            }}
            data-range-end={move || {
                matches!(
                    cal_ctx.value.get(),
                    CalendarValue::Range { end: Some(e), .. }
                    if e == date
                )
            }}
        >
            {date.day()}
        </button>
    }
}

fn handle_day_click(ctx: CalendarContext, date: NaiveDate) {
    let new_val = match ctx.selection_type {
        SelectionType::Single => {
            if matches!(ctx.value.get(), CalendarValue::Single(Some(d)) if d == date) {
                CalendarValue::Single(None)
            } else {
                CalendarValue::Single(Some(date))
            }
        }
        SelectionType::Multiple => {
            let mut dates = match ctx.value.get() {
                CalendarValue::Multiple(v) => v,
                _ => vec![],
            };
            if let Some(pos) = dates.iter().position(|&d| d == date) {
                dates.remove(pos);
            } else {
                dates.push(date);
            }
            CalendarValue::Multiple(dates)
        }
        SelectionType::Range => match ctx.value.get() {
            CalendarValue::Range { start: None, .. } => CalendarValue::Range {
                start: Some(date),
                end: None,
            },
            CalendarValue::Range {
                start: Some(s),
                end: None,
            } => {
                let (lo, hi) = if date >= s { (s, date) } else { (date, s) };
                CalendarValue::Range {
                    start: Some(lo),
                    end: Some(hi),
                }
            }
            CalendarValue::Range {
                start: Some(_),
                end: Some(_),
            } => CalendarValue::Range {
                start: Some(date),
                end: None,
            },
            _ => CalendarValue::Range {
                start: Some(date),
                end: None,
            },
        },
    };
    ctx.emit_change(new_val);
}

fn handle_day_keydown(ctx: CalendarContext, date: NaiveDate, evt: web_sys::KeyboardEvent) {
    let key = evt.key();
    let new_focus: Option<NaiveDate> = match key.as_str() {
        "ArrowLeft" => date.checked_sub_signed(Duration::days(1)),
        "ArrowRight" => date.checked_add_signed(Duration::days(1)),
        "ArrowUp" => date.checked_sub_signed(Duration::days(7)),
        "ArrowDown" => date.checked_add_signed(Duration::days(7)),
        "Home" => NaiveDate::from_ymd_opt(date.year(), date.month(), 1),
        "End" => last_day_of_month(date),
        "PageUp" => date.checked_sub_months(Months::new(1)),
        "PageDown" => date.checked_add_months(Months::new(1)),
        "Enter" | " " => {
            // Prevent the browser from synthesising a click event on the button,
            // which would call handle_day_click a second time and toggle it back.
            evt.prevent_default();
            if !ctx.date_is_disabled(date) {
                ctx.focused_date.set(Some(date));
                handle_day_click(ctx, date);
            }
            return;
        }
        _ => return,
    };

    if let Some(new_date) = new_focus {
        evt.prevent_default();
        ctx.focused_date.set(Some(new_date));

        // Scroll the view if new_date is outside the visible month window.
        let first_visible = ctx.placeholder.get();
        let last_visible = first_visible
            .checked_add_months(Months::new(ctx.months.saturating_sub(1) as u32))
            .and_then(last_day_of_month)
            .unwrap_or(first_visible);

        if new_date < first_visible {
            if let Some(anchor) = NaiveDate::from_ymd_opt(new_date.year(), new_date.month(), 1) {
                ctx.placeholder.set(anchor);
            }
        } else if new_date > last_visible {
            if let Some(anchor) = NaiveDate::from_ymd_opt(new_date.year(), new_date.month(), 1) {
                ctx.placeholder.set(anchor);
            }
        }
    }
}

fn render_month_grid(cal_ctx: CalendarContext, grid_ctx: GridContext) -> impl IntoView {
    // Untracked: outer closure subscribes.
    let year = grid_ctx.month.get_untracked().year();
    let today = chrono::Local::now().date_naive();
    let current_month = today.month();
    let current_year = today.year();

    const MONTH_NAMES: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    (1u32..=12)
        .map(|month_num| {
            let month_date = NaiveDate::from_ymd_opt(year, month_num, 1).expect("valid month");
            let is_current = month_num == current_month && year == current_year;

            let btn_ref = NodeRef::<html::Button>::new();

            // Move DOM focus to this button when it becomes the keyboard target.
            Effect::new(move |_| {
                if let Some(fd) = cal_ctx.focused_date.get() {
                    if fd.year() == year && fd.month() == month_num {
                        if let Some(el) = btn_ref.get() {
                            let _ = el.focus();
                        }
                    }
                }
            });

            // Roving tabindex: entry point when nothing is focused yet is the
            // anchor month (grid_ctx.month is always the 1st of that month).
            let is_tab_target = move || match cal_ctx.focused_date.get() {
                Some(fd) => fd.year() == year && fd.month() == month_num,
                None => month_date == grid_ctx.month.get(),
            };

            let _ = use_event_listener(btn_ref, click, move |_| {
                cal_ctx.focused_date.set(Some(month_date));
                cal_ctx.placeholder.set(month_date);
                cal_ctx.view.set(CalendarView::Day);
            });
            let _ = use_event_listener(btn_ref, keydown, move |evt| {
                handle_month_keydown(cal_ctx, month_date, evt);
            });

            view! {
                <button
                    node_ref={btn_ref}
                    tabindex={move || if is_tab_target() { 0 } else { -1 }}
                    data-current-month={is_current}
                    data-selected={move || {
                        match cal_ctx.value.get() {
                            CalendarValue::Single(Some(d)) => {
                                d.year() == year && d.month() == month_num
                            }
                            _ => false,
                        }
                    }}
                >
                    {MONTH_NAMES[(month_num - 1) as usize]}
                </button>
            }
        })
        .collect_view()
}

fn handle_month_keydown(ctx: CalendarContext, month_date: NaiveDate, evt: web_sys::KeyboardEvent) {
    let key = evt.key();
    let new_focus: Option<NaiveDate> = match key.as_str() {
        // Left/Right: ±1 month; wraps into adjacent year.
        "ArrowLeft" => month_date.checked_sub_months(Months::new(1)),
        "ArrowRight" => month_date.checked_add_months(Months::new(1)),
        // Up/Down: ±4 months (one row in the 4-column grid).
        "ArrowUp" => month_date.checked_sub_months(Months::new(4)),
        "ArrowDown" => month_date.checked_add_months(Months::new(4)),
        // Home/End: first/last month of the displayed year.
        "Home" => NaiveDate::from_ymd_opt(month_date.year(), 1, 1),
        "End" => NaiveDate::from_ymd_opt(month_date.year(), 12, 1),
        // PageUp/PageDown: same month, previous/next year.
        "PageUp" => month_date.with_year(month_date.year() - 1),
        "PageDown" => month_date.with_year(month_date.year() + 1),
        "Enter" | " " => {
            evt.prevent_default();
            ctx.focused_date.set(Some(month_date));
            ctx.placeholder.set(month_date);
            ctx.view.set(CalendarView::Day);
            return;
        }
        _ => return,
    };

    if let Some(new_date) = new_focus {
        evt.prevent_default();
        ctx.focused_date.set(Some(new_date));
        // If focus moved to a different year, scroll the grid to show it.
        if new_date.year() != month_date.year() {
            if let Some(anchor) = NaiveDate::from_ymd_opt(new_date.year(), new_date.month(), 1) {
                ctx.placeholder.set(anchor);
            }
        }
    }
}

fn render_year_grid(cal_ctx: CalendarContext, grid_ctx: GridContext) -> impl IntoView {
    let anchor_year = grid_ctx.month.get_untracked().year();
    let anchor_month = grid_ctx.month.get_untracked().month();
    let decade_start = (anchor_year / 12) * 12;
    let current_year = chrono::Local::now().date_naive().year();

    (0i32..12)
        .map(|i| {
            let year = decade_start + i;
            let is_current = year == current_year;

            let btn_ref = NodeRef::<html::Button>::new();

            // Move DOM focus to this button when it becomes the keyboard target.
            Effect::new(move |_| {
                if let Some(fd) = cal_ctx.focused_date.get() {
                    if fd.year() == year {
                        if let Some(el) = btn_ref.get() {
                            let _ = el.focus();
                        }
                    }
                }
            });

            // Roving tabindex: entry point when nothing is focused is the anchor year.
            let is_tab_target = move || match cal_ctx.focused_date.get() {
                Some(fd) => fd.year() == year,
                None => year == grid_ctx.month.get().year(),
            };

            let _ = use_event_listener(btn_ref, click, move |_| {
                if let Some(new_date) = NaiveDate::from_ymd_opt(year, anchor_month, 1) {
                    cal_ctx.focused_date.set(Some(new_date));
                    cal_ctx.placeholder.set(new_date);
                }
                cal_ctx.view.set(CalendarView::Month);
            });
            let _ = use_event_listener(btn_ref, keydown, move |evt| {
                handle_year_keydown(cal_ctx, year, anchor_month, decade_start, evt);
            });

            view! {
                <button
                    node_ref={btn_ref}
                    tabindex={move || if is_tab_target() { 0 } else { -1 }}
                    data-current-year={is_current}
                    data-selected={move || {
                        match cal_ctx.value.get() {
                            CalendarValue::Single(Some(d)) => d.year() == year,
                            _ => false,
                        }
                    }}
                >
                    {year}
                </button>
            }
        })
        .collect_view()
}

fn handle_year_keydown(
    ctx: CalendarContext,
    year: i32,
    anchor_month: u32,
    decade_start: i32,
    evt: web_sys::KeyboardEvent,
) {
    let key = evt.key();
    let new_year: Option<i32> = match key.as_str() {
        // Left/Right: ±1 year; wraps into adjacent decade.
        "ArrowLeft" => Some(year - 1),
        "ArrowRight" => Some(year + 1),
        // Up/Down: ±4 years (one row in the 4-column grid).
        "ArrowUp" => Some(year - 4),
        "ArrowDown" => Some(year + 4),
        // Home/End: first/last year in the current 12-year window.
        "Home" => Some(decade_start),
        "End" => Some(decade_start + 11),
        // PageUp/PageDown: previous/next 12-year window.
        "PageUp" => Some(year - 12),
        "PageDown" => Some(year + 12),
        "Enter" | " " => {
            evt.prevent_default();
            if let Some(new_date) = NaiveDate::from_ymd_opt(year, anchor_month, 1) {
                ctx.focused_date.set(Some(new_date));
                ctx.placeholder.set(new_date);
            }
            ctx.view.set(CalendarView::Month);
            return;
        }
        _ => return,
    };

    if let Some(new_y) = new_year {
        if let Some(new_date) = NaiveDate::from_ymd_opt(new_y, anchor_month, 1) {
            evt.prevent_default();
            ctx.focused_date.set(Some(new_date));
            // If focus moved outside the current decade window, scroll to show it.
            let new_decade = (new_y / 12) * 12;
            if new_decade != decade_start {
                ctx.placeholder.set(new_date);
            }
        }
    }
}

/// Returns a flat vector of weeks, each week being 7 `Option<NaiveDate>` cells.
/// `None` represents a padding cell (date outside the month).
fn compute_weeks(month: NaiveDate, week_starts_on: WeekStartsOn) -> Vec<Vec<Option<NaiveDate>>> {
    let first_day =
        NaiveDate::from_ymd_opt(month.year(), month.month(), 1).expect("valid first of month");
    let days_in_month = days_in_month(month);

    // Days from Sunday for the first of the month (0 = Sun, 1 = Mon, …)
    let first_day_from_sun = first_day.weekday().num_days_from_sunday();
    // Shift so that week_starts_on falls in column 0.
    let padding_before = ((first_day_from_sun + 7 - week_starts_on.offset()) % 7) as usize;

    let total_cells = padding_before + days_in_month;
    let num_rows = total_cells.div_ceil(7);

    // Always render 6 rows so the grid height is constant across all months,
    // preventing layout shift when navigating between months or switching views.
    let num_rows = num_rows.max(6);

    let mut weeks = Vec::with_capacity(num_rows);
    for row in 0..num_rows {
        let mut week = Vec::with_capacity(7);
        for col in 0..7 {
            let idx = row * 7 + col;
            let day_num = idx as isize - padding_before as isize + 1;
            if day_num < 1 || day_num > days_in_month as isize {
                week.push(None);
            } else {
                week.push(first_day.with_day(day_num as u32));
            }
        }
        weeks.push(week);
    }
    weeks
}

fn days_in_month(date: NaiveDate) -> usize {
    let (year, month) = if date.month() == 12 {
        (date.year() + 1, 1u32)
    } else {
        (date.year(), date.month() + 1)
    };
    match (
        NaiveDate::from_ymd_opt(date.year(), date.month(), 1),
        NaiveDate::from_ymd_opt(year, month, 1),
    ) {
        (Some(a), Some(b)) => (b - a).num_days() as usize,
        _ => 31,
    }
}

fn last_day_of_month(date: NaiveDate) -> Option<NaiveDate> {
    let (year, month) = if date.month() == 12 {
        (date.year() + 1, 1u32)
    } else {
        (date.year(), date.month() + 1)
    };
    NaiveDate::from_ymd_opt(year, month, 1)?.pred_opt()
}
