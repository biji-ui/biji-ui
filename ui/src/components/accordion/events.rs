use leptos::{
    ev::{blur, click, focus, keydown},
    *,
};
use leptos_use::use_event_listener;

use super::contexts::{AccordionContext, AccordionItemContext};

#[component]
pub fn RootEvents(children: Children) -> impl IntoView {
    let ctx = expect_context::<AccordionContext>();

    let _ = use_event_listener(ctx.accordion_ref, keydown, move |evt| {
        if evt.key() == "ArrowDown" {
            ctx.focus_next();
        } else if evt.key() == "ArrowUp" {
            ctx.focus_previous();
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
