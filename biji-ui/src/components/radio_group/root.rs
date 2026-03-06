use std::sync::atomic::AtomicUsize;

use leptos::{context::Provider, ev::{click, focus, keydown}, prelude::*};
use leptos_use::use_event_listener;

use crate::items::{FilterActiveItems, Focus, ManageFocus, NavigateItems};

use super::context::{RadioGroupContext, RadioItemContext};

#[component]
pub fn Root(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] value: Option<String>,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] on_value_change: Option<Callback<String>>,
) -> impl IntoView {
    let ctx = RadioGroupContext {
        value: RwSignal::new(value),
        item_focus: RwSignal::new(None),
        items: RwSignal::new(Default::default()),
        disabled,
        next_id: StoredValue::new(AtomicUsize::new(0)),
        on_value_change,
    };

    view! {
        <Provider value={ctx}>
            <div
                role="radiogroup"
                aria-disabled={if ctx.disabled { Some("true") } else { None }}
                data-disabled={ctx.disabled}
                class={class}
            >
                {children()}
            </div>
        </Provider>
    }
}

#[component]
pub fn Item(
    children: Children,
    #[prop(into)] value: String,
    #[prop(into, optional)] class: String,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let group_ctx = expect_context::<RadioGroupContext>();

    let index = group_ctx.next_index();
    let item_ctx = RadioItemContext {
        index,
        value: StoredValue::new(value),
        disabled: disabled || group_ctx.disabled,
        trigger_ref: NodeRef::new(),
    };

    group_ctx.upsert_item(index, item_ctx);

    on_cleanup(move || {
        group_ctx.remove_item(index);
    });

    let _ = use_event_listener(item_ctx.trigger_ref, click, move |_| {
        if item_ctx.disabled {
            return;
        }
        let val = item_ctx.value.with_value(|v| v.clone());
        group_ctx.select(val.clone());
        if let Some(cb) = group_ctx.on_value_change {
            cb.run(val);
        }
    });

    let _ = use_event_listener(item_ctx.trigger_ref, focus, move |_| {
        group_ctx.set_focus(Some(item_ctx.index));
    });

    let _ = use_event_listener(item_ctx.trigger_ref, keydown, move |evt| {
        if item_ctx.disabled {
            return;
        }
        match evt.key().as_str() {
            "ArrowDown" | "ArrowRight" => {
                evt.prevent_default();
                if let Some(next) = group_ctx.navigate_next_item() {
                    next.focus();
                    let val = next.value.with_value(|v| v.clone());
                    group_ctx.select(val.clone());
                    if let Some(cb) = group_ctx.on_value_change {
                        cb.run(val);
                    }
                }
            }
            "ArrowUp" | "ArrowLeft" => {
                evt.prevent_default();
                if let Some(prev) = group_ctx.navigate_previous_item() {
                    prev.focus();
                    let val = prev.value.with_value(|v| v.clone());
                    group_ctx.select(val.clone());
                    if let Some(cb) = group_ctx.on_value_change {
                        cb.run(val);
                    }
                }
            }
            _ => {}
        }
    });

    let is_checked = Memo::new(move |_| item_ctx.is_checked(group_ctx.value.get()));

    view! {
        <Provider value={item_ctx}>
            <button
                node_ref={item_ctx.trigger_ref}
                type="button"
                role="radio"
                aria-checked={move || if is_checked.get() { "true" } else { "false" }}
                aria-disabled={if item_ctx.disabled { Some("true") } else { None }}
                data-state={move || if is_checked.get() { "checked" } else { "unchecked" }}
                data-disabled={item_ctx.disabled}
                tabindex={move || {
                    if item_ctx.disabled {
                        "-1"
                    } else if is_checked.get() {
                        "0"
                    } else if group_ctx.value.get().is_none()
                        && group_ctx
                            .filter_active_items()
                            .into_iter()
                            .next()
                            .map(|i| i.index)
                            == Some(item_ctx.index)
                    {
                        "0"
                    } else {
                        "-1"
                    }
                }}
                class={class}
            >
                {children()}
            </button>
        </Provider>
    }
}

#[component]
pub fn Indicator(#[prop(into, optional)] class: String) -> impl IntoView {
    let group_ctx = expect_context::<RadioGroupContext>();
    let item_ctx = expect_context::<RadioItemContext>();

    view! {
        <span
            aria-hidden="true"
            data-state={move || item_ctx.data_state(group_ctx.value.get())}
            data-disabled={item_ctx.disabled}
            class={class}
        />
    }
}
