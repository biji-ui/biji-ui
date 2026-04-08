use leptos::{
    context::Provider,
    ev::{click, focus, keydown},
    prelude::*,
};
use leptos_use::use_event_listener;

use crate::items::{FilterActiveItems, Focus, ManageFocus, NavigateItems};

use super::context::{ToggleGroupState, ToggleGroupType, ToggleItemContext};

/// Returns the [`ToggleGroupState`] from the nearest [`Root`] or [`RootWith`] ancestor.
///
/// Call this inside any child component that needs access to toggle group state.
pub fn use_toggle_group() -> ToggleGroupState {
    expect_context::<ToggleGroupState>()
}

/// The render-prop variant of [`Root`]. Use this when you need access to [`ToggleGroupState`]
/// directly inside the children via the `let:` binding.
///
/// ```rust
/// <toggle_group::RootWith let:tg>
///     <p class="text-sm">{move || tg.value.with(|v| v.join(", "))}</p>
///     <toggle_group::Item value="bold">B</toggle_group::Item>
/// </toggle_group::RootWith>
/// ```
#[component]
pub fn RootWith<IV: IntoView + 'static>(
    children: impl Fn(ToggleGroupState) -> IV + Send + Sync + 'static,
    #[prop(into, optional)] class: String,
    /// Controlled signal. When provided, the toggle group reads and writes this signal directly.
    #[prop(into, default = None)]
    value: Option<RwSignal<Vec<String>>>,
    #[prop(into, default = None)] default_value: Option<String>,
    #[prop(into, default = None)] default_values: Option<Vec<String>>,
    #[prop(default = ToggleGroupType::Single)] group_type: ToggleGroupType,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let state = ToggleGroupState::new(value, default_value, default_values, group_type, disabled);

    view! {
        <Provider value={state}>
            <div
                role="group"
                aria-disabled={if state.disabled { Some("true") } else { None }}
                data-disabled={state.disabled}
                class={class}
            >
                {children(state)}
            </div>
        </Provider>
    }
}

/// The standard toggle group root. Renders a `<div role="group">` and provides
/// [`ToggleGroupState`] to all descendants via context.
///
/// Use [`RootWith`] instead when you need to access [`ToggleGroupState`] inline via `let:tg`.
#[component]
pub fn Root(
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
    #[prop(into, default = None)] value: Option<RwSignal<Vec<String>>>,
    #[prop(into, default = None)] default_value: Option<String>,
    #[prop(into, default = None)] default_values: Option<Vec<String>>,
    #[prop(default = ToggleGroupType::Single)] group_type: ToggleGroupType,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    view! {
        <RootWith value=value default_value=default_value default_values=default_values group_type=group_type disabled=disabled class=class let:_>
            {children()}
        </RootWith>
    }
}

#[component]
pub fn Item(
    children: Children,
    #[prop(into)] value: String,
    #[prop(into, optional)] class: String,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let ctx = expect_context::<ToggleGroupState>();

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

    let _ = use_event_listener(item_ctx.trigger_ref, click, move |_| {
        if item_ctx.disabled {
            return;
        }
        let val = item_ctx.value.with_value(|v| v.clone());
        ctx.toggle_value(val);
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
                ctx.toggle_value(val);
            }
            _ => {}
        }
    });

    let is_pressed = Memo::new(move |_| item_ctx.value.with_value(|v| ctx.is_pressed(v)));

    let is_tab_stop = Memo::new(move |_| {
        if item_ctx.disabled {
            return false;
        }
        let first_pressed_idx = ctx.items.with(|m| {
            ctx.value.with(|vals| {
                m.values()
                    .filter(|i| !i.disabled && i.value.with_value(|v| vals.contains(v)))
                    .map(|i| i.index)
                    .min()
            })
        });
        if let Some(fp) = first_pressed_idx {
            return fp == item_ctx.index;
        }
        ctx.filter_active_items()
            .into_iter()
            .next()
            .map(|i| i.index)
            == Some(item_ctx.index)
    });

    view! {
        <button
            node_ref={item_ctx.trigger_ref}
            type="button"
            aria-pressed={move || if is_pressed.get() { "true" } else { "false" }}
            aria-disabled={if item_ctx.disabled { Some("true") } else { None }}
            data-state={move || if is_pressed.get() { "on" } else { "off" }}
            data-disabled={item_ctx.disabled}
            tabindex={move || if is_tab_stop.get() { "0" } else { "-1" }}
            class={class}
        >
            {children()}
        </button>
    }
}
