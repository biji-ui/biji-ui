use leptos::{context::Provider, prelude::*};

use crate::components::menubar::context::{MenubarContext, RootContext};

#[component]
pub fn Root(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(default = false)] allow_menu_loop: bool,
    #[prop(default = false)] allow_item_loop: bool,
    #[prop(default = false)] prevent_scroll: bool,
) -> impl IntoView {
    let menubar_ref = NodeRef::new();

    let root_ctx = RootContext {
        allow_item_loop,
        allow_menu_loop,
        prevent_scroll,
        ..RootContext::default()
    };
    let ctx = MenubarContext {
        menubar_ref,
        root: RwSignal::new(root_ctx),
    };

    view! {
        <Provider value={ctx}>
            <div node_ref={menubar_ref} class={class}>
                <Provider value={root_ctx}>{children()}</Provider>
            </div>
        </Provider>
    }
}
