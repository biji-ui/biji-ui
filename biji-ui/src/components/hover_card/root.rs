use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use leptos::{context::Provider, ev::keydown, prelude::*};
use leptos_use::{
    UseElementBoundingReturn, use_document, use_element_bounding, use_event_listener,
};
use wasm_bindgen::JsCast;

use crate::{
    cn,
    custom_animated_show::CustomAnimatedShow,
    utils::positioning::{AvoidCollisions, Positioning},
};

use super::context::{HoverCardState, next_hover_card_id};

fn build_state(
    open: Option<RwSignal<bool>>,
    positioning: Positioning,
    avoid_collisions: AvoidCollisions,
    open_delay: Duration,
    close_delay: Duration,
    hide_delay: Duration,
    arrow_size: i32,
) -> HoverCardState {
    let open_arc = Arc::new(Mutex::new(None));
    let close_arc = Arc::new(Mutex::new(None));
    let open_sig = open.unwrap_or_else(|| RwSignal::new(false));
    HoverCardState {
        open: open_sig,
        data_state: Signal::derive(move || if open_sig.get() { "open" } else { "closed" }),
        trigger_ref: NodeRef::new(),
        content_ref: NodeRef::new(),
        open_delay,
        close_delay,
        hide_delay,
        positioning,
        avoid_collisions,
        arrow_size,
        hover_card_id: StoredValue::new(next_hover_card_id()),
        open_timer: StoredValue::new(open_arc),
        close_timer: StoredValue::new(close_arc),
    }
}

/// Returns the [`HoverCardState`] from the nearest [`Root`] or [`RootWith`] ancestor.
pub fn use_hover_card() -> HoverCardState {
    expect_context::<HoverCardState>()
}

/// The render-prop variant of [`Root`]. Use this when you need access to [`HoverCardState`]
/// directly inside the children via the `let:` binding.
#[component]
pub fn RootWith<IV: IntoView + 'static>(
    children: impl Fn(HoverCardState) -> IV + Send + Sync + 'static,
    #[prop(into, optional)] class: String,
    #[prop(default = Positioning::Bottom)] positioning: Positioning,
    #[prop(default = AvoidCollisions::Flip)] avoid_collisions: AvoidCollisions,
    #[prop(default = Duration::from_millis(700))] open_delay: Duration,
    #[prop(default = Duration::from_millis(300))] close_delay: Duration,
    #[prop(default = Duration::from_millis(200))] hide_delay: Duration,
    #[prop(default = 8)] arrow_size: i32,
    #[prop(into, default = None)] open: Option<RwSignal<bool>>,
) -> impl IntoView {
    let state = build_state(
        open,
        positioning,
        avoid_collisions,
        open_delay,
        close_delay,
        hide_delay,
        arrow_size,
    );
    on_cleanup(move || {
        state.cancel_open_timer();
        state.cancel_close_timer();
    });
    view! {
        <Provider value={state}>
            <RootEvents>
                <span class={class}>{children(state)}</span>
            </RootEvents>
        </Provider>
    }
}

/// The standard hover card root. Use [`RootWith`] instead when you need to access
/// [`HoverCardState`] inline via `let:h`.
#[component]
pub fn Root(
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
    #[prop(default = Positioning::Bottom)] positioning: Positioning,
    #[prop(default = AvoidCollisions::Flip)] avoid_collisions: AvoidCollisions,
    #[prop(default = Duration::from_millis(700))] open_delay: Duration,
    #[prop(default = Duration::from_millis(300))] close_delay: Duration,
    #[prop(default = Duration::from_millis(200))] hide_delay: Duration,
    #[prop(default = 8)] arrow_size: i32,
    #[prop(into, default = None)] open: Option<RwSignal<bool>>,
) -> impl IntoView {
    view! {
        <RootWith
            positioning={positioning}
            avoid_collisions={avoid_collisions}
            open_delay={open_delay}
            close_delay={close_delay}
            hide_delay={hide_delay}
            arrow_size={arrow_size}
            open={open}
            class={class}
            let:_
        >
            {children()}
        </RootWith>
    }
}

#[component]
fn RootEvents(children: Children) -> impl IntoView {
    let ctx = expect_context::<HoverCardState>();

    let _ = use_event_listener(use_document(), keydown, move |evt| {
        if evt.key() == "Escape" && ctx.open.get() {
            ctx.close_immediate();
        }
    });

    children()
}

#[component]
pub fn Trigger(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let ctx = expect_context::<HoverCardState>();

    view! {
        <TriggerEvents>
            <span
                node_ref={ctx.trigger_ref}
                aria-describedby={move || ctx.open.get().then(|| ctx.hover_card_id.get_value())}
                data-state={ctx.data_state}
                class={class}
            >
                {children()}
            </span>
        </TriggerEvents>
    }
}

#[component]
fn TriggerEvents(children: Children) -> impl IntoView {
    let ctx = expect_context::<HoverCardState>();

    let _ = use_event_listener(ctx.trigger_ref, leptos::ev::pointerenter, move |_| {
        ctx.cancel_close_timer();
        ctx.schedule_open();
    });

    let _ = use_event_listener(ctx.trigger_ref, leptos::ev::pointerleave, move |_| {
        ctx.cancel_open_timer();
        ctx.schedule_close();
    });

    let _ = use_event_listener(ctx.content_ref, leptos::ev::pointerenter, move |_| {
        ctx.cancel_close_timer();
    });

    let _ = use_event_listener(ctx.content_ref, leptos::ev::pointerleave, move |_| {
        ctx.schedule_close();
    });

    let _ = use_event_listener(ctx.trigger_ref, leptos::ev::focus, move |_| {
        ctx.cancel_close_timer();
        ctx.schedule_open();
    });

    let _ = use_event_listener(ctx.trigger_ref, leptos::ev::blur, move |_| {
        ctx.cancel_open_timer();
        ctx.schedule_close();
    });

    children()
}

#[component]
pub fn Content(
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] show_class: String,
    #[prop(into, optional)] hide_class: String,
) -> impl IntoView {
    let ctx = expect_context::<HoverCardState>();
    let content_ref = ctx.content_ref;

    let UseElementBoundingReturn {
        width: content_width,
        height: content_height,
        ..
    } = use_element_bounding(content_ref);

    let UseElementBoundingReturn {
        top,
        left,
        width,
        height,
        ..
    } = use_element_bounding(ctx.trigger_ref);

    let style_signal = Signal::derive(move || {
        let raw_cw = *content_width.read();
        let raw_ch = *content_height.read();
        let _ = ctx.open.get();
        let _ = top.read();
        let _ = left.read();
        let _ = width.read();
        let _ = height.read();
        let hidden = || {
            format!(
                "position: fixed; top: 0; left: 0; visibility: hidden; --biji-transform-origin: {};",
                ctx.positioning.transform_origin()
            )
        };
        if raw_cw == 0.0 && raw_ch == 0.0 {
            return hidden();
        }
        let Some(content_div) = content_ref.get_untracked() else {
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
        let Some(trigger) = ctx.trigger_ref.get_untracked() else {
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
        let eff = ctx.positioning.effective_positioning(
            cw,
            ch,
            t_top,
            t_left,
            t_width,
            t_height,
            ctx.arrow_size as f64,
            vp_w,
            vp_h,
            ctx.avoid_collisions,
        );
        eff.calculate_position_style(
            t_top,
            t_left,
            t_width,
            t_height,
            ch,
            cw,
            ctx.arrow_size as f64,
            ctx.arrow_size as f64,
        )
    });

    view! {
        <CustomAnimatedShow
            when={ctx.open}
            show_class={cn!(class, show_class)}
            hide_class={cn!(class, hide_class)}
            hide_delay={ctx.hide_delay}
            style_signal={style_signal}
            node_ref={content_ref}
            attr:id={ctx.hover_card_id.get_value()}
        >
            {children()}
        </CustomAnimatedShow>
    }
}

#[component]
pub fn Arrow(#[prop(into, optional)] class: String) -> impl IntoView {
    let ctx = expect_context::<HoverCardState>();
    view! {
        <div
            class={class}
            style={move || {
                format!(
                    "position: fixed; top: var(--biji-tooltip-arrow-top); left: var(--biji-tooltip-arrow-left); height: {}px; width: {}px; background-color: inherit; transform: rotate(var(--biji-tooltip-arrow-rotation));",
                    ctx.arrow_size,
                    ctx.arrow_size,
                )
            }}
        ></div>
    }
}
