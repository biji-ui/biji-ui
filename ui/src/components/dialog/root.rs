use std::time::Duration;

use leptos::{
    context::Provider,
    leptos_dom::{self, helpers::TimeoutHandle},
    prelude::*,
};

use crate::components::dialog::context::{DialogContext, RootContext};

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
    let hide_handle: StoredValue<Option<TimeoutHandle>> = StoredValue::new(None);
    let dialog_ctx = expect_context::<DialogContext>();

    let _ = RenderEffect::new(move |_| {
        if dialog_ctx.prevent_scroll {
            if dialog_ctx.open.get() {
                if let Some(h) = hide_handle.get_value() {
                    h.clear();
                }
                if let Some(doc) = document().body() {
                    let client_width = f64::from(doc.client_width());
                    let inner_width = window()
                        .inner_width()
                        .unwrap()
                        .as_f64()
                        .unwrap_or(client_width);
                    let scrollbar_width = inner_width - client_width;

                    let _ = doc.style().set_property("overflow", "hidden");
                    let _ = doc
                        .style()
                        .set_property("--scrollbar-width", &format!("{}px", scrollbar_width));
                    let _ = doc
                        .style()
                        .set_property("padding-right", &format!("calc({}px)", scrollbar_width));
                }
            } else {
                let h = leptos_dom::helpers::set_timeout_with_handle(
                    move || {
                        if let Some(doc) = document().body() {
                            let _ = doc.style().remove_property("overflow");
                            let _ = doc.style().remove_property("--scrollbar-width");
                            let _ = doc.style().remove_property("padding-right");
                        }
                    },
                    dialog_ctx.hide_delay,
                )
                .expect("set timeout in AnimatedShow");
                hide_handle.set_value(Some(h));
            }
        }
    });

    children()
}
