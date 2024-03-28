use leptos::{html::Div, *};

#[derive(Copy, Clone)]
pub struct MenuContext {
    pub open: RwSignal<bool>,
    pub close_on_outside_click: bool,
    pub menu_ref: NodeRef<Div>,
}

#[component]
pub fn Root(
    children: Children,
    #[prop(default = true)] close_on_outside_click: bool,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let menu_ref = create_node_ref::<Div>();

    let ctx = MenuContext {
        open: create_rw_signal(false),
        close_on_outside_click,
        menu_ref,
    };

    view! {
        <Provider value={ctx}>
            <div class={class} _ref={menu_ref}>
                {children()}
            </div>
        </Provider>
    }
}
