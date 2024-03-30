use leptos::{html::Div, *};

use crate::components::accordion::events::RootEvents;

use super::item::AccordionItemContext;

#[derive(Clone)]
pub struct AccordionContext {
    pub items: RwSignal<Vec<AccordionItemContext>>,
    pub accordion_ref: NodeRef<Div>,
    pub current_focus: RwSignal<Option<usize>>,
}

#[component]
pub fn Root(children: ChildrenFn, #[prop(into, optional)] class: String) -> impl IntoView {
    let accordion_ref = create_node_ref::<Div>();

    let ctx = AccordionContext {
        items: create_rw_signal(Vec::new()),
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
