use leptos::{html::Div, *};

use crate::components::menubar::context::{MenubarContext, RootContext};

#[component]
pub fn Root(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(default = false)] allow_menu_loop: bool,
    #[prop(default = false)] allow_item_loop: bool,
) -> impl IntoView {
    let menubar_ref = create_node_ref::<Div>();

    let root_ctx = RootContext {
        allow_item_loop,
        allow_menu_loop,
        ..RootContext::default()
    };
    let ctx = MenubarContext {
        menubar_ref,
        root: create_rw_signal(root_ctx),
    };

    view! {
        <Provider value={ctx}>
            <div node_ref={menubar_ref} class={class}>
                <Provider value={root_ctx}>{children()}</Provider>
            </div>
        </Provider>
    }
}
