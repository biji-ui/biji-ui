use std::time::Duration;

use leptos::{ev::click, *};
use leptos_use::use_event_listener;

use crate::{
    cn, components::dialog::context::DialogContext, custom_animated_show::CustomAnimatedShow,
};

use super::context::RootContext;

#[component]
pub fn Trigger(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let dialog_ctx = expect_context::<DialogContext>();

    let _trigger_ref = dialog_ctx.trigger_ref;

    view! {
        <TriggerEvents>
            <button _ref={_trigger_ref} class={class}>
                {children()}
            </button>
        </TriggerEvents>
    }
}

#[component]
pub fn TriggerEvents(children: Children) -> impl IntoView {
    let dialog_ctx = expect_context::<DialogContext>();

    let _ = use_event_listener(dialog_ctx.trigger_ref, click, move |_| {
        dialog_ctx.toggle();
        if let Some(trigger_ref) = dialog_ctx.trigger_ref.get() {
            let _ = trigger_ref.blur();
        }
    });

    children()
}

#[component]
pub fn Content(
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
    let dialog_ctx = expect_context::<DialogContext>();

    let children = store_value(children);

    view! {
        <CustomAnimatedShow
            when={dialog_ctx.open}
            show_class={cn!(class, show_class)}
            hide_class={cn!(class, hide_class)}
            hide_delay={hide_delay}
        >
            {children()}
        </CustomAnimatedShow>
    }
}

#[component]
pub fn Overlay(
    #[prop(into, optional)] class: String,
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
    let dialog_ctx = expect_context::<DialogContext>();
    let root_ctx = expect_context::<RootContext>();

    let overlay_ref = root_ctx.overlay_ref;

    view! {
        <CustomAnimatedShow
            when={dialog_ctx.open}
            show_class={cn!(class, show_class)}
            hide_class={cn!(class, hide_class)}
            hide_delay={hide_delay}
        >
            <OverlayEvents>
                <div _ref={overlay_ref} style="inset: 0; width: 100%; height: 100%"></div>
            </OverlayEvents>
        </CustomAnimatedShow>
    }
}

#[component]
pub fn OverlayEvents(children: Children) -> impl IntoView {
    let dialog_ctx = expect_context::<DialogContext>();
    let root_ctx = expect_context::<RootContext>();

    let _ = use_event_listener(root_ctx.overlay_ref, click, move |_| {
        dialog_ctx.open.set(false);
    });

    children()
}

#[component]
pub fn Close(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let root_ctx = expect_context::<RootContext>();

    let close_ref = root_ctx.close_ref;

    view! {
        <CloseEvents>
            <button _ref={close_ref} class={class}>
                {children()}
            </button>
        </CloseEvents>
    }
}

#[component]
pub fn CloseEvents(children: Children) -> impl IntoView {
    let dialog_ctx = expect_context::<DialogContext>();
    let root_ctx = expect_context::<RootContext>();

    let _ = use_event_listener(root_ctx.close_ref, click, move |_| {
        dialog_ctx.open.set(false);
    });

    children()
}
