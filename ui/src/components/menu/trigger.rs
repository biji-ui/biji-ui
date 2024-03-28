use leptos::*;

use super::root::MenuContext;

#[component]
pub fn TriggerFn<F, IV>(render: F) -> impl IntoView
where
    F: Fn(MenuContext) -> IV,
    IV: IntoView,
{
    let ctx = expect_context::<MenuContext>();
    render(ctx)
}

#[component]
pub fn Trigger(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let ctx = expect_context::<MenuContext>();

    view! {
        <button
            class={class}
            on:click={move |_| {
                ctx.open.set(!ctx.open.get());
            }}
        >

            {children()}
        </button>
    }
}
