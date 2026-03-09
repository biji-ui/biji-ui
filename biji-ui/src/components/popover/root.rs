use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use leptos::{context::Provider, ev::keydown, leptos_dom::helpers::TimeoutHandle, prelude::*};
use leptos_use::{
    UseElementBoundingReturn, on_click_outside, use_document, use_element_bounding,
    use_event_listener,
};
use wasm_bindgen::JsCast;

use crate::{
    cn,
    custom_animated_show::CustomAnimatedShow,
    utils::positioning::{AvoidCollisions, Positioning},
};

use super::context::PopoverContext;

static POPOVER_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn next_popover_id() -> String {
    let id = POPOVER_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("biji-popover-{id}")
}

#[component]
pub fn Root(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(default = Positioning::Bottom)] positioning: Positioning,
    #[prop(default = Duration::from_millis(200))] hide_delay: Duration,
    #[prop(default = 8)] arrow_size: i32,
    #[prop(default = false)] open: bool,
    #[prop(default = AvoidCollisions::Flip)] avoid_collisions: AvoidCollisions,
    /// When true, focuses the first focusable element inside Content when the popover opens.
    #[prop(default = true)]
    auto_focus: bool,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
) -> impl IntoView {
    let ctx = PopoverContext {
        open: RwSignal::new(open),
        hide_delay,
        positioning,
        arrow_size,
        popover_id: StoredValue::new(next_popover_id()),
        avoid_collisions,
        auto_focus,
        on_open_change,
        ..PopoverContext::default()
    };

    view! {
        <Provider value={ctx}>
            <RootEvents>
                <div class={class}>{children()}</div>
            </RootEvents>
        </Provider>
    }
}

#[component]
fn RootEvents(children: Children) -> impl IntoView {
    let ctx = expect_context::<PopoverContext>();

    let _ = use_event_listener(use_document(), keydown, move |evt| {
        if evt.key() == "Escape" && ctx.open.get() {
            ctx.close();
            if let Some(trigger) = ctx.trigger_ref.get() {
                let _ = trigger.focus();
            }
        }
    });

    let _ = on_click_outside(ctx.content_ref, move |evt| {
        if !ctx.open.get() {
            return;
        }
        let is_trigger_click = evt
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            .zip(ctx.trigger_ref.get())
            .is_some_and(|(el, trigger)| {
                let trigger_node: &web_sys::Node = trigger.as_ref();
                el.is_same_node(Some(trigger_node)) || trigger.contains(Some(&el))
            });
        if !is_trigger_click {
            ctx.close();
        }
    });

    children()
}

#[component]
pub fn Trigger(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let ctx = expect_context::<PopoverContext>();

    let _ = use_event_listener(ctx.trigger_ref, leptos::ev::click, move |_| {
        ctx.toggle();
    });

    view! {
        <button
            node_ref={ctx.trigger_ref}
            type="button"
            aria-expanded={move || if ctx.open.get() { "true" } else { "false" }}
            aria-controls={ctx.popover_id.get_value()}
            data-state={move || ctx.data_state()}
            class={class}
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
) -> impl IntoView {
    let ctx = expect_context::<PopoverContext>();
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
        // Use use_element_bounding signals only for reactive dependencies.
        // They fire when the element mounts/unmounts (zero → non-zero transition
        // tells us the content is now in the DOM).
        let raw_cw = *content_width.read();
        let raw_ch = *content_height.read();
        // Re-run when open state changes so we pick up the fresh trigger position.
        let _ = ctx.open.get();
        // Establish reactive dependencies on trigger bounding signals so that
        // scroll / resize events cause the position to recompute.
        let _ = top.read();
        let _ = left.read();
        let _ = width.read();
        let _ = height.read();
        // use_element_bounding reports 0 when the element is not yet in the DOM.
        let hidden = || format!(
            "position: fixed; top: 0; left: 0; visibility: hidden; --biji-transform-origin: {};",
            ctx.positioning.transform_origin()
        );
        if raw_cw == 0.0 && raw_ch == 0.0 {
            return hidden();
        }
        // Use offsetWidth/offsetHeight for content dimensions: unlike
        // getBoundingClientRect (used by use_element_bounding), these are not
        // affected by CSS transforms.  This prevents the initial scale-95
        // animation class from producing shrunken dimensions (e.g. 273px instead
        // of 288px for w-72 at 0.95 scale).
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
        // Read the trigger's bounding rect fresh from the DOM.  use_element_bounding
        // can return a stale value from initial hydration when the page layout shifts
        // before the first scroll/resize event fires.
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

    let focus_handle: Arc<Mutex<Option<TimeoutHandle>>> = Arc::new(Mutex::new(None));
    let focus_handle_cleanup = Arc::clone(&focus_handle);
    let focus_eff = RenderEffect::new(move |_| {
        // Cancel any pending focus timeout before scheduling a new one.
        if let Some(h) = focus_handle.lock().unwrap().take() {
            h.clear();
        }
        if ctx.open.get() && ctx.auto_focus {
            let fh = Arc::clone(&focus_handle);
            let h = leptos::leptos_dom::helpers::set_timeout_with_handle(
                move || {
                    *fh.lock().unwrap() = None;
                    if let Some(el) = content_ref.get() {
                        focus_first_element(&el);
                    }
                },
                Duration::from_millis(10),
            )
            .expect("set_timeout in popover focus");
            *focus_handle.lock().unwrap() = Some(h);
        }
    });

    on_cleanup(move || {
        if let Some(h) = focus_handle_cleanup.lock().unwrap().take() {
            h.clear();
        }
        drop(focus_eff);
    });

    view! {
        <CustomAnimatedShow
            when={ctx.open}
            show_class={cn!(class, show_class)}
            hide_class={cn!(class, hide_class)}
            hide_delay={ctx.hide_delay}
            style_signal={style_signal}
            node_ref={content_ref}
            attr:id={ctx.popover_id.get_value()}
            attr:role="dialog"
            attr:tabindex="-1"
        >
            {children()}
        </CustomAnimatedShow>
    }
}

fn focus_first_element(container: &web_sys::HtmlElement) {
    let selector = r#"a[href], button:not([disabled]), textarea:not([disabled]), input:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex="-1"])"#;
    let Ok(node_list) = container.query_selector_all(selector) else {
        return;
    };
    for i in 0..node_list.length() {
        if let Some(node) = node_list.get(i) {
            if let Ok(el) = node.dyn_into::<web_sys::HtmlElement>() {
                let _ = el.focus();
                return;
            }
        }
    }
    // No focusable child — focus the container itself (requires tabindex on it).
    let _ = container.focus();
}

#[component]
pub fn Arrow(#[prop(into, optional)] class: String) -> impl IntoView {
    let ctx = expect_context::<PopoverContext>();
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
