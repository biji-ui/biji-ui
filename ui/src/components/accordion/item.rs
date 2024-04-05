use leptos::{html::Button, *};

use crate::components::accordion::contexts::{AccordionContext, AccordionItemContext};

#[component]
pub fn Item(
    index: usize,
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let main_ctx = expect_context::<AccordionContext>();

    let trigger_ref = create_node_ref::<Button>();

    let ctx = AccordionItemContext {
        index,
        disabled,
        open: create_rw_signal(false),
        trigger_ref,
    };

    main_ctx.upsert_item(index, ctx);

    view! {
        <Provider value={ctx}>
            <div class={class}>{children()}</div>
        </Provider>
    }
}
