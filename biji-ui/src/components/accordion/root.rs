use leptos::{context::Provider, prelude::*};

use super::context::{AccordionState, RootContext};

fn build_state(allow_loop: bool) -> AccordionState {
    let root_ctx = RootContext {
        allow_loop,
        ..RootContext::default()
    };
    AccordionState {
        root: RwSignal::new(root_ctx),
        accordion_ref: NodeRef::new(),
    }
}

pub fn use_accordion() -> AccordionState {
    expect_context::<AccordionState>()
}

#[component]
pub fn RootWith<IV: IntoView + 'static>(
    children: impl Fn(AccordionState) -> IV + Send + Sync + 'static,
    #[prop(into, optional)] class: String,
    #[prop(default = false)] allow_loop: bool,
) -> impl IntoView {
    let state = build_state(allow_loop);

    view! {
        <Provider value={state}>
            <div node_ref={state.accordion_ref} class={class}>
                {children(state)}
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
