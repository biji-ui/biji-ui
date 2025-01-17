use std::time::Duration;

use leptos::{
    context::Provider,
    ev::{click, focus, keydown},
    prelude::*,
};
use leptos_use::{on_click_outside, use_event_listener};

use crate::{
    cn,
    custom_animated_show::CustomAnimatedShow,
    items::{Focus, ManageFocus, NavigateItems, Toggle},
};

use super::context::{MenuContext, RootContext};

#[component]
pub fn Menu(
    #[prop(default = false)] disabled: bool,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<RootContext>();

    let index = ctx.next_index();

    let menu_ctx = MenuContext {
        index,
        disabled,
        allow_loop: ctx.allow_item_loop,
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

    let _ = on_click_outside(menu_ctx.menu_ref, move |_| {
        if menu_ctx.open.get() {
            menu_ctx.close();
        }
    });

    on_cleanup(move || {
        drop(eff);
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
    /// The timeout after which the component will be unmounted if `when == false`
    #[prop(default = Duration::from_millis(200))]
    hide_delay: Duration,
) -> impl IntoView {
    let menu_ctx = expect_context::<MenuContext>();

    view! {
        <CustomAnimatedShow
            when={menu_ctx.open}
            show_class={cn!(class, show_class)}
            hide_class={cn!(class, hide_class)}
            hide_delay={hide_delay}
        >
            {children()}
        </CustomAnimatedShow>
    }
}
