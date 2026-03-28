use leptos::{context::Provider, prelude::*};

use crate::components::menubar::context::{MenubarContext, RootContext};

fn build_state(allow_menu_loop: bool, allow_item_loop: bool, prevent_scroll: bool) -> MenubarContext {
    let root_ctx = RootContext {
        allow_item_loop,
        allow_menu_loop,
        prevent_scroll,
        ..RootContext::default()
    };
    MenubarContext {
        menubar_ref: NodeRef::new(),
        root: RwSignal::new(root_ctx),
    }
}

#[component]
pub fn Root(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(default = false)] allow_menu_loop: bool,
    #[prop(default = false)] allow_item_loop: bool,
    #[prop(default = false)] prevent_scroll: bool,
) -> impl IntoView {
    let ctx = build_state(allow_menu_loop, allow_item_loop, prevent_scroll);

    view! {
        <Provider value={ctx}>
            <div node_ref={ctx.menubar_ref} class={class} role="menubar">
                {children()}
            </div>
        </Provider>
    }
}
