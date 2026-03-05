use leptos::{context::Provider, ev::click, prelude::*};
use leptos_use::use_event_listener;

use super::context::{CheckboxContext, CheckedState};

#[component]
pub fn Root(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] indeterminate: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] on_checked_change: Option<Callback<bool>>,
) -> impl IntoView {
    let initial = if indeterminate {
        CheckedState::Indeterminate
    } else if checked {
        CheckedState::Checked
    } else {
        CheckedState::Unchecked
    };

    let ctx = CheckboxContext {
        checked: RwSignal::new(initial),
        disabled,
        trigger_ref: NodeRef::new(),
    };

    let _ = use_event_listener(ctx.trigger_ref, click, move |_| {
        if ctx.disabled {
            return;
        }
        ctx.checked.update(|state| {
            *state = match *state {
                CheckedState::Checked => CheckedState::Unchecked,
                CheckedState::Unchecked | CheckedState::Indeterminate => CheckedState::Checked,
            };
        });
        if let Some(cb) = on_checked_change {
            cb.run(ctx.checked.get() == CheckedState::Checked);
        }
    });

    view! {
        <Provider value={ctx}>
            <button
                node_ref={ctx.trigger_ref}
                role="checkbox"
                aria-checked={move || ctx.checked.get().aria_checked()}
                aria-disabled={if ctx.disabled { Some("true") } else { None }}
                data-state={move || ctx.checked.get().as_str()}
                data-disabled={ctx.disabled}
                class={class}
            >
                {children()}
            </button>
        </Provider>
    }
}

#[component]
pub fn Indicator(
    children: Children,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let ctx = expect_context::<CheckboxContext>();

    view! {
        <span
            aria-hidden="true"
            data-state={move || ctx.checked.get().as_str()}
            data-disabled={ctx.disabled}
            class={class}
        >
            {children()}
        </span>
    }
}
