use leptos::{ev::click, prelude::*};
use leptos_use::use_event_listener;
use wasm_bindgen::JsCast;

use crate::{
    cn, components::dialog::context::DialogState, custom_animated_show::CustomAnimatedShow,
};

use super::context::RootContext;

#[component]
pub fn Trigger(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let dialog_ctx = expect_context::<DialogState>();

    let trigger_ref = dialog_ctx.trigger_ref;

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
    let dialog_ctx = expect_context::<DialogState>();

    let _ = use_event_listener(dialog_ctx.trigger_ref, click, move |_| {
        dialog_ctx.toggle();
        if let Some(trigger_ref) = dialog_ctx.trigger_ref.get() {
            let _ = trigger_ref.blur();
        }
    });

    children()
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
    let dialog_ctx = expect_context::<DialogState>();
    let root_ctx = expect_context::<RootContext>();

    let content_ref = root_ctx.content_ref;

    // Focus the first focusable element when dialog opens
    let _focus_eff = RenderEffect::new(move |_| {
        if dialog_ctx.open.get() {
            // Use a small delay to let the DOM render
            let _ = leptos::leptos_dom::helpers::set_timeout_with_handle(
                move || {
                    if let Some(el) = content_ref.get() {
                        focus_first_element(&el);
                    }
                },
                std::time::Duration::from_millis(10),
            );
        }
    });

    // Handle Tab / Shift+Tab focus trapping and Escape to close
    let _ = use_event_listener(content_ref, leptos::ev::keydown, move |evt| {
        let key = evt.key();
        match key.as_str() {
            "Escape" => {
                evt.prevent_default();
                dialog_ctx.close();
                // Return focus to the trigger
                if let Some(trigger) = dialog_ctx.trigger_ref.get() {
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
            when={dialog_ctx.open}
            show_class={cn!(class, show_class)}
            hide_class={cn!(class, hide_class)}
            hide_delay={dialog_ctx.hide_delay}
            node_ref={content_ref}
            attr:role="dialog"
            attr:aria-modal="true"
        >
            {children()}
        </CustomAnimatedShow>
    }
}

/// Query all focusable elements inside the given container.
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

/// Focus the first focusable element inside the container, or the container itself.
fn focus_first_element(container: &web_sys::HtmlElement) {
    let focusable = get_focusable_elements(container);
    if let Some(first) = focusable.first() {
        let _ = first.focus();
    } else {
        // If no focusable element found, focus the container itself
        let _ = container.focus();
    }
}

/// Trap Tab / Shift+Tab focus within the container.
fn trap_tab_focus(container: &web_sys::HtmlElement, shift_key: bool, evt: &web_sys::KeyboardEvent) {
    let focusable = get_focusable_elements(container);
    if focusable.is_empty() {
        evt.prevent_default();
        return;
    }

    let active = document().active_element();

    if shift_key {
        // Shift+Tab: if focus is on the first element, wrap to the last
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
        // Tab: if focus is on the last element, wrap to the first
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

#[component]
pub fn Overlay(
    #[prop(into, optional)] class: String,
    /// Optional CSS class to apply if `when == true`
    #[prop(into, optional)]
    show_class: String,
    /// Optional CSS class to apply if `when == false`
    #[prop(into, optional)]
    hide_class: String,
) -> impl IntoView {
    let dialog_ctx = expect_context::<DialogState>();
    let root_ctx = expect_context::<RootContext>();

    let overlay_ref = root_ctx.overlay_ref;

    view! {
        <CustomAnimatedShow
            when={dialog_ctx.open}
            show_class={cn!(class, show_class)}
            hide_class={cn!(class, hide_class)}
            hide_delay={dialog_ctx.hide_delay}
        >
            <OverlayEvents>
                <div node_ref={overlay_ref} style="inset: 0; width: 100%; height: 100%"></div>
            </OverlayEvents>
        </CustomAnimatedShow>
    }
}

#[component]
pub fn OverlayEvents(children: Children) -> impl IntoView {
    let dialog_ctx = expect_context::<DialogState>();
    let root_ctx = expect_context::<RootContext>();

    let _ = use_event_listener(root_ctx.overlay_ref, click, move |_| {
        dialog_ctx.close();
    });

    children()
}

#[component]
pub fn Close(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let root_ctx = expect_context::<RootContext>();

    let close_ref = root_ctx.close_ref;

    view! {
        <CloseEvents>
            <button node_ref={close_ref} class={class}>
                {children()}
            </button>
        </CloseEvents>
    }
}

#[component]
pub fn CloseEvents(children: Children) -> impl IntoView {
    let dialog_ctx = expect_context::<DialogState>();
    let root_ctx = expect_context::<RootContext>();

    let _ = use_event_listener(root_ctx.close_ref, click, move |_| {
        dialog_ctx.close();
    });

    children()
}
