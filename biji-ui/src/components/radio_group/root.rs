use leptos::{context::Provider, ev::{click, focus, keydown}, prelude::*};
use leptos_use::use_event_listener;

use crate::items::{FilterActiveItems, Focus, ManageFocus, NavigateItems};

use super::context::{RadioGroupState, RadioItemContext};

/// Returns the [`RadioGroupState`] from the nearest [`Root`] or [`RootWith`] ancestor.
///
/// Call this inside any child component that needs access to radio group state.
pub fn use_radio_group() -> RadioGroupState {
    expect_context::<RadioGroupState>()
}

/// The render-prop variant of [`Root`]. Use this when you need access to [`RadioGroupState`]
/// directly inside the children via the `let:` binding.
///
/// ```rust
/// <radio_group::RootWith let:rg>
///     <p class="text-sm">{move || rg.value.get().unwrap_or_default()}</p>
///     <radio_group::Item value="a">...</radio_group::Item>
/// </radio_group::RootWith>
/// ```
#[component]
pub fn RootWith<IV: IntoView + 'static>(
    children: impl Fn(RadioGroupState) -> IV + Send + Sync + 'static,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] value: Option<String>,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let state = RadioGroupState::new(value, disabled);

    view! {
        <Provider value={state}>
            <div
                role="radiogroup"
                aria-disabled={if state.disabled { Some("true") } else { None }}
                data-disabled={state.disabled}
                class={class}
            >
                {children(state)}
            </div>
        </Provider>
    }
}

/// The standard radio group root. Renders a `<div role="radiogroup">` and provides
/// [`RadioGroupState`] to all descendants via context.
///
/// Use [`RootWith`] instead when you need to access [`RadioGroupState`] inline via `let:rg`.
#[component]
pub fn Root(
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] value: Option<String>,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let state = RadioGroupState::new(value, disabled);

    view! {
        <Provider value={state}>
            <div
                role="radiogroup"
                aria-disabled={if state.disabled { Some("true") } else { None }}
                data-disabled={state.disabled}
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
    let group = expect_context::<RadioGroupState>();

    let index = group.next_index();
    let item_ctx = RadioItemContext {
        index,
        value: StoredValue::new(value),
        disabled: disabled || group.disabled,
        trigger_ref: NodeRef::new(),
    };

    group.upsert_item(index, item_ctx);

    on_cleanup(move || {
        group.remove_item(index);
    });

    let _ = use_event_listener(item_ctx.trigger_ref, click, move |_| {
        if item_ctx.disabled {
            return;
        }
        let val = item_ctx.value.with_value(|v| v.clone());
        group.select(val);
    });

    let _ = use_event_listener(item_ctx.trigger_ref, focus, move |_| {
        group.set_focus(Some(item_ctx.index));
    });

    let _ = use_event_listener(item_ctx.trigger_ref, keydown, move |evt| {
        if item_ctx.disabled {
            return;
        }
        match evt.key().as_str() {
            "ArrowDown" | "ArrowRight" => {
                evt.prevent_default();
                if let Some(next) = group.navigate_next_item() {
                    next.focus();
                    let val = next.value.with_value(|v| v.clone());
                    group.select(val);
                }
            }
            "ArrowUp" | "ArrowLeft" => {
                evt.prevent_default();
                if let Some(prev) = group.navigate_previous_item() {
                    prev.focus();
                    let val = prev.value.with_value(|v| v.clone());
                    group.select(val);
                }
            }
            _ => {}
        }
    });

    let is_checked = Memo::new(move |_| item_ctx.is_checked(group.value.get()));

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
                    } else if group.value.get().is_none()
                        && group
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
    let group = expect_context::<RadioGroupState>();
    let item_ctx = expect_context::<RadioItemContext>();

    view! {
        <span
            aria-hidden="true"
            data-state={move || item_ctx.data_state(group.value.get())}
            data-disabled={item_ctx.disabled}
            class={class}
        />
    }
}
