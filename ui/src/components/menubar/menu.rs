use std::time::Duration;

use leptos::{
    context::Provider,
    ev::{click, focus, keydown},
    prelude::*,
};
use leptos_use::{
    on_click_outside, use_element_bounding, use_event_listener, UseElementBoundingReturn,
};
use wasm_bindgen::JsCast;

use crate::{
    custom_animated_show::CustomAnimatedShow,
    items::{Focus, ManageFocus, NavigateItems, Toggle},
    utils::{positioning::Positioning, prevent_scroll::use_prevent_scroll},
};

use super::context::{MenuContext, RootContext};

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
        if menu_ctx.open.get() == false {
            menu_ctx.set_focus(None);
        }
    });

    let _ = use_event_listener(menu_ctx.trigger_ref, click, move |_| {
        menu_ctx.toggle();
    });

    let _ = use_event_listener(menu_ctx.trigger_ref, keydown, move |evt| {
        let key = evt.key();

        if key == "ArrowRight" {
            if let Some(item) = root_ctx.navigate_next_item() {
                if menu_ctx.open.get() {
                    item.open();
                }
                item.focus();
                menu_ctx.close();
            }
        } else if key == "ArrowLeft" {
            if let Some(item) = root_ctx.navigate_previous_item() {
                if menu_ctx.open.get() {
                    item.open();
                }
                item.focus();
                menu_ctx.close();
            }
        } else if key == "ArrowDown" || key == "Enter" {
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
        if menu_ctx.open.get() {
            // Recursive function to check if click is within any submenu or nested submenu
            fn is_click_in_submenu_tree(
                menu_context: &super::context::MenuContext,
                target: &web_sys::Element,
            ) -> bool {
                menu_context.items.with(|items| {
                    items.values().any(|item| {
                        if let super::context::ItemData::SubMenuItem { child_context, .. } = item {
                            // Check if click is on the submenu trigger
                            if let Some(trigger_el) = child_context.trigger_ref.get() {
                                if trigger_el.contains(Some(target)) {
                                    return true;
                                }
                            }
                            // Check if click is within the submenu content
                            if let Some(menu_el) = child_context.menu_ref.get() {
                                if menu_el.contains(Some(target)) {
                                    return true;
                                }
                            }
                            // Recursively check nested submenus
                            if is_click_in_submenu_tree(child_context, target) {
                                return true;
                            }
                        }
                        false
                    })
                })
            }

            // Check if the click target is within any submenu (recursively)
            let is_submenu_click = if let Some(target) = evt.target() {
                if let Ok(target_el) = target.dyn_into::<web_sys::Element>() {
                    is_click_in_submenu_tree(&menu_ctx, &target_el)
                } else {
                    false
                }
            } else {
                false
            };

            if !is_submenu_click {
                menu_ctx.close();
            }
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
            <div node_ref={content_ref} class={class.clone()} style={style}>
                {children()}
            </div>
        </CustomAnimatedShow>
    }
}
