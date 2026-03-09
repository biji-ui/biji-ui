use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

use leptos::{context::Provider, ev::click, prelude::*};
use leptos_use::use_event_listener;
use wasm_bindgen::JsCast;

use crate::{
    cn, custom_animated_show::CustomAnimatedShow, utils::prevent_scroll::use_prevent_scroll,
};

use super::context::AlertDialogContext;

static ALERT_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn next_alert_ids() -> (String, String) {
    let id = ALERT_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    (
        format!("biji-alert-title-{id}"),
        format!("biji-alert-desc-{id}"),
    )
}

#[component]
pub fn Root(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(default = true)] prevent_scroll: bool,
    #[prop(default = Duration::from_millis(200))] hide_delay: Duration,
    #[prop(default = false)] open: bool,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
) -> impl IntoView {
    let (title_id, desc_id) = next_alert_ids();
    let ctx = AlertDialogContext {
        open: RwSignal::new(open),
        prevent_scroll,
        hide_delay,
        title_id: StoredValue::new(title_id),
        desc_id: StoredValue::new(desc_id),
        on_open_change,
        ..AlertDialogContext::default()
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
    let ctx = expect_context::<AlertDialogContext>();

    let eff = use_prevent_scroll(move || ctx.prevent_scroll && ctx.open.get(), ctx.hide_delay);

    on_cleanup(move || {
        drop(eff);
    });

    children()
}

#[component]
pub fn Trigger(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let ctx = expect_context::<AlertDialogContext>();

    let _ = use_event_listener(ctx.trigger_ref, click, move |_| {
        ctx.open();
    });

    view! {
        <button
            node_ref={ctx.trigger_ref}
            type="button"
            data-state={move || ctx.data_state()}
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
    let ctx = expect_context::<AlertDialogContext>();

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
    let ctx = expect_context::<AlertDialogContext>();
    let content_ref = ctx.content_ref;

    let _focus_eff = RenderEffect::new(move |_| {
        if ctx.open.get() {
            let _ = leptos::leptos_dom::helpers::set_timeout_with_handle(
                move || {
                    if let Some(cancel) = ctx.cancel_ref.get() {
                        let _ = cancel.focus();
                    } else if let Some(el) = content_ref.get() {
                        focus_first_element(&el);
                    }
                },
                Duration::from_millis(10),
            );
        }
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

    on_cleanup(move || {
        drop(_focus_eff);
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
        >
            {children()}
        </CustomAnimatedShow>
    }
}

#[component]
pub fn Title(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let ctx = expect_context::<AlertDialogContext>();
    view! {
        <h2 id={ctx.title_id.get_value()} class={class}>
            {children()}
        </h2>
    }
}

#[component]
pub fn Description(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let ctx = expect_context::<AlertDialogContext>();
    view! {
        <p id={ctx.desc_id.get_value()} class={class}>
            {children()}
        </p>
    }
}

#[component]
pub fn Cancel(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let ctx = expect_context::<AlertDialogContext>();

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
