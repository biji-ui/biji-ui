use std::{
    sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}},
    time::Duration,
};

use leptos::{context::Provider, ev::click, leptos_dom::helpers::TimeoutHandle, prelude::*};
use leptos_use::use_event_listener;
use wasm_bindgen::JsCast;

use crate::{
    cn, custom_animated_show::CustomAnimatedShow, utils::prevent_scroll::use_prevent_scroll,
};

use super::context::AlertDialogState;

static ALERT_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn next_alert_ids() -> (String, String) {
    let id = ALERT_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    (
        format!("biji-alert-title-{id}"),
        format!("biji-alert-desc-{id}"),
    )
}

fn build_state(
    open: bool,
    prevent_scroll: bool,
    hide_delay: Duration,
    on_open_change: Option<Callback<bool>>,
) -> AlertDialogState {
    let open_sig = RwSignal::new(open);
    let (title_id, desc_id) = next_alert_ids();
    AlertDialogState {
        open: open_sig,
        data_state: Signal::derive(move || if open_sig.get() { "open" } else { "closed" }),
        trigger_ref: NodeRef::new(),
        content_ref: NodeRef::new(),
        cancel_ref: NodeRef::new(),
        prevent_scroll,
        hide_delay,
        title_id: StoredValue::new(title_id),
        desc_id: StoredValue::new(desc_id),
        on_open_change,
    }
}

/// Returns the [`AlertDialogState`] from the nearest [`Root`] or [`RootWith`] ancestor.
pub fn use_alert_dialog() -> AlertDialogState {
    expect_context::<AlertDialogState>()
}

/// The render-prop variant of [`Root`]. Use this when you need access to [`AlertDialogState`]
/// directly inside the children via the `let:` binding.
///
/// ```rust
/// <alert_dialog::RootWith let:d>
///     <p>{move || if d.open.get() { "Alert open" } else { "Alert closed" }}</p>
///     <alert_dialog::Trigger>"Delete"</alert_dialog::Trigger>
///     <alert_dialog::Overlay />
///     <alert_dialog::Content>
///         <alert_dialog::Cancel>"Cancel"</alert_dialog::Cancel>
///         <alert_dialog::Action>"Confirm"</alert_dialog::Action>
///     </alert_dialog::Content>
/// </alert_dialog::RootWith>
/// ```
#[component]
pub fn RootWith<IV: IntoView + 'static>(
    children: impl Fn(AlertDialogState) -> IV + Send + Sync + 'static,
    #[prop(into, optional)] class: String,
    #[prop(default = true)] prevent_scroll: bool,
    #[prop(default = Duration::from_millis(200))] hide_delay: Duration,
    #[prop(default = false)] open: bool,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
) -> impl IntoView {
    let state = build_state(open, prevent_scroll, hide_delay, on_open_change);

    view! {
        <Provider value={state}>
            <RootEvents>
                <div class={class}>{children(state)}</div>
            </RootEvents>
        </Provider>
    }
}

/// The standard alert dialog root. Use [`RootWith`] instead when you need to access
/// [`AlertDialogState`] inline via `let:d`.
#[component]
pub fn Root(
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
    #[prop(default = true)] prevent_scroll: bool,
    #[prop(default = Duration::from_millis(200))] hide_delay: Duration,
    #[prop(default = false)] open: bool,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
) -> impl IntoView {
    let state = build_state(open, prevent_scroll, hide_delay, on_open_change);

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
    let ctx = expect_context::<AlertDialogState>();

    let eff = use_prevent_scroll(move || ctx.prevent_scroll && ctx.open.get(), ctx.hide_delay);

    on_cleanup(move || {
        drop(eff);
    });

    children()
}

#[component]
pub fn Trigger(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let ctx = expect_context::<AlertDialogState>();

    let _ = use_event_listener(ctx.trigger_ref, click, move |_| {
        ctx.open();
    });

    view! {
        <button
            node_ref={ctx.trigger_ref}
            type="button"
            data-state={ctx.data_state}
            class={class}
        >
            {children()}
        </button>
    }
}

#[component]
pub fn Overlay(
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] show_class: String,
    #[prop(into, optional)] hide_class: String,
) -> impl IntoView {
    let ctx = expect_context::<AlertDialogState>();

    view! {
        <CustomAnimatedShow
            when={ctx.open}
            show_class={cn!(class, show_class)}
            hide_class={cn!(class, hide_class)}
            hide_delay={ctx.hide_delay}
        >
            <div aria-hidden="true"></div>
        </CustomAnimatedShow>
    }
}

#[component]
pub fn Content(
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] show_class: String,
    #[prop(into, optional)] hide_class: String,
) -> impl IntoView {
    let ctx = expect_context::<AlertDialogState>();
    let content_ref = ctx.content_ref;

    let focus_handle: Arc<Mutex<Option<TimeoutHandle>>> = Arc::new(Mutex::new(None));
    let focus_handle_cleanup = Arc::clone(&focus_handle);
    let focus_eff = RenderEffect::new(move |_| {
        // Cancel any pending focus timeout before scheduling a new one.
        if let Some(h) = focus_handle.lock().unwrap().take() {
            h.clear();
        }
        if ctx.open.get() {
            let fh = Arc::clone(&focus_handle);
            let h = leptos::leptos_dom::helpers::set_timeout_with_handle(
                move || {
                    *fh.lock().unwrap() = None;
                    if let Some(cancel) = ctx.cancel_ref.get() {
                        let _ = cancel.focus();
                    } else if let Some(el) = content_ref.get() {
                        focus_first_element(&el);
                    }
                },
                Duration::from_millis(10),
            )
            .expect("set_timeout in alert_dialog focus");
            *focus_handle.lock().unwrap() = Some(h);
        }
    });
    on_cleanup(move || {
        if let Some(h) = focus_handle_cleanup.lock().unwrap().take() {
            h.clear();
        }
        drop(focus_eff);
    });

    let _ = use_event_listener(content_ref, leptos::ev::keydown, move |evt| {
        match evt.key().as_str() {
            "Escape" => {
                evt.prevent_default();
                ctx.close();
                if let Some(trigger) = ctx.trigger_ref.get() {
                    let _ = trigger.focus();
                }
            }
            "Tab" => {
                if let Some(el) = content_ref.get() {
                    trap_tab_focus(&el, evt.shift_key(), &evt);
                }
            }
            _ => {}
        }
    });

    view! {
        <CustomAnimatedShow
            when={ctx.open}
            show_class={cn!(class, show_class)}
            hide_class={cn!(class, hide_class)}
            hide_delay={ctx.hide_delay}
            node_ref={content_ref}
            attr:role="alertdialog"
            attr:aria-modal="true"
            attr:aria-labelledby={ctx.title_id.get_value()}
            attr:aria-describedby={ctx.desc_id.get_value()}
            attr:tabindex="-1"
        >
            {children()}
        </CustomAnimatedShow>
    }
}

#[component]
pub fn Title(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let ctx = expect_context::<AlertDialogState>();
    view! {
        <h2 id={ctx.title_id.get_value()} class={class}>
            {children()}
        </h2>
    }
}

#[component]
pub fn Description(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let ctx = expect_context::<AlertDialogState>();
    view! {
        <p id={ctx.desc_id.get_value()} class={class}>
            {children()}
        </p>
    }
}

#[component]
pub fn Cancel(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let ctx = expect_context::<AlertDialogState>();

    let _ = use_event_listener(ctx.cancel_ref, click, move |_| {
        ctx.close();
        if let Some(trigger) = ctx.trigger_ref.get() {
            let _ = trigger.focus();
        }
    });

    view! {
        <button node_ref={ctx.cancel_ref} type="button" class={class}>
            {children()}
        </button>
    }
}

#[component]
pub fn Action(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    view! {
        <button type="button" class={class}>
            {children()}
        </button>
    }
}

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

fn trap_tab_focus(container: &web_sys::HtmlElement, shift_key: bool, evt: &web_sys::KeyboardEvent) {
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
                focusable
                    .first()
                    .map(|f| {
                        let a_node: &web_sys::Node = a.as_ref();
                        let f_node: &web_sys::Node = f.as_ref();
                        a_node.is_same_node(Some(f_node))
                    })
                    .unwrap_or(false)
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
                focusable
                    .last()
                    .map(|f| {
                        let a_node: &web_sys::Node = a.as_ref();
                        let f_node: &web_sys::Node = f.as_ref();
                        a_node.is_same_node(Some(f_node))
                    })
                    .unwrap_or(false)
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
