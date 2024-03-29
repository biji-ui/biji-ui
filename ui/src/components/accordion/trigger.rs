use leptos::*;

use crate::components::accordion::item::AccordionItemContext;

#[component]
pub fn Trigger(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let ctx = expect_context::<AccordionItemContext>();

    view! {
        <button
            class={class}
            data-state={move || if ctx.open.get() { "open" } else { "closed" }}
            on:click={move |_| {
                ctx.open.set(!ctx.open.get());
            }}
        >

            {children()}
        </button>
    }
}
