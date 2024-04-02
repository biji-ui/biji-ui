use std::collections::HashMap;

use leptos::{
    ev::{blur, click, focus, keydown},
    *,
};
use leptos_use::{on_click_outside, use_event_listener};
use wasm_bindgen::JsCast;
use web_sys::{HtmlAnchorElement, HtmlButtonElement};

use super::{item::MenuItemContext, root::MenuContext};

fn filter_active_menu_items(items: HashMap<usize, MenuItemContext>) -> Vec<MenuItemContext> {
    let mut items = items
        .values()
        .filter(|item| !item.disabled)
        .cloned()
        .collect::<Vec<MenuItemContext>>();

    items.sort_by(|a, b| a.index.cmp(&b.index));

    items
}

fn next_active_menu_item(
    items: HashMap<usize, MenuItemContext>,
    current_focus: Option<usize>,
) -> Option<MenuItemContext> {
    let active_menu_items = filter_active_menu_items(items);

    let Some(current_focus) = current_focus else {
        if let Some(first) = active_menu_items.get(0) {
            return Some(first.clone());
        }
        return None;
    };

    active_menu_items
        .iter()
        .position(|item| item.index == current_focus)
        .map(|i| {
            if i < active_menu_items.len() - 1 {
                active_menu_items[i + 1].clone()
            } else {
                active_menu_items[0].clone()
            }
        })
}

fn prev_active_menu_item(
    items: HashMap<usize, MenuItemContext>,
    current_focus: Option<usize>,
) -> Option<MenuItemContext> {
    let active_menu_items = filter_active_menu_items(items);

    let Some(current_focus) = current_focus else {
        if let Some(last) = active_menu_items.last() {
            return Some(last.clone());
        }
        return None;
    };

    active_menu_items
        .iter()
        .position(|item| item.index == current_focus)
        .map(|i| {
            if i > 0 {
                active_menu_items[i - 1].clone()
            } else {
                active_menu_items[active_menu_items.len() - 1].clone()
            }
        })
}

#[component]
pub fn RootEvents(children: Children) -> impl IntoView {
    let ctx = expect_context::<MenuContext>();

    let _ = use_event_listener(ctx.trigger_ref, click, move |_| {
        ctx.open.set(!ctx.open.get());
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

    let focus_menu_item = move |i: usize| {
        if let Some(item) = ctx.items.get().get(&i) {
            if let Some(trigger_ref) = item.trigger_ref.get() {
                let _ = trigger_ref.focus();
                return true;
            }
        }
        false
    };

    let focus_next = move || {
        if let Some(item) = next_active_menu_item(ctx.items.get(), ctx.current_focus.get()) {
            if focus_menu_item(item.index) {
                ctx.current_focus.set(Some(item.index));
            }
        }
    };

    let focus_previous = move || {
        if let Some(item) = prev_active_menu_item(ctx.items.get(), ctx.current_focus.get()) {
            if focus_menu_item(item.index) {
                ctx.current_focus.set(Some(item.index));
            }
        }
    };

    let _ = use_event_listener(ctx.menu_ref, keydown, move |evt| {
        if evt.key() == "Escape" {
            close();
        } else if evt.key() == "ArrowDown" {
            focus_next();
        } else if evt.key() == "ArrowUp" {
            focus_previous();
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
