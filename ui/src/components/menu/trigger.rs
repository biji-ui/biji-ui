use leptos::*;

use crate::components::menu::contexts::MenuContext;

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
    let _trigger_ref = ctx.trigger_ref;

    view! {
        <button node_ref={_trigger_ref} class={class}>
            {children()}
        </button>
    }
}
