use std::time::Duration;

use leptos::{
    ev::{click, mouseout, mouseover},
    *,
};
use leptos_use::use_event_listener;

use crate::{
    cn, components::tooltip::context::TooltipContext, custom_animated_show::CustomAnimatedShow,
};

#[component]
pub fn Trigger(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let tooltip_ctx = expect_context::<TooltipContext>();

    let trigger_ref = tooltip_ctx.trigger_ref;

    view! {
        <TriggerEvents>
            <button node_ref={trigger_ref} class={class}>
                {children()}
            </button>
        </TriggerEvents>
    }
}

#[component]
pub fn TriggerEvents(children: Children) -> impl IntoView {
    let tooltip_ctx = expect_context::<TooltipContext>();

    let _ = use_event_listener(tooltip_ctx.trigger_ref, click, move |_| {
        tooltip_ctx.toggle();
    });

    let _ = use_event_listener(tooltip_ctx.trigger_ref, mouseover, move |_| {
        tooltip_ctx.open();
    });

    let _ = use_event_listener(tooltip_ctx.trigger_ref, mouseout, move |_| {
        tooltip_ctx.close();
    });

    children()
}

#[component]
pub fn Root(
    #[prop(into, optional)] class: String,
    children: Children,
    /// The timeout after which the component will be unmounted if `when == false`
    #[prop(default = Duration::from_millis(200))]
    hide_delay: Duration,
) -> impl IntoView {
    let ctx = TooltipContext {
        hide_delay,
        ..TooltipContext::default()
    };

    view! {
        <Provider value={ctx}>
            <div class={class}>{children()}</div>
        </Provider>
    }
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
) -> impl IntoView {
    let tooltip_ctx = expect_context::<TooltipContext>();

    let children = store_value(children);

    view! {
        <CustomAnimatedShow
            when={tooltip_ctx.open}
            show_class={cn!(class, show_class)}
            hide_class={cn!(class, hide_class)}
            hide_delay={tooltip_ctx.hide_delay}
        >
            {children()}
        </CustomAnimatedShow>
    }
}
