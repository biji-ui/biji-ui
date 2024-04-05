use std::collections::HashMap;

use leptos::{html::Div, *};

use crate::components::accordion::{contexts::AccordionContext, events::RootEvents};

#[component]
pub fn Root(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let accordion_ref = create_node_ref::<Div>();

    let ctx = AccordionContext {
        items: create_rw_signal(HashMap::new()),
        accordion_ref,
        current_focus: create_rw_signal(None),
    };

    view! {
        <Provider value={ctx.clone()}>
            <RootEvents>
                <div _ref={accordion_ref} class={class}>
                    {children()}
                </div>
            </RootEvents>
        </Provider>
    }
}
