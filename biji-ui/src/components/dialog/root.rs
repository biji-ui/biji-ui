use std::time::Duration;

use leptos::{context::Provider, prelude::*};

use crate::components::dialog::context::{DialogContext, RootContext};
use crate::utils::prevent_scroll::use_prevent_scroll;

#[component]
pub fn Root(
    children: Children,
    #[prop(into, optional)] class: String,
    /// Prevent scrolling when the dialog is open
    #[prop(default = true)]
    prevent_scroll: bool,
    /// The timeout after which the component will be unmounted if `when == false`
    #[prop(default = Duration::from_millis(200))]
    hide_delay: Duration,
) -> impl IntoView {
    let root_ctx = RootContext {
        ..RootContext::default()
    };
    let ctx = DialogContext {
        root: RwSignal::new(root_ctx),
        prevent_scroll,
        hide_delay,
        ..Default::default()
    };

    view! {
        <Provider value={ctx}>
            <RootEvents>
                <div class={class}>
                    <Provider value={root_ctx}>{children()}</Provider>
                </div>
            </RootEvents>
        </Provider>
    }
}

#[component]
pub fn RootEvents(children: Children) -> impl IntoView {
    let dialog_ctx = expect_context::<DialogContext>();

    let eff = use_prevent_scroll(
        move || dialog_ctx.prevent_scroll && dialog_ctx.open.get(),
        dialog_ctx.hide_delay,
    );

    on_cleanup(move || {
        drop(eff);
    });

    children()
}
