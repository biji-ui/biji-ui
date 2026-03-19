use std::{sync::{Arc, Mutex}, time::Duration};

use leptos::{
    context::Provider,
    ev::{click, keydown},
    prelude::*,
};
use leptos_use::{on_click_outside, use_document, use_event_listener};
use wasm_bindgen::JsCast;

use crate::{
    cn,
    custom_animated_show::CustomAnimatedShow,
    utils::prevent_scroll::use_prevent_scroll,
};

use super::context::{DrawerState, DrawerSide, next_drawer_id};

fn build_state(
    open: bool,
    side: DrawerSide,
    prevent_scroll: bool,
    hide_delay: Duration,
    on_open_change: Option<Callback<bool>>,
) -> DrawerState {
    let open_sig = RwSignal::new(open);
    let base_id = next_drawer_id();
    DrawerState {
        open: open_sig,
        data_state: Signal::derive(move || if open_sig.get() { "open" } else { "closed" }),
        side,
        trigger_ref: NodeRef::new(),
        overlay_ref: NodeRef::new(),
        content_ref: NodeRef::new(),
        prevent_scroll,
        hide_delay,
        title_id: StoredValue::new(format!("{base_id}-title")),
        description_id: StoredValue::new(format!("{base_id}-description")),
        drawer_id: StoredValue::new(base_id),
        on_open_change,
    }
}

/// Returns the [`DrawerState`] from the nearest [`Root`] or [`RootWith`] ancestor.
pub fn use_drawer() -> DrawerState {
    expect_context::<DrawerState>()
}

/// The render-prop variant of [`Root`]. Use this when you need access to [`DrawerState`]
/// directly inside the children via the `let:` binding.
///
/// ```rust
/// <drawer::RootWith let:d>
///     <p>{move || if d.open.get() { "Drawer open" } else { "Drawer closed" }}</p>
///     <drawer::Trigger>"Open"</drawer::Trigger>
///     <drawer::Content>
///         <drawer::Close>"Close"</drawer::Close>
///     </drawer::Content>
/// </drawer::RootWith>
/// ```
#[component]
pub fn RootWith<IV: IntoView + 'static>(
    children: impl Fn(DrawerState) -> IV + Send + Sync + 'static,
    #[prop(into, optional)] class: String,
    #[prop(default = DrawerSide::Right)] side: DrawerSide,
    #[prop(default = true)] prevent_scroll: bool,
    /// Animation unmount delay — should match your CSS transition duration.
    #[prop(default = Duration::from_millis(300))]
    hide_delay: Duration,
    #[prop(default = false)] open: bool,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
) -> impl IntoView {
    let state = build_state(open, side, prevent_scroll, hide_delay, on_open_change);

    view! {
        <Provider value={state}>
            <RootEvents>
                <div class={class}>{children(state)}</div>
            </RootEvents>
        </Provider>
    }
}

/// The standard drawer root. Use [`RootWith`] instead when you need to access
/// [`DrawerState`] inline via `let:d`.
#[component]
pub fn Root(
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
    #[prop(default = DrawerSide::Right)] side: DrawerSide,
    #[prop(default = true)] prevent_scroll: bool,
    /// Animation unmount delay — should match your CSS transition duration.
    #[prop(default = Duration::from_millis(300))]
    hide_delay: Duration,
    #[prop(default = false)] open: bool,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
) -> impl IntoView {
    let state = build_state(open, side, prevent_scroll, hide_delay, on_open_change);

    view! {
        <Provider value={state}>
            <RootEvents>
                <div class={class}>{children()}</div>
            </RootEvents>
        </Provider>
    }
}

#[component]
fn RootEvents(children: Children) -> impl IntoView {
    let ctx = expect_context::<DrawerState>();

    let eff = use_prevent_scroll(
        move || ctx.prevent_scroll && ctx.open.get(),
        ctx.hide_delay,
    );
    on_cleanup(move || drop(eff));

    // Escape: close and return focus to the trigger.
    let _ = use_event_listener(use_document(), keydown, move |evt| {
        if evt.key() == "Escape" && ctx.open.get() {
            ctx.close();
            if let Some(trigger) = ctx.trigger_ref.get() {
                let _ = trigger.focus();
            }
        }
    });

    // Click outside the panel: close (also handles overlay clicks when no Overlay
    // component is used).
    let _ = on_click_outside(ctx.content_ref, move |evt| {
        if !ctx.open.get() {
            return;
        }
        let is_trigger_click = evt
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            .zip(ctx.trigger_ref.get())
            .is_some_and(|(el, trigger)| {
                let node: &web_sys::Node = trigger.as_ref();
                el.is_same_node(Some(node)) || trigger.contains(Some(&el))
            });
        if !is_trigger_click {
            ctx.close();
        }
    });

    children()
}

#[component]
pub fn Trigger(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let ctx = expect_context::<DrawerState>();

    let _ = use_event_listener(ctx.trigger_ref, click, move |_| {
        ctx.toggle();
    });

    view! {
        <button
            node_ref={ctx.trigger_ref}
            type="button"
            aria-expanded={move || if ctx.open.get() { "true" } else { "false" }}
            aria-controls={ctx.drawer_id.get_value()}
            data-state={ctx.data_state}
            class={class}
        >
            {children()}
        </button>
    }
}

/// A semi-transparent backdrop rendered behind the drawer panel.  Clicking it
/// closes the drawer.  Use inside a `<Portal>` if you want it to cover the full
/// viewport.
#[component]
pub fn Overlay(
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] show_class: String,
    #[prop(into, optional)] hide_class: String,
) -> impl IntoView {
    let ctx = expect_context::<DrawerState>();

    view! {
        <CustomAnimatedShow
            when={ctx.open}
            show_class={cn!(class, show_class)}
            hide_class={cn!(class, hide_class)}
            hide_delay={ctx.hide_delay}
            node_ref={ctx.overlay_ref}
        >
            <div on:click={move |_| {
                ctx.close();
                if let Some(trigger) = ctx.trigger_ref.get() {
                    let _ = trigger.focus();
                }
            }} style="position: absolute; inset: 0;"></div>
        </CustomAnimatedShow>
    }
}

/// The sliding panel.  Position it with CSS classes; use `show_class` /
/// `hide_class` for the slide-in animation.  Use inside a `<Portal>`.
#[component]
pub fn Content(
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] show_class: String,
    #[prop(into, optional)] hide_class: String,
) -> impl IntoView {
    let ctx = expect_context::<DrawerState>();
    let content_ref = ctx.content_ref;

    // Focus the first focusable element when the drawer opens.
    // Store the handle so rapid open/close doesn't fire a stale focus.
    let focus_handle: Arc<Mutex<Option<leptos::leptos_dom::helpers::TimeoutHandle>>> =
        Arc::new(Mutex::new(None));
    let focus_handle_cleanup = Arc::clone(&focus_handle);
    let _focus_eff = RenderEffect::new(move |_| {
        if ctx.open.get() {
            if let Some(h) = focus_handle.lock().unwrap().take() {
                h.clear();
            }
            let handle = leptos::leptos_dom::helpers::set_timeout_with_handle(
                move || {
                    if let Some(el) = content_ref.get() {
                        focus_first_element(&el);
                    }
                },
                Duration::from_millis(10),
            );
            if let Ok(h) = handle {
                *focus_handle.lock().unwrap() = Some(h);
            }
        }
    });
    on_cleanup(move || {
        drop(_focus_eff);
        if let Some(h) = focus_handle_cleanup.lock().unwrap().take() {
            h.clear();
        }
    });

    // Tab / Shift+Tab focus trap.
    let _ = use_event_listener(content_ref, keydown, move |evt| {
        if evt.key() == "Tab" {
            if let Some(el) = content_ref.get() {
                trap_tab_focus(&el, evt.shift_key(), &evt);
            }
        }
    });

    view! {
        <CustomAnimatedShow
            when={ctx.open}
            show_class={cn!(class, show_class)}
            hide_class={cn!(class, hide_class)}
            hide_delay={ctx.hide_delay}
            node_ref={content_ref}
            attr:id={ctx.drawer_id.get_value()}
            attr:role="dialog"
            attr:aria-modal="true"
            attr:aria-labelledby={ctx.title_id.get_value()}
            attr:aria-describedby={ctx.description_id.get_value()}
            attr:data-state={ctx.data_state}
            attr:data-side={ctx.side.as_str()}
            attr:tabindex="-1"
        >
            {children()}
        </CustomAnimatedShow>
    }
}

/// A button that closes the drawer when clicked.
#[component]
pub fn Close(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let ctx = expect_context::<DrawerState>();

    view! {
        <button type="button" class={class} on:click={move |_| {
            ctx.close();
            if let Some(trigger) = ctx.trigger_ref.get() {
                let _ = trigger.focus();
            }
        }}>
            {children()}
        </button>
    }
}

/// Semantic heading for the drawer panel — renders as `<h2>`.
/// Its `id` is automatically wired to `aria-labelledby` on `Content`.
#[component]
pub fn Title(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let ctx = expect_context::<DrawerState>();
    view! { <h2 id={ctx.title_id.get_value()} class={class}>{children()}</h2> }
}

/// Supplementary description for the drawer panel — renders as `<p>`.
/// Its `id` is automatically wired to `aria-describedby` on `Content`.
#[component]
pub fn Description(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let ctx = expect_context::<DrawerState>();
    view! { <p id={ctx.description_id.get_value()} class={class}>{children()}</p> }
}

// ── Focus helpers ─────────────────────────────────────────────────────────────

fn get_focusable_elements(container: &web_sys::HtmlElement) -> Vec<web_sys::HtmlElement> {
    let selector = r#"a[href], button:not([disabled]), textarea:not([disabled]), input:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex="-1"])"#;
    let Ok(node_list) = container.query_selector_all(selector) else {
        return Vec::new();
    };
    let mut elements = Vec::new();
    for i in 0..node_list.length() {
        if let Some(node) = node_list.get(i) {
            if let Ok(el) = node.dyn_into::<web_sys::HtmlElement>() {
                elements.push(el);
            }
        }
    }
    elements
}

fn focus_first_element(container: &web_sys::HtmlElement) {
    let focusable = get_focusable_elements(container);
    if let Some(first) = focusable.first() {
        let _ = first.focus();
    } else {
        let _ = container.focus();
    }
}

fn trap_tab_focus(
    container: &web_sys::HtmlElement,
    shift_key: bool,
    evt: &web_sys::KeyboardEvent,
) {
    let focusable = get_focusable_elements(container);
    if focusable.is_empty() {
        evt.prevent_default();
        return;
    }
    let active = document().active_element();
    if shift_key {
        let is_first = active
            .as_ref()
            .and_then(|a| a.dyn_ref::<web_sys::HtmlElement>())
            .map(|a| {
                focusable.first().map(|f| {
                    let a_node: &web_sys::Node = a.as_ref();
                    let f_node: &web_sys::Node = f.as_ref();
                    a_node.is_same_node(Some(f_node))
                }).unwrap_or(false)
            })
            .unwrap_or(true);
        if is_first {
            evt.prevent_default();
            if let Some(last) = focusable.last() {
                let _ = last.focus();
            }
        }
    } else {
        let is_last = active
            .as_ref()
            .and_then(|a| a.dyn_ref::<web_sys::HtmlElement>())
            .map(|a| {
                focusable.last().map(|f| {
                    let a_node: &web_sys::Node = a.as_ref();
                    let f_node: &web_sys::Node = f.as_ref();
                    a_node.is_same_node(Some(f_node))
                }).unwrap_or(false)
            })
            .unwrap_or(true);
        if is_last {
            evt.prevent_default();
            if let Some(first) = focusable.first() {
                let _ = first.focus();
            }
        }
    }
}
