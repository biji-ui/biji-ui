use std::time::Duration;

use leptos::ev::{blur, focus, mousemove, pointerenter, pointerleave};
use leptos::{context::Provider, prelude::*};
use leptos_use::{
    UseElementBoundingReturn, use_document, use_element_bounding, use_event_listener,
};

use crate::{
    cn,
    components::tooltip::context::TooltipContext,
    custom_animated_show::CustomAnimatedShow,
    utils::{
        polygon::{get_points_from_el, make_hull, point_in_polygon},
        positioning::Positioning,
    },
};

static TOOLTIP_ID_COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

fn next_tooltip_id() -> String {
    let id = TOOLTIP_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    format!("biji-tooltip-{}", id)
}

#[component]
pub fn Trigger(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let tooltip_ctx = expect_context::<TooltipContext>();

    let trigger_ref = tooltip_ctx.trigger_ref;

    view! {
        <TriggerEvents>
            <button
                node_ref={trigger_ref}
                class={class}
                aria-describedby={move || {
                    if tooltip_ctx.open.get() {
                        Some(tooltip_ctx.tooltip_id.get_value())
                    } else {
                        None
                    }
                }}
            >
                {children()}
            </button>
        </TriggerEvents>
    }
}

#[component]
pub fn TriggerEvents(children: Children) -> impl IntoView {
    let tooltip_ctx = expect_context::<TooltipContext>();

    let UseElementBoundingReturn {
        top: trigger_top,
        left: trigger_left,
        bottom: trigger_bottom,
        right: trigger_right,
        width: trigger_width,
        height: trigger_height,
        ..
    } = use_element_bounding(tooltip_ctx.trigger_ref);

    let UseElementBoundingReturn {
        width: content_width,
        height: content_height,
        ..
    } = use_element_bounding(tooltip_ctx.content_ref);

    let polygon_elements = move || {
        let mut trigger_points =
            get_points_from_el(&(trigger_top, trigger_right, trigger_bottom, trigger_left));

        let content_pos = tooltip_ctx.positioning.calculate_position(
            trigger_top.get(),
            trigger_left.get(),
            trigger_width.get(),
            trigger_height.get(),
            content_height.get(),
            content_width.get(),
            tooltip_ctx.arrow_size as f64,
        );

        let mut content_points = vec![
            (content_pos.1, content_pos.0),
            (content_pos.1 + content_width.get(), content_pos.0),
            (
                content_pos.1 + content_width.get(),
                content_pos.0 + content_height.get(),
            ),
            (content_pos.1, content_pos.0 + content_height.get()),
        ];

        trigger_points.append(&mut content_points);

        trigger_points
    };

    let polygon = move || make_hull(&polygon_elements());

    let _ = use_event_listener(tooltip_ctx.trigger_ref, pointerenter, move |_| {
        tooltip_ctx.pointer_inside_trigger.set(true);
        tooltip_ctx.open();
    });

    let _ = use_event_listener(tooltip_ctx.trigger_ref, pointerleave, move |_| {
        tooltip_ctx.pointer_inside_trigger.set(false);
    });

    let _ = use_event_listener(tooltip_ctx.content_ref, pointerenter, move |_| {
        tooltip_ctx.pointer_inside_content.set(true);
    });

    let _ = use_event_listener(tooltip_ctx.content_ref, pointerleave, move |_| {
        tooltip_ctx.pointer_inside_content.set(false);
    });

    let _ = use_event_listener(use_document(), mousemove, move |e| {
        if tooltip_ctx.pointer_inside_content.get()
            || tooltip_ctx.pointer_inside_trigger.get()
            || point_in_polygon((e.x() as f64, e.y() as f64), &polygon())
        {
            return;
        }
        if tooltip_ctx.open.get() {
            tooltip_ctx.close();
        }
    });

    let _ = use_event_listener(tooltip_ctx.trigger_ref, focus, move |_| {
        tooltip_ctx.open();
    });

    let _ = use_event_listener(tooltip_ctx.trigger_ref, blur, move |_| {
        if tooltip_ctx.open.get() {
            tooltip_ctx.close();
        }
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
    #[prop(default = Positioning::default())] positioning: Positioning,
) -> impl IntoView {
    let ctx = TooltipContext {
        hide_delay,
        positioning,
        tooltip_id: StoredValue::new(next_tooltip_id()),
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

    let content_ref = tooltip_ctx.content_ref;

    let UseElementBoundingReturn {
        width: content_width,
        height: content_height,
        ..
    } = use_element_bounding(tooltip_ctx.content_ref);

    let UseElementBoundingReturn {
        top,
        left,
        width,
        height,
        ..
    } = use_element_bounding(tooltip_ctx.trigger_ref);

    let style_signal = Signal::derive(move || {
        tooltip_ctx.positioning.calculate_position_style(
            *top.read(),
            *left.read(),
            *width.read(),
            *height.read(),
            *content_height.read(),
            *content_width.read(),
            tooltip_ctx.arrow_size as f64,
            tooltip_ctx.arrow_size as f64,
        )
    });

    view! {
        <CustomAnimatedShow
            when={tooltip_ctx.open}
            show_class={cn!(class, show_class)}
            hide_class={cn!(class, hide_class)}
            hide_delay={tooltip_ctx.hide_delay}
            style_signal={style_signal}
            node_ref={content_ref}
            attr:id={tooltip_ctx.tooltip_id.get_value()}
            attr:role="tooltip"
        >
            {children()}
        </CustomAnimatedShow>
    }
}

#[component]
pub fn Arrow(#[prop(into, optional)] class: String) -> impl IntoView {
    let tooltip_ctx = expect_context::<TooltipContext>();
    view! {
        <div
            class={class}
            style={move || {
                format!(
                    "position: fixed; top: var(--biji-tooltip-arrow-top); left: var(--biji-tooltip-arrow-left); height: {}px; width: {}px; background-color: inherit; transform: rotate(var(--biji-tooltip-arrow-rotation));",
                    tooltip_ctx.arrow_size,
                    tooltip_ctx.arrow_size,
                )
            }}
        ></div>
    }
}
