use std::collections::HashMap;

use leptos::{html::Div, *};

use crate::components::menubar::{contexts::MenubarContext, events::RootEvents};

#[component]
pub fn Root(
    children: Children,
    #[prop(default = true)] close_on_outside_click: bool,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let menubar_ref = create_node_ref::<Div>();

    let ctx = MenubarContext {
        in_focus: create_rw_signal(false),
        items: create_rw_signal(HashMap::new()),
        menubar_ref,
        current_focus: create_rw_signal(None),
        close_on_outside_click,
    };

    view! {
        <Provider value={ctx}>
            <RootEvents>
                <div _ref={menubar_ref} class={class}>
                    {children()}
                </div>
            </RootEvents>
        </Provider>
    }
}
