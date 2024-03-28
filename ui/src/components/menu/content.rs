use std::time::Duration;

use leptos::{ev::keydown, *};
use leptos_use::{on_click_outside, use_event_listener, use_window};

use crate::custom_animated_show::CustomAnimatedShow;

use super::root::MenuContext;

#[component]
pub fn Content(
    children: ChildrenFn,
    /// Optional CSS class to apply if `when == true`
    #[prop(into, optional)]
    show_class: String,
    /// Optional CSS class to apply if `when == false`
    #[prop(into, optional)]
    hide_class: String,
    /// The timeout after which the component will be unmounted if `when == false`
    hide_delay: Duration,
) -> impl IntoView {
    let ctx = expect_context::<MenuContext>();

    let children = store_value(children);
    view! {
        <CustomAnimatedShow
            when={ctx.open}
            show_class={show_class}
            hide_class={hide_class}
            hide_delay={hide_delay}
        >
            <Events>{children()}</Events>
        </CustomAnimatedShow>
    }
}

#[component]
fn Events(children: ChildrenFn) -> impl IntoView {
    let ctx = expect_context::<MenuContext>();

    let _ = use_event_listener(use_window(), keydown, move |evt| {
        if evt.key() == "Escape" {
            ctx.open.set(false);
        }
    });

    if ctx.close_on_outside_click {
        let _ = on_click_outside(ctx.menu_ref, move |_| {
            ctx.open.set(false);
        });
    }

    children()
}
