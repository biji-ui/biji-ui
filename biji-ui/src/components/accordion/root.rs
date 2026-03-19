use leptos::{context::Provider, prelude::*};

use super::context::{AccordionState, RootContext};

pub fn use_accordion() -> AccordionState {
    expect_context::<AccordionState>()
}

#[component]
pub fn RootWith<IV: IntoView + 'static>(
    children: impl Fn(AccordionState) -> IV + Send + Sync + 'static,
    #[prop(into, optional)] class: String,
    #[prop(default = false)] allow_loop: bool,
) -> impl IntoView {
    let accordion_ref = NodeRef::new();
    let root_ctx = RootContext {
        allow_loop,
        ..RootContext::default()
    };
    let state = AccordionState {
        root: RwSignal::new(root_ctx),
        accordion_ref,
    };

    view! {
        <Provider value={state}>
            <div node_ref={accordion_ref} class={class}>
                <Provider value={root_ctx}>{children(state)}</Provider>
            </div>
        </Provider>
    }
}

#[component]
pub fn Root(
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
    #[prop(default = false)] allow_loop: bool,
) -> impl IntoView {
    view! {
        <RootWith allow_loop={allow_loop} class={class} let:_>
            {children()}
        </RootWith>
    }
}
