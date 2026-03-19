use leptos::{context::Provider, ev::click, prelude::*};
use leptos_use::use_event_listener;

use super::context::{CheckboxState, CheckedState};

/// Returns the [`CheckboxState`] from the nearest [`Root`] or [`RootWith`] ancestor.
///
/// Call this inside any child component that needs access to checkbox state.
pub fn use_checkbox() -> CheckboxState {
    expect_context::<CheckboxState>()
}

/// The render-prop variant of [`Root`]. Use this when you need access to [`CheckboxState`]
/// directly inside the children via the `let:` binding.
///
/// ```rust
/// <checkbox::RootWith checked=false let:cb>
///     <checkbox::Indicator>
///         <Icon name="check" />
///     </checkbox::Indicator>
///     <span class="sr-only">{move || cb.data_state.get()}</span>
/// </checkbox::RootWith>
/// ```
///
/// The `cb: CheckboxState` binding is `Copy`, so it can be passed to child components as a prop.
#[component]
pub fn RootWith<IV: IntoView + 'static>(
    children: impl Fn(CheckboxState) -> IV + Send + Sync + 'static,
    #[prop(into, optional)] class: String,
    /// Controlled signal. When provided, the checkbox reads and writes this signal directly.
    #[prop(into, default = None)]
    checked: Option<RwSignal<CheckedState>>,
    #[prop(default = false)] default_checked: bool,
    #[prop(default = false)] indeterminate: bool,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let state = CheckboxState::new(checked, default_checked, indeterminate, disabled);

    let _ = use_event_listener(state.trigger_ref, click, move |_| {
        if state.disabled {
            return;
        }
        state.checked.update(|s| {
            *s = match *s {
                CheckedState::Checked => CheckedState::Unchecked,
                CheckedState::Unchecked | CheckedState::Indeterminate => CheckedState::Checked,
            };
        });
    });

    view! {
        <Provider value={state}>
            <button
                node_ref={state.trigger_ref}
                type="button"
                role="checkbox"
                aria-checked={move || state.checked.get().aria_checked()}
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

/// The standard checkbox root. Renders a `<button role="checkbox">` with ARIA attributes
/// and provides [`CheckboxState`] to all descendants via context.
///
/// Use [`RootWith`] instead when you need to access [`CheckboxState`] inline via `let:cb`.
#[component]
pub fn Root(
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
    #[prop(into, default = None)] checked: Option<RwSignal<CheckedState>>,
    #[prop(default = false)] default_checked: bool,
    #[prop(default = false)] indeterminate: bool,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    view! {
        <RootWith checked=checked default_checked=default_checked indeterminate=indeterminate disabled=disabled class=class let:_>
            {children()}
        </RootWith>
    }
}

#[component]
pub fn Indicator(
    children: Children,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let state = expect_context::<CheckboxState>();

    view! {
        <span
            aria-hidden="true"
            data-state={move || state.data_state.get()}
            data-disabled={state.disabled}
            class={class}
        >
            {children()}
        </span>
    }
}
