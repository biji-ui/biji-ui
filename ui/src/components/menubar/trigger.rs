use leptos::*;

use crate::components::menubar::contexts::{MenubarContext, MenubarMenuContext};

#[component]
pub fn Trigger(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let menubar_ctx = expect_context::<MenubarContext>();
    let ctx = expect_context::<MenubarMenuContext>();
    let _trigger_ref = ctx.trigger_ref;
    view! {
        <button
            _ref={_trigger_ref}
            class={class}
            data-state={ctx.index}
            data-disabled={ctx.disabled}
            data-highlighted={move || {
                menubar_ctx.current_focus.get().map(|f| f == ctx.index).unwrap_or(false)
            }}
        >

            {children()}
        </button>
    }
}
