use std::collections::HashMap;

use leptos::{
    ev::{blur, click, focus, keydown},
    *,
};
use leptos_use::{on_click_outside, use_event_listener};
use wasm_bindgen::JsCast;
use web_sys::{HtmlAnchorElement, HtmlButtonElement};

use super::contexts::{MenuContext, MenuItemContext};

#[component]
pub fn RootEvents(children: Children) -> impl IntoView {
    let ctx = expect_context::<MenuContext>();

    let _ = use_event_listener(ctx.trigger_ref, click, move |_| {
        if ctx.open.get() {
            ctx.close();
        } else {
            ctx.open();
        }
    });

    children()
}

#[component]
pub fn SubRootEvents(
    children: Children,
    parent_index: usize,
    parent_ctx: MenuContext,
) -> impl IntoView {
    let ctx = expect_context::<MenuContext>();

    let open = move || {
        if !ctx.open.get() {
            parent_ctx.in_focus.set(false);
            ctx.open();
            ctx.focus_next();
        }
    };

    let close = move || {
        if ctx.open.get() {
            ctx.close();
            parent_ctx.in_focus.set(true);
            parent_ctx.focus_item(parent_index);
        }
    };

    let _ = use_event_listener(ctx.trigger_ref, click, move |_| {
        if ctx.open.get() {
            close();
        } else {
            open();
        }
    });

    let _ = use_event_listener(ctx.menu_ref, keydown, move |evt| {
        if evt.key() == "ArrowLeft" {
            close();
        } else if evt.key() == "ArrowRight" {
            open();
        } else if evt.key() == "Enter" {
            open();
        }
    });

    children()
}

#[component]
pub fn ContentEvents(children: Children) -> impl IntoView {
    let ctx = expect_context::<MenuContext>();

    let close = move || {
        ctx.open.set(false);
        if let Some(trigger_ref) = ctx.trigger_ref.get() {
            let _ = trigger_ref.focus();
            ctx.current_focus.set(None);
            ctx.items.set(HashMap::new());
        }
    };

    let _ = use_event_listener(ctx.menu_ref, keydown, move |evt| {
        if evt.key() == "Escape" {
            close();
        } else if evt.key() == "ArrowDown" {
            if ctx.in_focus.get() {
                ctx.focus_next();
            }
        } else if evt.key() == "ArrowUp" {
            if ctx.in_focus.get() {
                ctx.focus_previous();
            }
        }
    });

    if ctx.close_on_outside_click {
        let _ = on_click_outside(ctx.menu_ref, move |_| {
            close();
        });
    }

    children()
}

#[component]
pub fn ItemEvents(children: Children) -> impl IntoView {
    let menu_ctx = expect_context::<MenuContext>();
    let ctx = expect_context::<MenuItemContext>();

    let _ = use_event_listener(ctx.trigger_ref, focus, move |_| {
        menu_ctx.current_focus.set(Some(ctx.index));
    });

    let _ = use_event_listener(ctx.trigger_ref, keydown, move |evt| {
        if evt.key() == "Enter" {
            if let Some(trigger_ref) = ctx.trigger_ref.get() {
                if let Some(child) = trigger_ref.children().get_with_index(0) {
                    if let Ok(child) = child.clone().dyn_into::<HtmlButtonElement>() {
                        let _ = child.click();
                    } else if let Ok(child) = child.dyn_into::<HtmlAnchorElement>() {
                        let _ = child.click();
                    }
                }
            }
        }
    });

    let _ = use_event_listener(ctx.trigger_ref, blur, move |_| {
        if let Some(current_focus) = menu_ctx.current_focus.get() {
            if current_focus == ctx.index {
                menu_ctx.current_focus.set(None);
            }
        }
    });

    children()
}
