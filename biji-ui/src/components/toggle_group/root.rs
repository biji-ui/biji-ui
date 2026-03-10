use std::sync::atomic::AtomicUsize;

use leptos::{
    context::Provider,
    ev::{click, focus, keydown},
    prelude::*,
};
use leptos_use::use_event_listener;

use crate::items::{FilterActiveItems, Focus, ManageFocus, NavigateItems};

use super::context::{ToggleGroupContext, ToggleGroupType, ToggleItemContext};

#[component]
pub fn Root(
    children: Children,
    #[prop(into, optional)] class: String,
    /// Initial selected value (Single mode).
    #[prop(into, optional)] value: Option<String>,
    /// Initial selected values (Multiple mode).
    #[prop(optional)] values: Option<Vec<String>>,
    #[prop(default = ToggleGroupType::Single)] group_type: ToggleGroupType,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] on_value_change: Option<Callback<String>>,
    #[prop(optional)] on_values_change: Option<Callback<Vec<String>>>,
) -> impl IntoView {
    let initial = match group_type {
        ToggleGroupType::Single => value.map(|v| vec![v]).unwrap_or_default(),
        ToggleGroupType::Multiple => values.unwrap_or_default(),
    };

    let ctx = ToggleGroupContext {
        value: RwSignal::new(initial),
        group_type,
        disabled,
        item_focus: RwSignal::new(None),
        items: RwSignal::new(Default::default()),
        next_id: StoredValue::new(AtomicUsize::new(0)),
        on_value_change,
        on_values_change,
    };

    view! {
        <Provider value={ctx}>
            <div
                role="group"
                aria-disabled={if disabled { Some("true") } else { None }}
                data-disabled={disabled}
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
    let ctx = expect_context::<ToggleGroupContext>();

    let index = ctx.next_index();
    let item_ctx = ToggleItemContext {
        index,
        value: StoredValue::new(value),
        disabled: disabled || ctx.disabled,
        trigger_ref: NodeRef::new(),
    };

    ctx.upsert_item(index, item_ctx);

    on_cleanup(move || {
        ctx.remove_item(index);
    });

    let fire_callbacks = move |val: String| {
        if let Some(cb) = ctx.on_value_change {
            cb.run(val);
        }
        if let Some(cb) = ctx.on_values_change {
            let current = ctx.value.with(|v| v.clone());
            cb.run(current);
        }
    };

    let _ = use_event_listener(item_ctx.trigger_ref, click, move |_| {
        if item_ctx.disabled {
            return;
        }
        let val = item_ctx.value.with_value(|v| v.clone());
        ctx.toggle_value(val.clone());
        fire_callbacks(val);
    });

    let _ = use_event_listener(item_ctx.trigger_ref, focus, move |_| {
        ctx.set_focus(Some(item_ctx.index));
    });

    let _ = use_event_listener(item_ctx.trigger_ref, keydown, move |evt| {
        if item_ctx.disabled {
            return;
        }
        match evt.key().as_str() {
            "ArrowRight" | "ArrowDown" => {
                evt.prevent_default();
                if let Some(next) = ctx.navigate_next_item() {
                    next.focus();
                }
            }
            "ArrowLeft" | "ArrowUp" => {
                evt.prevent_default();
                if let Some(prev) = ctx.navigate_previous_item() {
                    prev.focus();
                }
            }
            "Home" => {
                evt.prevent_default();
                if let Some(first) = ctx.navigate_first_item() {
                    first.focus();
                }
            }
            "End" => {
                evt.prevent_default();
                if let Some(last) = ctx.navigate_last_item() {
                    last.focus();
                }
            }
            "Enter" | " " => {
                evt.prevent_default();
                let val = item_ctx.value.with_value(|v| v.clone());
                ctx.toggle_value(val.clone());
                fire_callbacks(val);
            }
            _ => {}
        }
    });

    let is_pressed = Memo::new(move |_| item_ctx.value.with_value(|v| ctx.is_pressed(v)));

    view! {
        <button
            node_ref={item_ctx.trigger_ref}
            type="button"
            role="switch"
            aria-pressed={move || if is_pressed.get() { "true" } else { "false" }}
            aria-disabled={if item_ctx.disabled { Some("true") } else { None }}
            data-state={move || if is_pressed.get() { "on" } else { "off" }}
            data-disabled={item_ctx.disabled}
            tabindex={move || {
                if item_ctx.disabled {
                    return "-1";
                }
                // Roving tabindex: the pressed item holds the tab stop.
                // If nothing is pressed, the first active item holds it.
                let any_pressed = ctx.value.with(|v| !v.is_empty());
                if is_pressed.get() {
                    "0"
                } else if !any_pressed
                    && ctx
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
    }
}
