use leptos::{context::Provider, ev::click, prelude::*};
use leptos_use::use_event_listener;

use super::context::SwitchContext;

#[component]
pub fn Root(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] on_checked_change: Option<Callback<bool>>,
) -> impl IntoView {
    let ctx = SwitchContext {
        checked: RwSignal::new(checked),
        disabled,
        trigger_ref: NodeRef::new(),
    };

    let _ = use_event_listener(ctx.trigger_ref, click, move |_| {
        if ctx.disabled {
            return;
        }
        ctx.checked.update(|c| *c = !*c);
        if let Some(cb) = on_checked_change {
            cb.run(ctx.checked.get());
        }
    });

    view! {
        <Provider value={ctx}>
            <button
                node_ref={ctx.trigger_ref}
                role="switch"
                aria-checked={move || if ctx.checked.get() { "true" } else { "false" }}
                aria-disabled={if ctx.disabled { Some("true") } else { None }}
                data-state={move || ctx.data_state()}
                data-disabled={ctx.disabled}
                class={class}
            >
                {children()}
            </button>
        </Provider>
    }
}

#[component]
pub fn Thumb(#[prop(into, optional)] class: String) -> impl IntoView {
    let ctx = expect_context::<SwitchContext>();

    view! {
        <span
            aria-hidden="true"
            data-state={move || ctx.data_state()}
            data-disabled={ctx.disabled}
            class={class}
        />
    }
}
