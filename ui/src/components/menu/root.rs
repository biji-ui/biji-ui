use std::collections::HashMap;

use leptos::{
    html::{Button, Div},
    *,
};

use crate::components::menu::{contexts::MenuContext, events::RootEvents};

#[component]
pub fn Root(
    children: Children,
    #[prop(default = true)] close_on_outside_click: bool,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let menu_ref = create_node_ref::<Div>();

    let ctx = MenuContext {
        open: create_rw_signal(false),
        in_focus: create_rw_signal(false),
        close_on_outside_click,
        menu_ref,
        trigger_ref: create_node_ref::<Button>(),
        items: create_rw_signal(HashMap::new()),
        current_focus: create_rw_signal(None),
    };

    view! {
        <Provider value={ctx}>
            <RootEvents>
                <div _ref={menu_ref} class={class}>
                    {children()}
                </div>
            </RootEvents>
        </Provider>
    }
}
