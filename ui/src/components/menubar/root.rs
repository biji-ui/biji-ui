use leptos::{html::Div, *};

use crate::components::menubar::context::{MenubarContext, RootContext};

#[component]
pub fn Root(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let menubar_ref = create_node_ref::<Div>();

    let root_ctx = RootContext::default();
    let ctx = MenubarContext {
        menubar_ref,
        root: create_rw_signal(root_ctx),
    };

    view! {
        <Provider value={ctx}>
            <div _ref={menubar_ref} class={class}>
                <Provider value={root_ctx}>{children()}</Provider>
            </div>
        </Provider>
    }
}
