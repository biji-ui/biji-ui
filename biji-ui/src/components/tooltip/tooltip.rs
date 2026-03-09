use std::time::Duration;

use leptos::ev::{blur, focus, mousemove, pointerenter, pointerleave};
use leptos::{context::Provider, prelude::*};
use leptos_use::{
    UseElementBoundingReturn, use_document, use_element_bounding, use_event_listener,
};
use wasm_bindgen::JsCast;

use crate::{
    cn,
    components::tooltip::{context::TooltipContext, singleton},
    custom_animated_show::CustomAnimatedShow,
    utils::{
        polygon::{get_points_from_el, make_hull, point_in_polygon},
        positioning::{AvoidCollisions, Positioning},
    },
};

static TOOLTIP_ID_COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

fn next_tooltip_id() -> (usize, String) {
    let id = TOOLTIP_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    (id, format!("biji-tooltip-{}", id))
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

        // Use use_element_bounding signals only as reactive dependencies so this
        // closure re-runs when the content mounts/unmounts.
        let _ = content_width.get();
        let _ = content_height.get();

        // Read actual dimensions via offsetWidth/offsetHeight — identical to the
        // approach in Content — so that CSS scale transforms (e.g. scale-95 during
        // the open animation) do not produce shrunken values.
        let (cw, ch) = tooltip_ctx
            .content_ref
            .get_untracked()
            .and_then(|el| {
                let node: &web_sys::Node = el.as_ref();
                node.dyn_ref::<web_sys::HtmlElement>()
                    .map(|h| (h.offset_width() as f64, h.offset_height() as f64))
            })
            .unwrap_or((0.0, 0.0));

        let vp_w = web_sys::window()
            .and_then(|w| w.inner_width().ok())
            .and_then(|v| v.as_f64())
            .unwrap_or(1920.0);
        let vp_h = web_sys::window()
            .and_then(|w| w.inner_height().ok())
            .and_then(|v| v.as_f64())
            .unwrap_or(1080.0);
        let eff = tooltip_ctx.positioning.effective_positioning(
            cw,
            ch,
            trigger_top.get(),
            trigger_left.get(),
            trigger_width.get(),
            trigger_height.get(),
            tooltip_ctx.arrow_size as f64,
            vp_w,
            vp_h,
            tooltip_ctx.avoid_collisions,
        );
        let content_pos = eff.calculate_position(
            trigger_top.get(),
            trigger_left.get(),
            trigger_width.get(),
            trigger_height.get(),
            ch,
            cw,
            tooltip_ctx.arrow_size as f64,
        );

        let mut content_points = vec![
            (content_pos.1, content_pos.0),
            (content_pos.1 + cw, content_pos.0),
            (content_pos.1 + cw, content_pos.0 + ch),
            (content_pos.1, content_pos.0 + ch),
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
    #[prop(default = AvoidCollisions::Flip)] avoid_collisions: AvoidCollisions,
) -> impl IntoView {
    let (numeric_id, string_id) = next_tooltip_id();
    let open_signal = RwSignal::new(false);

    let ctx = TooltipContext {
        hide_delay,
        positioning,
        avoid_collisions,
        numeric_id,
        tooltip_id: StoredValue::new(string_id),
        open: open_signal,
        ..TooltipContext::default()
    };

    singleton::register(numeric_id, move || open_signal.set(false));
    on_cleanup(move || singleton::unregister(numeric_id));

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
        let raw_cw = *content_width.read();
        let raw_ch = *content_height.read();
        let _ = tooltip_ctx.open.get();
        let _ = top.read();
        let _ = left.read();
        let _ = width.read();
        let _ = height.read();
        let hidden = || format!(
            "position: fixed; top: 0; left: 0; visibility: hidden; --biji-transform-origin: {};",
            tooltip_ctx.positioning.transform_origin()
        );
        if raw_cw == 0.0 && raw_ch == 0.0 {
            return hidden();
        }
        // Use offsetWidth/offsetHeight to avoid measuring scaled-down dimensions
        // when the hide_class includes a CSS scale transform.
        let Some(content_div) = tooltip_ctx.content_ref.get_untracked() else {
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
        let Some(trigger) = tooltip_ctx.trigger_ref.get_untracked() else {
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
        let eff = tooltip_ctx.positioning.effective_positioning(
            cw,
            ch,
            t_top,
            t_left,
            t_width,
            t_height,
            tooltip_ctx.arrow_size as f64,
            vp_w,
            vp_h,
            tooltip_ctx.avoid_collisions,
        );
        eff.calculate_position_style(
            t_top,
            t_left,
            t_width,
            t_height,
            ch,
            cw,
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
