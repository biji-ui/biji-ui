use std::time::Duration;

use leptos::{
    context::Provider,
    ev::{focus, keydown, mouseover},
    prelude::*,
};
use leptos_use::{UseElementBoundingReturn, use_element_bounding, use_event_listener};
use wasm_bindgen::JsCast;
use web_sys::{HtmlAnchorElement, HtmlButtonElement};

use crate::{
    cn,
    components::menubar::context::ItemData,
    custom_animated_show::CustomAnimatedShow,
    items::{Focus, GetIndex, ManageFocus, NavigateItems, Toggle},
    utils::positioning::Positioning,
};

use super::context::{MenuContext, MenubarContext};

#[component]
pub fn Item(
    #[prop(default = false)] disabled: bool,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let menu_ctx = expect_context::<MenuContext>();

    let item_ctx = use_context::<ItemData>();

    let trigger_ref = NodeRef::new();

    let index = menu_ctx.next_index();

    let item_ctx = ItemData::Item {
        index,
        disabled,
        trigger_ref,
        is_submenu: item_ctx.is_some(),
    };

    menu_ctx.upsert_item(index, item_ctx);

    on_cleanup(move || {
        menu_ctx.remove_item(index);
    });

    view! {
        <Provider value={item_ctx}>
            <ItemTriggerEvents>
                <div
                    node_ref={trigger_ref}
                    class={class}
                    tabindex=0
                    role="menuitem"
                    aria-disabled={if item_ctx.get_disabled() { Some("true") } else { None }}
                    data-state={item_ctx.get_index()}
                    data-disabled={item_ctx.get_disabled()}
                    data-highlighted={move || menu_ctx.item_in_focus(item_ctx.get_index())}
                >
                    {children()}
                </div>
            </ItemTriggerEvents>
        </Provider>
    }
}

#[component]
pub fn ItemTriggerEvents(children: Children) -> impl IntoView {
    let root_ctx = expect_context::<MenubarContext>().root.get();
    let menu_ctx = expect_context::<MenuContext>();
    let item_ctx = expect_context::<ItemData>();

    let handle_on_click = move || {
        if let Some(trigger_ref) = item_ctx.get_trigger_ref().get() {
            if let Some(child) = trigger_ref.children().get_with_index(0) {
                if let Ok(child) = child.clone().dyn_into::<HtmlButtonElement>() {
                    let _ = child.click();
                } else if let Ok(child) = child.dyn_into::<HtmlAnchorElement>() {
                    let _ = child.click();
                }
            }
        }
    };

    let _ = use_event_listener(item_ctx.get_trigger_ref(), keydown, move |evt| {
        let key = evt.key();

        match key.as_str() {
            "ArrowDown" => {
                evt.prevent_default();
                if let Some(item) = menu_ctx.navigate_next_item() {
                    item.focus();
                }
            }
            "ArrowUp" => {
                evt.prevent_default();
                if let Some(item) = menu_ctx.navigate_previous_item() {
                    item.focus();
                }
            }
            "ArrowRight" => {
                evt.prevent_default();
                if let ItemData::SubMenuItem { child_context, .. } = item_ctx {
                    // Keyboard-focused item is a SubMenuItem → enter it.
                    if !child_context.open.get_untracked() {
                        child_context.open();
                    }
                    if let Some(item) = child_context.navigate_first_item() {
                        item.focus();
                    }
                } else {
                    // Keyboard-focused item is a regular Item.
                    // If hover has highlighted a SubMenuItem, enter that instead.
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
                        root_ctx.close_all();
                        item.focus();
                        item.open();
                    }
                }
            }
            "ArrowLeft" => {
                evt.prevent_default();
                if item_ctx.is_submenu() {
                    menu_ctx.skip_open_on_focus.set(true);
                    menu_ctx.close_with_submenus();
                    menu_ctx.focus();
                    menu_ctx.item_focus.set(None);
                } else if let Some(item) = root_ctx.navigate_previous_item() {
                    item.focus();
                    item.open();
                    menu_ctx.close_with_submenus();
                }
            }
            "Enter" => {
                if let ItemData::Item { .. } = item_ctx {
                    handle_on_click();
                    root_ctx.close_all();
                    root_ctx.focus_active_item();
                }
                // SubMenuItem Enter is handled by SubMenuItemTriggerEvents
            }
            "Escape" => {
                menu_ctx.close_with_submenus();
                menu_ctx.focus();
            }
            _ => {}
        };
    });

    let _ = use_event_listener(item_ctx.get_trigger_ref(), focus, move |_| {
        menu_ctx.set_focus(Some(item_ctx.get_index()));
        if let ItemData::SubMenuItem { child_context, .. } = item_ctx {
            menu_ctx.close_all_except(child_context.index);
            if child_context.skip_open_on_focus.get_untracked() {
                child_context.skip_open_on_focus.set(false);
            } else if !child_context.open.get_untracked() {
                child_context.open();
            }
        } else {
            menu_ctx.close_all();
        }
    });

    let _ = use_event_listener(item_ctx.get_trigger_ref(), mouseover, move |evt| {
        evt.stop_propagation();
        menu_ctx.set_focus(Some(item_ctx.get_index()));
        if let ItemData::SubMenuItem { child_context, .. } = item_ctx {
            // close_all_except avoids a flash: mouseover re-fires each time the
            // mouse crosses a child element (text → icon, etc.), so we must not
            // close our own submenu only to immediately reopen it.
            menu_ctx.close_all_except(child_context.index);
            if !child_context.open.get_untracked() {
                child_context.open();
            }
        } else {
            menu_ctx.close_all();
        }
        // Move DOM focus to this item on hover so that if the user switches to
        // keyboard navigation, focus is not stranded inside a just-closed submenu.
        // If the element already has focus the browser does not fire a second
        // `focus` event, so the focus handler is only re-invoked on an actual change.
        item_ctx.focus();
    });

    children()
}

#[component]
pub fn SubMenuItem(
    #[prop(default = false)] disabled: bool,
    #[prop(into, optional)] class: String,
    #[prop(default = Positioning::RightStart)] positioning: Positioning,
    /// The timeout after which the component will be unmounted if `when == false`
    #[prop(default = Duration::from_millis(200))]
    hide_delay: Duration,
    children: Children,
) -> impl IntoView {
    let menu_ctx = expect_context::<MenuContext>();

    let item_ctx = use_context::<ItemData>();

    let index = menu_ctx.next_index();

    let sub_menu_ctx = MenuContext {
        index,
        disabled,
        allow_loop: menu_ctx.allow_loop,
        positioning,
        hide_delay,
        ..Default::default()
    };

    let item_ctx = ItemData::SubMenuItem {
        index,
        disabled,
        is_submenu: item_ctx.is_some(),
        parent_context: menu_ctx,
        child_context: sub_menu_ctx,
    };

    menu_ctx.upsert_item(index, item_ctx);

    on_cleanup(move || {
        menu_ctx.remove_item(index);
    });

    view! {
        <Provider value={item_ctx}>
            <div class={class}>
                <ItemTriggerEvents>
                    <Provider value={sub_menu_ctx}>{children()}</Provider>
                </ItemTriggerEvents>
            </div>
        </Provider>
    }
}

#[component]
pub fn SubMenuItemTrigger(
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    // `child_menu_ctx` is the MenuContext for this submenu (its open state, items, etc.).
    // `item_data`      is the ItemData::SubMenuItem registered in the parent menu.
    let child_menu_ctx = expect_context::<MenuContext>();
    let item_data = expect_context::<ItemData>();

    let trigger_ref = child_menu_ctx.trigger_ref;

    view! {
        <SubMenuItemTriggerEvents>
            <div
                node_ref={trigger_ref}
                class={class}
                tabindex=0
                role="menuitem"
                aria-haspopup="menu"
                aria-expanded={move || if child_menu_ctx.open.get() { "true" } else { "false" }}
                aria-disabled={if child_menu_ctx.disabled { Some("true") } else { None }}
                data-state={child_menu_ctx.index}
                data-disabled={child_menu_ctx.disabled}
                data-highlighted={move || {
                    if let ItemData::SubMenuItem { parent_context, .. } = item_data {
                        parent_context.item_in_focus(child_menu_ctx.index)
                    } else {
                        false
                    }
                }}
            >
                {children()}
            </div>
        </SubMenuItemTriggerEvents>
    }
}

#[component]
pub fn SubMenuItemTriggerEvents(children: Children) -> impl IntoView {
    // `menu_ctx` is the child MenuContext for this submenu — the same context
    // that `SubMenuItemTrigger` exposes as `child_menu_ctx`.
    let menu_ctx = expect_context::<MenuContext>();

    let _ = use_event_listener(menu_ctx.trigger_ref, keydown, move |evt| {
        if evt.key() == "Enter" {
            evt.prevent_default();
            // stop_propagation prevents this from also bubbling to the
            // ItemTriggerEvents keydown handler on the same element.
            evt.stop_propagation();
            menu_ctx.toggle();
            // If we just opened it, move focus to the first item.
            if menu_ctx.open.get_untracked() {
                if let Some(item) = menu_ctx.navigate_first_item() {
                    item.focus();
                }
            }
        }
    });

    children()
}

#[component]
pub fn SubMenuItemContent(
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

    let style_signal = Signal::derive(move || {
        let raw_cw = *content_width.read();
        let raw_ch = *content_height.read();
        let _ = menu_ctx.open.get();
        let _ = trigger_top.read();
        let _ = trigger_left.read();
        let _ = trigger_width.read();
        let _ = trigger_height.read();
        let hidden = || format!(
            "position: fixed; top: 0; left: 0; visibility: hidden; --biji-transform-origin: {};",
            menu_ctx.positioning.transform_origin()
        );
        if raw_cw == 0.0 && raw_ch == 0.0 {
            return hidden();
        }
        let Some(content_div) = content_ref.get_untracked() else {
            return hidden();
        };
        let content_node: &web_sys::Node = content_div.as_ref();
        let Some(content_html) = content_node.dyn_ref::<web_sys::HtmlElement>() else {
            return hidden();
        };
        let cw = content_html.offset_width() as f64;
        let ch = content_html.offset_height() as f64;
        if cw == 0.0 && ch == 0.0 {
            return hidden();
        }
        let Some(trigger) = menu_ctx.trigger_ref.get_untracked() else {
            return hidden();
        };
        let trigger_node: &web_sys::Node = trigger.as_ref();
        let Some(trigger_el) = trigger_node.dyn_ref::<web_sys::Element>() else {
            return hidden();
        };
        let rect = trigger_el.get_bounding_client_rect();
        let (t_top, t_left, t_width, t_height) =
            (rect.top(), rect.left(), rect.width(), rect.height());
        let vp_w = web_sys::window()
            .and_then(|w| w.inner_width().ok())
            .and_then(|v| v.as_f64())
            .unwrap_or(1920.0);
        let vp_h = web_sys::window()
            .and_then(|w| w.inner_height().ok())
            .and_then(|v| v.as_f64())
            .unwrap_or(1080.0);
        let eff = menu_ctx.positioning.effective_positioning(
            cw,
            ch,
            t_top,
            t_left,
            t_width,
            t_height,
            0.0,
            vp_w,
            vp_h,
            menu_ctx.avoid_collisions,
        );
        eff.calculate_position_style_simple(
            t_top,
            t_left,
            t_width,
            t_height,
            ch,
            cw,
            0.0,
        )
    });

    view! {
        <CustomAnimatedShow
            when={menu_ctx.open}
            show_class={cn!(class, show_class)}
            hide_class={cn!(class, hide_class)}
            hide_delay={menu_ctx.hide_delay}
            style_signal={style_signal}
            node_ref={content_ref}
            attr:role="menu"
        >
            {children()}
        </CustomAnimatedShow>
    }
}
