use chrono::{Datelike, Months, NaiveDate};
use leptos::{ev::click, html, prelude::*};
use leptos_use::use_event_listener;

use super::{
    context::CalendarState,
    types::CalendarView,
};

/// Wrapper element for the calendar header row. Purely structural.
#[component]
pub fn Header(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class={class}>{children()}</div>
    }
}

/// Clickable button that displays the current month/year and cycles Day → Month → Year on click.
#[component]
pub fn Heading(#[prop(into, optional)] class: String) -> impl IntoView {
    let ctx = expect_context::<CalendarState>();
    let btn_ref = NodeRef::<html::Button>::new();

    let _ = use_event_listener(btn_ref, click, move |_| {
        let next = match ctx.view.get() {
            CalendarView::Day => CalendarView::Month,
            CalendarView::Month => CalendarView::Year,
            CalendarView::Year => CalendarView::Day,
        };
        ctx.view.set(next);
    });

    let label = move || {
        let placeholder = ctx.placeholder.get();
        match ctx.view.get() {
            CalendarView::Day => {
                let months = ctx.months;
                if months > 1 {
                    let end = placeholder
                        .checked_add_months(Months::new(months as u32 - 1))
                        .unwrap_or(placeholder);
                    if placeholder.year() == end.year() {
                        format!(
                            "{} \u{2013} {} {}",
                            placeholder.format("%B"),
                            end.format("%B"),
                            placeholder.year()
                        )
                    } else {
                        format!(
                            "{} {} \u{2013} {} {}",
                            placeholder.format("%B"),
                            placeholder.year(),
                            end.format("%B"),
                            end.year()
                        )
                    }
                } else {
                    format!("{} {}", placeholder.format("%B"), placeholder.year())
                }
            }
            CalendarView::Month => {
                format!("{}", placeholder.year())
            }
            CalendarView::Year => {
                let year = placeholder.year();
                let decade_start = (year / 10) * 10;
                format!("{} \u{2013} {}", decade_start, decade_start + 9)
            }
        }
    };

    view! {
        <button node_ref={btn_ref} class={class}>
            {label}
        </button>
    }
}

/// Navigate to the previous month / year / decade depending on the current view.
#[component]
pub fn PrevButton(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let ctx = expect_context::<CalendarState>();
    let btn_ref = NodeRef::<html::Button>::new();

    let _ = use_event_listener(btn_ref, click, move |_| {
        navigate(ctx, Direction::Prev);
    });

    view! {
        <button node_ref={btn_ref} class={class}>
            {children()}
        </button>
    }
}

/// Navigate to the next month / year / decade depending on the current view.
#[component]
pub fn NextButton(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let ctx = expect_context::<CalendarState>();
    let btn_ref = NodeRef::<html::Button>::new();

    let _ = use_event_listener(btn_ref, click, move |_| {
        navigate(ctx, Direction::Next);
    });

    view! {
        <button node_ref={btn_ref} class={class}>
            {children()}
        </button>
    }
}

enum Direction {
    Prev,
    Next,
}

fn navigate(ctx: CalendarState, direction: Direction) {
    let current = ctx.placeholder.get();
    let new_date = match (&direction, ctx.view.get()) {
        (Direction::Prev, CalendarView::Day) => {
            current.checked_sub_months(Months::new(1))
        }
        (Direction::Next, CalendarView::Day) => {
            current.checked_add_months(Months::new(1))
        }
        (Direction::Prev, CalendarView::Month) => {
            current.with_year(current.year() - 1)
        }
        (Direction::Next, CalendarView::Month) => {
            current.with_year(current.year() + 1)
        }
        (Direction::Prev, CalendarView::Year) => {
            current.with_year(current.year() - 10)
        }
        (Direction::Next, CalendarView::Year) => {
            current.with_year(current.year() + 10)
        }
    };

    if let Some(d) = new_date {
        let clamped = NaiveDate::from_ymd_opt(d.year(), d.month(), 1).unwrap_or(d);
        ctx.placeholder.set(clamped);
    }
}
