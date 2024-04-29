use leptos::{html::Div, *};

use super::context::{AccordionContext, RootContext};

#[component]
pub fn Root(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let accordion_ref = create_node_ref::<Div>();
    let root_ctx = RootContext::default();
    let ctx = AccordionContext {
        root: create_rw_signal(root_ctx),
        accordion_ref,
    };

    view! {
        <Provider value={ctx}>
            <div _ref={accordion_ref} class={class}>
                <Provider value={root_ctx}>{children()}</Provider>
            </div>
        </Provider>
    }
}
