use std::time::Duration;

use leptos::ev::{blur, focus, mousemove, pointerenter, pointerleave};
use leptos::{context::Provider, leptos_dom, prelude::*};
use leptos_dom::helpers::TimeoutHandle;
use leptos_use::{
    use_document, use_element_bounding, use_event_listener, UseElementBoundingReturn,
};

use crate::{
    cn,
    components::tooltip::context::TooltipContext,
    utils::polygon::{get_points_from_el, make_hull, point_in_polygon},
};

#[derive(Copy, Clone)]
pub enum Positioning {
    Top,
    TopStart,
    TopEnd,
    Right,
    RightStart,
    RightEnd,
    Bottom,
    BottomStart,
    BottomEnd,
    Left,
    LeftStart,
    LeftEnd,
}

impl Default for Positioning {
    fn default() -> Self {
        Positioning::Top
    }
}

impl Positioning {
    pub fn calculate_position_style(
        self,
        top: f64,
        left: f64,
        width: f64,
        height: f64,
        content_height: f64,
        content_width: f64,
        arrow_size: f64,
    ) -> String {
        let position = self.calculate_position(
            top,
            left,
            width,
            height,
            content_height,
            content_width,
            arrow_size,
        );
        let arrow_position = self.calculate_arrow_position(top, left, width, height, arrow_size);
        format!(
            "position: fixed; top: {}px; left: {}px; --biji-tooltip-arrow-top: {}px; --biji-tooltip-arrow-left: {}px; --biji-tooltip-arrow-rotation: {}deg;",
            position.0, position.1, arrow_position.0, arrow_position.1, arrow_position.2
        )
    }

    pub fn calculate_position(
        self,
        top: f64,
        left: f64,
        width: f64,
        height: f64,
        content_height: f64,
        content_width: f64,
        arrow_size: f64,
    ) -> (f64, f64) {
        match self {
            Positioning::Top => {
                let top = top - content_height - arrow_size;
                let left = left + (width / 2.0) - (content_width / 2.0);
                (top, left)
            }
            Positioning::TopStart => {
                let top = top - content_height - arrow_size;
                (top, left)
            }
            Positioning::TopEnd => {
                let top = top - content_height - arrow_size;
                let left = left + width - content_width;
                (top, left)
            }
            Positioning::Right => {
                let top = top + (height / 2.0) - (content_height / 2.0);
                let left = left + width + arrow_size;
                (top, left)
            }
            Positioning::RightStart => {
                let left = left + width + arrow_size;
                (top, left)
            }
            Positioning::RightEnd => {
                let top = top + height - content_height;
                let left = left + width + arrow_size;
                (top, left)
            }
            Positioning::Bottom => {
                let top = top + height + arrow_size;
                let left = left + (width / 2.0) - (content_width / 2.0);
                (top, left)
            }
            Positioning::BottomStart => {
                let top = top + height + arrow_size;
                (top, left)
            }
            Positioning::BottomEnd => {
                let top = top + height + arrow_size;
                let left = left + width - content_width;
                (top, left)
            }
            Positioning::Left => {
                let top = top + (height / 2.0) - (content_height / 2.0);
                let left = left - content_width - arrow_size;
                (top, left)
            }
            Positioning::LeftStart => {
                let left = left - content_width - arrow_size;
                (top, left)
            }
            Positioning::LeftEnd => {
                let left = left - content_width - arrow_size;
                let top = top + height - content_height;
                (top, left)
            }
        }
    }

    pub fn calculate_arrow_position(
        self,
        top: f64,
        left: f64,
        width: f64,
        height: f64,
        arrow_size: f64,
    ) -> (f64, f64, i32) {
        match self {
            Positioning::Top => {
                let top = top - arrow_size - (arrow_size / 2.0);
                let left = left + (width / 2.0) - (arrow_size / 2.0);
                (top, left, 225)
            }
            Positioning::TopStart => {
                let top = top - arrow_size - (arrow_size / 2.0);
                let left = left + (width / 2.0) - (arrow_size / 2.0);
                (top, left, 225)
            }
            Positioning::TopEnd => {
                let top = top - arrow_size - (arrow_size / 2.0);
                let left = left + (width / 2.0) - (arrow_size / 2.0);
                (top, left, 225)
            }
            Positioning::Right => {
                let top = top + (height / 2.0) - (arrow_size / 2.0);
                let left = left + width + (arrow_size / 2.0);
                (top, left, 315)
            }
            Positioning::RightStart => {
                let top = top + (height / 2.0) - (arrow_size / 2.0);
                let left = left + width + (arrow_size / 2.0);
                (top, left, 315)
            }
            Positioning::RightEnd => {
                let top = top + (height / 2.0) - (arrow_size / 2.0);
                let left = left + width + (arrow_size / 2.0);
                (top, left, 315)
            }
            Positioning::Bottom => {
                let top = top + height + arrow_size - (arrow_size / 2.0);
                let left = left + (width / 2.0) - (arrow_size / 2.0);
                (top, left, 45)
            }
            Positioning::BottomStart => {
                let top = top + height + arrow_size - (arrow_size / 2.0);
                let left = left + (width / 2.0) - (arrow_size / 2.0);
                (top, left, 45)
            }
            Positioning::BottomEnd => {
                let top = top + height + arrow_size - (arrow_size / 2.0);
                let left = left + (width / 2.0) - (arrow_size / 2.0);
                (top, left, 45)
            }
            Positioning::Left => {
                let top = top + (height / 2.0) - (arrow_size / 2.0);
                let left = left - arrow_size - (arrow_size / 2.0);
                (top, left, 135)
            }
            Positioning::LeftStart => {
                let top = top + (height / 2.0) - (arrow_size / 2.0);
                let left = left - arrow_size - (arrow_size / 2.0);
                (top, left, 135)
            }
            Positioning::LeftEnd => {
                let top = top + (height / 2.0) - (arrow_size / 2.0);
                let left = left - arrow_size - (arrow_size / 2.0);
                (top, left, 135)
            }
        }
    }
}

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
    let show_class = cn!(class, show_class);
    let hide_class = cn!(class, hide_class);

    let tooltip_ctx = expect_context::<TooltipContext>();

    let content_ref = tooltip_ctx.content_ref;

    let hide_delay = tooltip_ctx.hide_delay;
    let when = tooltip_ctx.open;

    let show_handle: StoredValue<Option<TimeoutHandle>> = StoredValue::new(None);
    let hide_handle: StoredValue<Option<TimeoutHandle>> = StoredValue::new(None);
    let cls = RwSignal::new(if when.get_untracked() {
        show_class.clone()
    } else {
        hide_class.clone()
    });
    let show = RwSignal::new(when.get_untracked());

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

    let style = move || {
        // with!(|top, left, width, height, content_width, content_height| {
        tooltip_ctx.positioning.calculate_position_style(
            top.get().clone(),
            left.get().clone(),
            width.get().clone(),
            height.get().clone(),
            content_height.get().clone(),
            content_width.get().clone(),
            tooltip_ctx.arrow_size as f64,
        )
        // })
    };

    let _ = RenderEffect::new(move |_| {
        let show_class = show_class.clone();
        if when.get() {
            // clear any possibly active timer
            if let Some(h) = show_handle.get_value() {
                h.clear();
            }
            if let Some(h) = hide_handle.get_value() {
                h.clear();
            }

            let h = leptos_dom::helpers::set_timeout_with_handle(
                move || cls.set(show_class.clone()),
                Duration::from_millis(1),
            )
            .expect("set timeout in AnimatedShow");
            show_handle.set_value(Some(h));

            cls.set(hide_class.clone());
            show.set(true);
        } else {
            cls.set(hide_class.clone());

            let h =
                leptos_dom::helpers::set_timeout_with_handle(move || show.set(false), hide_delay)
                    .expect("set timeout in AnimatedShow");
            hide_handle.set_value(Some(h));
        }
    });

    on_cleanup(move || {
        if let Some(Some(h)) = show_handle.try_get_value() {
            h.clear();
        }
        if let Some(Some(h)) = hide_handle.try_get_value() {
            h.clear();
        }
    });

    view! {
        <Show when={move || show.get()} fallback={|| ()}>
            <div node_ref={content_ref} class={move || cls.get()} style={style}>
                {children()}
            </div>
        </Show>
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
        >
        </div>
    }
}
