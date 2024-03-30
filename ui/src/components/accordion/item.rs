use leptos::{html::Button, *};

use crate::components::accordion::root::AccordionContext;

#[derive(Clone)]
pub struct AccordionItemContext {
    pub index: usize,
    pub open: RwSignal<bool>,
    pub trigger_ref: NodeRef<Button>,
}

#[component]
pub fn Item(
    index: usize,
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let main_ctx = expect_context::<AccordionContext>();

    let trigger_ref = create_node_ref::<Button>();

    let ctx = AccordionItemContext {
        index,
        open: create_rw_signal(false),
        trigger_ref,
    };

    main_ctx.items.update(|items| {
        items.push(ctx.clone());
    });

    view! {
        <Provider value={ctx}>
            <div class={class}>{children()}</div>
        </Provider>
    }
}
