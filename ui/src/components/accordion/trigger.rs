use leptos::*;

use crate::components::accordion::{
    events::ItemEvents, item::AccordionItemContext, root::AccordionContext,
};

#[component]
pub fn Trigger(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let root_ctx = expect_context::<AccordionContext>();
    let ctx = expect_context::<AccordionItemContext>();
    let trigger_ref = ctx.trigger_ref;

    view! {
        <ItemEvents>
            <button
                _ref={trigger_ref}
                class={class}
                data-state={move || if ctx.open.get() { "open" } else { "closed" }}
                data-value={ctx.index}
                data-highlighted={move || {
                    root_ctx.current_focus.get().map(|f| f == ctx.index).unwrap_or(false)
                }}
            >

                {children()}
            </button>
        </ItemEvents>
    }
}
