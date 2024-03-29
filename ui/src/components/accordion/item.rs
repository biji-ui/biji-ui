use leptos::*;

#[derive(Clone)]
pub struct AccordionItemContext {
    pub value: String,
    pub open: RwSignal<bool>,
}

#[component]
pub fn Item(
    #[prop(into)] value: String,
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let ctx = AccordionItemContext {
        value,
        open: create_rw_signal(false),
    };

    view! {
        <Provider value={ctx.clone()}>
            <div class={class}>{children()}</div>
        </Provider>
    }
}
