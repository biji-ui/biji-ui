use leptos::{
    ev::{blur, click, focus, keydown},
    *,
};
use leptos_use::use_event_listener;

use super::{item::AccordionItemContext, root::AccordionContext};

#[component]
pub fn RootEvents(children: Children) -> impl IntoView {
    let ctx = expect_context::<AccordionContext>();

    let focus_next = move || {
        if let Some(current_focus) = ctx.current_focus.get() {
            let items = ctx.items.get();
            if let Some(next) = items.get(current_focus + 1) {
                if let Some(next_ref) = next.trigger_ref.get() {
                    let _ = next_ref.focus();
                }
            } else {
                if let Some(first) = items.first() {
                    if let Some(first_ref) = first.trigger_ref.get() {
                        let _ = first_ref.focus();
                    }
                }
            }
        }
    };

    let focus_previous = move || {
        if let Some(current_focus) = ctx.current_focus.get() {
            let items = ctx.items.get();
            if current_focus > 0 {
                if let Some(next) = items.get(current_focus - 1) {
                    if let Some(next_ref) = next.trigger_ref.get() {
                        let _ = next_ref.focus();
                    }
                }
            } else {
                if let Some(last) = items.last() {
                    if let Some(last_ref) = last.trigger_ref.get() {
                        let _ = last_ref.focus();
                    }
                }
            }
        }
    };

    let _ = use_event_listener(ctx.accordion_ref, keydown, move |evt| {
        if evt.key() == "ArrowDown" {
            focus_next();
        } else if evt.key() == "ArrowUp" {
            focus_previous();
        }
    });

    children()
}

#[component]
pub fn ItemEvents(children: Children) -> impl IntoView {
    let accordion_ctx = expect_context::<AccordionContext>();
    let ctx = expect_context::<AccordionItemContext>();

    let _ = use_event_listener(ctx.trigger_ref, focus, move |_| {
        accordion_ctx.current_focus.set(Some(ctx.index));
    });

    let _ = use_event_listener(ctx.trigger_ref, blur, move |_| {
        if let Some(current_focus) = accordion_ctx.current_focus.get() {
            if current_focus == ctx.index {
                accordion_ctx.current_focus.set(None);
            }
        }
    });

    let _ = use_event_listener(ctx.trigger_ref, click, move |_| {
        ctx.open.set(!ctx.open.get());
    });

    children()
}
