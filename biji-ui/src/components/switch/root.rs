use leptos::{context::Provider, ev::click, prelude::*};
use leptos_use::use_event_listener;

use super::context::SwitchState;

/// Returns the [`SwitchState`] from the nearest [`Root`] or [`RootWith`] ancestor.
///
/// Call this inside any child component that needs access to switch state.
pub fn use_switch() -> SwitchState {
    expect_context::<SwitchState>()
}

/// The render-prop variant of [`Root`]. Use this when you need access to [`SwitchState`]
/// directly inside the children via the `let:` binding.
///
/// ```rust
/// <switch::RootWith checked=false let:s>
///     <span class="text-sm">{move || if s.checked.get() { "On" } else { "Off" }}</span>
///     <switch::Thumb ... />
/// </switch::RootWith>
/// ```
///
/// The `s: SwitchState` binding is `Copy`, so it can be passed to child components as a prop.
#[component]
pub fn RootWith<IV: IntoView + 'static>(
    children: impl Fn(SwitchState) -> IV + Send + Sync + 'static,
    #[prop(into, optional)] class: String,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let state = SwitchState::new(checked, disabled);

    let _ = use_event_listener(state.trigger_ref, click, move |_| {
        if state.disabled {
            return;
        }
        state.checked.update(|c| *c = !*c);
    });

    view! {
        <Provider value={state}>
            <button
                node_ref={state.trigger_ref}
                type="button"
                role="switch"
                aria-checked={move || if state.checked.get() { "true" } else { "false" }}
                aria-disabled={if state.disabled { Some("true") } else { None }}
                data-state={move || state.data_state.get()}
                data-disabled={state.disabled}
                class={class}
            >
                {children(state)}
            </button>
        </Provider>
    }
}

/// The standard switch root. Renders a `<button>` with ARIA roles and data attributes, and
/// provides [`SwitchState`] to all descendants via context.
///
/// Use [`RootWith`] instead when you need to access [`SwitchState`] inline via `let:s`.
#[component]
pub fn Root(
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    view! {
        <RootWith checked=checked disabled=disabled class=class let:_>
            {children()}
        </RootWith>
    }
}

#[component]
pub fn Thumb(#[prop(into, optional)] class: String) -> impl IntoView {
    let state = expect_context::<SwitchState>();

    view! {
        <span
            aria-hidden="true"
            data-state={move || state.data_state.get()}
            data-disabled={state.disabled}
            class={class}
        />
    }
}
