use leptos::{
    context::Provider,
    ev::{keydown, pointercancel, pointerdown, pointermove, pointerup},
    html::Div,
    prelude::*,
};
use leptos_use::use_event_listener;

use super::context::SliderContext;

#[component]
pub fn Root(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(default = 0.0)] value: f64,
    #[prop(default = 0.0)] min: f64,
    #[prop(default = 100.0)] max: f64,
    #[prop(default = 1.0)] step: f64,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] on_value_change: Option<Callback<f64>>,
) -> impl IntoView {
    let (min, max) = if min <= max { (min, max) } else { (max, min) };
    let ctx = SliderContext {
        value: RwSignal::new(value.clamp(min, max)),
        min,
        max,
        step,
        disabled,
        track_ref: NodeRef::new(),
    };

    provide_context(on_value_change);

    view! {
        <Provider value={ctx}>
            <div
                data-orientation="horizontal"
                data-disabled={ctx.disabled}
                data-state={ctx.data_state()}
                class={class}
            >
                {children()}
            </div>
        </Provider>
    }
}

#[component]
pub fn Track(
    children: Children,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let ctx = expect_context::<SliderContext>();

    view! {
        <div
            node_ref={ctx.track_ref}
            data-orientation="horizontal"
            data-disabled={ctx.disabled}
            class={class}
        >
            {children()}
        </div>
    }
}

#[component]
pub fn Range(#[prop(into, optional)] class: String) -> impl IntoView {
    let ctx = expect_context::<SliderContext>();

    view! {
        <div
            data-orientation="horizontal"
            data-disabled={ctx.disabled}
            class={class}
            style={move || format!("left: 0%; right: {}%", 100.0 - ctx.percentage())}
        />
    }
}

#[component]
pub fn Thumb(#[prop(into, optional)] class: String) -> impl IntoView {
    let ctx = expect_context::<SliderContext>();
    let on_value_change = use_context::<Option<Callback<f64>>>().flatten();
    let thumb_ref: NodeRef<Div> = NodeRef::new();
    let is_dragging = RwSignal::new(false);

    let _ = use_event_listener(thumb_ref, pointerdown, move |evt| {
        if ctx.disabled {
            return;
        }
        evt.prevent_default();
        if let Some(el) = thumb_ref.get() {
            let _ = el.set_pointer_capture(evt.pointer_id());
        }
        is_dragging.set(true);
    });

    let _ = use_event_listener(thumb_ref, pointermove, move |evt| {
        if !is_dragging.get() || ctx.disabled {
            return;
        }
        if let Some(track) = ctx.track_ref.get() {
            let rect = track.get_bounding_client_rect();
            let pct = (evt.client_x() as f64 - rect.left()) / rect.width();
            ctx.set_value_from_pct(pct);
            if let Some(cb) = on_value_change {
                cb.run(ctx.value.get());
            }
        }
    });

    let _ = use_event_listener(thumb_ref, pointerup, move |_| {
        is_dragging.set(false);
    });

    let _ = use_event_listener(thumb_ref, pointercancel, move |_| {
        is_dragging.set(false);
    });

    let _ = use_event_listener(thumb_ref, keydown, move |evt| {
        if ctx.disabled {
            return;
        }
        let current = ctx.value.get();
        let new_value = match evt.key().as_str() {
            "ArrowRight" | "ArrowUp" => current + ctx.step,
            "ArrowLeft" | "ArrowDown" => current - ctx.step,
            "PageUp" => current + ctx.step * 10.0,
            "PageDown" => current - ctx.step * 10.0,
            "Home" => ctx.min,
            "End" => ctx.max,
            _ => return,
        };
        evt.prevent_default();
        ctx.value.set(new_value.clamp(ctx.min, ctx.max));
        if let Some(cb) = on_value_change {
            cb.run(ctx.value.get());
        }
    });

    view! {
        <div
            node_ref={thumb_ref}
            role="slider"
            tabindex={if ctx.disabled { "-1" } else { "0" }}
            aria-valuemin={ctx.min.to_string()}
            aria-valuemax={ctx.max.to_string()}
            aria-valuenow={move || ctx.value.get().to_string()}
            aria-disabled={if ctx.disabled { Some("true") } else { None }}
            data-disabled={ctx.disabled}
            style={move || format!("left: {}%", ctx.percentage())}
            class={class}
        />
    }
}
