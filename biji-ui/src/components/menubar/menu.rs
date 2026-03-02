use std::time::Duration;

use leptos::{
    context::Provider,
    ev::{click, focus, keydown, mouseover},
    prelude::*,
};
use leptos_use::{
    UseElementBoundingReturn, on_click_outside, use_element_bounding, use_event_listener,
};
use wasm_bindgen::JsCast;

use crate::{
    custom_animated_show::CustomAnimatedShow,
    items::{Focus, ManageFocus, NavigateItems, Toggle},
    utils::{positioning::Positioning, prevent_scroll::use_prevent_scroll},
};

use super::context::{ItemData, MenuContext, RootContext};

/// Walks the submenu tree rooted at `menu_context` and returns `true` if
/// `target` is contained within any submenu trigger or content element.
/// Used by the click-outside handler to avoid closing a menu when the user
/// clicks inside one of its (possibly deeply-nested) submenus.
fn is_click_in_submenu_tree(menu_context: &MenuContext, target: &web_sys::Element) -> bool {
    menu_context.items.with(|items| {
        items.values().any(|item| {
            let ItemData::SubMenuItem { child_context, .. } = item else {
                return false;
            };
            if let Some(el) = child_context.trigger_ref.get() {
                if el.contains(Some(target)) {
                    return true;
                }
            }
            if let Some(el) = child_context.menu_ref.get() {
                if el.contains(Some(target)) {
                    return true;
                }
            }
            is_click_in_submenu_tree(child_context, target)
        })
    })
}

#[component]
pub fn Menu(
    #[prop(default = false)] disabled: bool,
    #[prop(into, optional)] class: String,
    #[prop(default = Positioning::BottomStart)] positioning: Positioning,
    /// The timeout after which the component will be unmounted if `when == false`
    #[prop(default = Duration::from_millis(200))]
    hide_delay: Duration,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<RootContext>();

    let index = ctx.next_index();

    let menu_ctx = MenuContext {
        index,
        disabled,
        allow_loop: ctx.allow_item_loop,
        positioning,
        hide_delay,
        ..Default::default()
    };

    ctx.upsert_item(index, menu_ctx);

    on_cleanup(move || {
        ctx.remove_item(index);
    });

    let menu_ref = menu_ctx.menu_ref;

    view! {
        <Provider value={menu_ctx}>
            <div node_ref={menu_ref} class={class} data-index={index}>
                {children()}
            </div>
        </Provider>
    }
}

#[component]
pub fn MenuTrigger(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let root_ctx = expect_context::<RootContext>();
    let menu_ctx = expect_context::<MenuContext>();

    let trigger_ref = menu_ctx.trigger_ref;

    view! {
        <MenuTriggerEvents>
            <div
                node_ref={trigger_ref}
                class={class}
                role="menuitem"
                aria-haspopup="menu"
                aria-expanded={move || if menu_ctx.open.get() { "true" } else { "false" }}
                aria-disabled={if menu_ctx.disabled { Some("true") } else { None }}
                data-state={menu_ctx.index}
                data-disabled={menu_ctx.disabled}
                data-highlighted={move || root_ctx.item_in_focus(menu_ctx.index)}
                data-open={move || menu_ctx.open.get()}
                tabindex=0
            >
                {children()}
            </div>
        </MenuTriggerEvents>
    }
}

#[component]
pub fn MenuTriggerEvents(children: Children) -> impl IntoView {
    let root_ctx = expect_context::<RootContext>();
    let menu_ctx = expect_context::<MenuContext>();

    let eff = RenderEffect::new(move |_| {
        if !menu_ctx.open.get() {
            menu_ctx.set_focus(None);
        }
    });

    let _ = use_event_listener(menu_ctx.trigger_ref, click, move |_| {
        menu_ctx.toggle();
    });

    let _ = use_event_listener(menu_ctx.trigger_ref, mouseover, move |_| {
        if menu_ctx.disabled {
            return;
        }
        root_ctx.set_focus(Some(menu_ctx.index));
        if root_ctx.any_open() && !menu_ctx.open.get_untracked() {
            root_ctx.close_all();
            menu_ctx.open();
        }
        // Keep DOM focus on the trigger so keyboard events land here, not on a
        // stale element inside a submenu that is animating closed.
        menu_ctx.focus();
    });

    let _ = use_event_listener(menu_ctx.trigger_ref, keydown, move |evt| {
        let key = evt.key();

        if key == "ArrowRight" {
            // If a SubMenuItem is highlighted via hover, enter it (macOS behavior).
            // Otherwise navigate to the next top-level menu.
            let hovered_sub = menu_ctx
                .item_focus
                .get_untracked()
                .and_then(|idx| {
                    menu_ctx
                        .items
                        .with_untracked(|items| items.get(&idx).copied())
                })
                .and_then(|item| {
                    if let ItemData::SubMenuItem { child_context, .. } = item {
                        Some(child_context)
                    } else {
                        None
                    }
                });
            if let Some(child_ctx) = hovered_sub {
                if !child_ctx.open.get_untracked() {
                    child_ctx.open();
                }
                if let Some(first) = child_ctx.navigate_first_item() {
                    first.focus();
                }
            } else if let Some(item) = root_ctx.navigate_next_item() {
                if menu_ctx.open.get() {
                    item.open();
                }
                item.focus();
                menu_ctx.close_with_submenus();
            }
        } else if key == "ArrowLeft" {
            if let Some(item) = root_ctx.navigate_previous_item() {
                if menu_ctx.open.get() {
                    item.open();
                }
                item.focus();
                menu_ctx.close_with_submenus();
            }
        } else if key == "ArrowDown" {
            if !menu_ctx.open.get() {
                menu_ctx.open();
            }
            // If an item is highlighted via hover, continue from that position.
            // Otherwise start at the first item.
            let item = if menu_ctx.item_focus.get_untracked().is_some() {
                menu_ctx.navigate_next_item()
            } else {
                menu_ctx.navigate_first_item()
            };
            if let Some(item) = item {
                item.focus();
            }
        } else if key == "ArrowUp" {
            if menu_ctx.open.get() {
                // If an item is highlighted via hover, continue from that position.
                // Otherwise start at the last item.
                let item = if menu_ctx.item_focus.get_untracked().is_some() {
                    menu_ctx.navigate_previous_item()
                } else {
                    menu_ctx.navigate_last_item()
                };
                if let Some(item) = item {
                    item.focus();
                }
            }
        } else if key == "Enter" {
            if !menu_ctx.open.get() {
                menu_ctx.open();
            }
            if let Some(item) = menu_ctx.navigate_first_item() {
                item.focus();
            }
        } else if key == "Escape" {
            root_ctx.close_all();
        }
    });

    let _ = use_event_listener(menu_ctx.trigger_ref, focus, move |_| {
        root_ctx.set_focus(Some(menu_ctx.index));
    });

    let _ = on_click_outside(menu_ctx.menu_ref, move |evt| {
        if !menu_ctx.open.get() {
            return;
        }
        let in_submenu = evt
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            .is_some_and(|el| is_click_in_submenu_tree(&menu_ctx, &el));
        if !in_submenu {
            menu_ctx.close();
        }
    });

    let ps_eff = use_prevent_scroll(
        move || root_ctx.prevent_scroll && menu_ctx.open.get(),
        menu_ctx.hide_delay,
    );

    on_cleanup(move || {
        drop(eff);
        drop(ps_eff);
    });

    children()
}

#[component]
pub fn MenuContent(
    children: ChildrenFn,
    /// Optional CSS class to apply to both show and hide classes
    #[prop(into, optional)]
    class: String,
    /// Optional CSS class to apply if `when == true`
    #[prop(into, optional)]
    show_class: String,
    /// Optional CSS class to apply if `when == false`
    #[prop(into, optional)]
    hide_class: String,
) -> impl IntoView {
    let menu_ctx = expect_context::<MenuContext>();

    let content_ref = NodeRef::<leptos::html::Div>::new();

    let UseElementBoundingReturn {
        width: content_width,
        height: content_height,
        ..
    } = use_element_bounding(content_ref);

    let UseElementBoundingReturn {
        top: trigger_top,
        left: trigger_left,
        width: trigger_width,
        height: trigger_height,
        ..
    } = use_element_bounding(menu_ctx.trigger_ref);

    let style = move || {
        menu_ctx.positioning.calculate_position_style_simple(
            *trigger_top.read(),
            *trigger_left.read(),
            *trigger_width.read(),
            *trigger_height.read(),
            *content_height.read(),
            *content_width.read(),
            0.0,
        )
    };

    view! {
        <CustomAnimatedShow
            when={menu_ctx.open}
            show_class={show_class.clone()}
            hide_class={hide_class.clone()}
            hide_delay={menu_ctx.hide_delay}
        >
            <div node_ref={content_ref} class={class.clone()} style={style} role="menu">
                {children()}
            </div>
        </CustomAnimatedShow>
    }
}
