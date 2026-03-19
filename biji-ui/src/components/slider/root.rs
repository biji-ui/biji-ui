use leptos::{
    context::Provider,
    ev::{keydown, pointercancel, pointerdown, pointermove, pointerup},
    html::Div,
    prelude::*,
};
use leptos_use::use_event_listener;

use super::context::SliderState;

/// Returns the [`SliderState`] from the nearest [`Root`] or [`RootWith`] ancestor.
///
/// Call this inside any child component that needs access to slider state.
pub fn use_slider() -> SliderState {
    expect_context::<SliderState>()
}

/// The render-prop variant of [`Root`]. Use this when you need access to [`SliderState`]
/// directly inside the children via the `let:` binding.
///
/// ```rust
/// <slider::RootWith value=50.0 let:s>
///     <div class="flex justify-between text-sm mb-1">
///         <span>"Volume"</span>
///         <span>{move || s.value.get() as u32}</span>
///     </div>
///     <slider::Track ...>
///         <slider::Range ... />
///     </slider::Track>
///     <slider::Thumb ... />
/// </slider::RootWith>
/// ```
///
/// The `s: SliderState` binding is `Copy`, so it can be passed to child components as a prop.
#[component]
pub fn RootWith<IV: IntoView + 'static>(
    children: impl Fn(SliderState) -> IV + Send + Sync + 'static,
    #[prop(into, optional)] class: String,
    #[prop(default = 0.0)] value: f64,
    #[prop(default = 0.0)] min: f64,
    #[prop(default = 100.0)] max: f64,
    #[prop(default = 1.0)] step: f64,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let (min, max) = if min <= max { (min, max) } else { (max, min) };
    let state = SliderState::new(value, min, max, step, disabled);

    view! {
        <Provider value={state}>
            <div
                data-orientation="horizontal"
                data-disabled={state.disabled}
                data-state={state.data_state()}
                class={class}
            >
                {children(state)}
            </div>
        </Provider>
    }
}

/// The standard slider root. Renders a wrapper `<div>` with data attributes and provides
/// [`SliderState`] to all descendants via context.
///
/// Use [`RootWith`] instead when you need to access [`SliderState`] inline via `let:s`.
#[component]
pub fn Root(
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
    #[prop(default = 0.0)] value: f64,
    #[prop(default = 0.0)] min: f64,
    #[prop(default = 100.0)] max: f64,
    #[prop(default = 1.0)] step: f64,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    view! {
        <RootWith value=value min=min max=max step=step disabled=disabled class=class let:_>
            {children()}
        </RootWith>
    }
}

#[component]
pub fn Track(
    children: Children,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let state = expect_context::<SliderState>();

    view! {
        <div
            node_ref={state.track_ref}
            data-orientation="horizontal"
            data-disabled={state.disabled}
            class={class}
        >
            {children()}
        </div>
    }
}

#[component]
pub fn Range(#[prop(into, optional)] class: String) -> impl IntoView {
    let state = expect_context::<SliderState>();

    view! {
        <div
            data-orientation="horizontal"
            data-disabled={state.disabled}
            class={class}
            style={move || format!("left: 0%; right: {}%", 100.0 - state.percentage.get())}
        />
    }
}

#[component]
pub fn Thumb(#[prop(into, optional)] class: String) -> impl IntoView {
    let state = expect_context::<SliderState>();
    let thumb_ref: NodeRef<Div> = NodeRef::new();
    let is_dragging = RwSignal::new(false);

    let _ = use_event_listener(thumb_ref, pointerdown, move |evt| {
        if state.disabled {
            return;
        }
        evt.prevent_default();
        if let Some(el) = thumb_ref.get() {
            let _ = el.set_pointer_capture(evt.pointer_id());
            let _ = el.focus();
        }
        is_dragging.set(true);
    });

    let _ = use_event_listener(thumb_ref, pointermove, move |evt| {
        if !is_dragging.get() || state.disabled {
            return;
        }
        if let Some(track) = state.track_ref.get() {
            let rect = track.get_bounding_client_rect();
            let pct = (evt.client_x() as f64 - rect.left()) / rect.width();
            state.set_value_from_pct(pct);
        }
    });

    let _ = use_event_listener(thumb_ref, pointerup, move |_| {
        is_dragging.set(false);
    });

    let _ = use_event_listener(thumb_ref, pointercancel, move |_| {
        is_dragging.set(false);
    });

    let _ = use_event_listener(thumb_ref, keydown, move |evt| {
        if state.disabled {
            return;
        }
        let step = if state.step.is_finite() && state.step > 0.0 { state.step } else { 1.0 };
        let current = state.value.get();
        let new_value = match evt.key().as_str() {
            "ArrowRight" | "ArrowUp" => current + step,
            "ArrowLeft" | "ArrowDown" => current - step,
            "PageUp" => current + step * 10.0,
            "PageDown" => current - step * 10.0,
            "Home" => state.min,
            "End" => state.max,
            _ => return,
        };
        evt.prevent_default();
        state.value.set(new_value.clamp(state.min, state.max));
    });

    view! {
        <div
            node_ref={thumb_ref}
            role="slider"
            tabindex={if state.disabled { "-1" } else { "0" }}
            aria-valuemin={state.min.to_string()}
            aria-valuemax={state.max.to_string()}
            aria-valuenow={move || state.value.get().to_string()}
            aria-disabled={if state.disabled { Some("true") } else { None }}
            data-disabled={state.disabled}
            style={move || format!("left: {}%", state.percentage.get())}
            class={class}
        />
    }
}
