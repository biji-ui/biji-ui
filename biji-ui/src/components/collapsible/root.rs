use std::time::Duration;

use leptos::{context::Provider, ev::click, prelude::*};
use leptos_use::use_event_listener;

use crate::{cn, custom_animated_show::CustomAnimatedShow};

use super::context::CollapsibleContext;

#[component]
pub fn Root(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(default = false)] open: bool,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let ctx = CollapsibleContext {
        open: RwSignal::new(open),
        disabled,
        trigger_ref: NodeRef::new(),
    };

    view! {
        <Provider value={ctx}>
            <div
                class={class}
                data-state={move || ctx.data_state()}
                data-disabled={ctx.disabled}
            >
                {children()}
            </div>
        </Provider>
    }
}

#[component]
pub fn Trigger(
    children: Children,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let ctx = expect_context::<CollapsibleContext>();

    let _ = use_event_listener(ctx.trigger_ref, click, move |_| {
        ctx.toggle();
    });

    view! {
        <button
            node_ref={ctx.trigger_ref}
            class={class}
            aria-expanded={move || if ctx.open.get() { "true" } else { "false" }}
            aria-disabled={if ctx.disabled { Some("true") } else { None }}
            data-state={move || ctx.data_state()}
            data-disabled={ctx.disabled}
        >
            {children()}
        </button>
    }
}

#[component]
pub fn Content(
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] show_class: String,
    #[prop(into, optional)] hide_class: String,
    #[prop(default = Duration::from_millis(200))] hide_delay: Duration,
) -> impl IntoView {
    let ctx = expect_context::<CollapsibleContext>();

    view! {
        <CustomAnimatedShow
            when={ctx.open}
            show_class={cn!(class, show_class)}
            hide_class={cn!(class, hide_class)}
            hide_delay={hide_delay}
        >
            {children()}
        </CustomAnimatedShow>
    }
}
