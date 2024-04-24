use leptos::*;

use crate::components::dialog::context::{DialogContext, RootContext};

#[component]
pub fn Root(
    children: Children,
    #[prop(into, optional)] class: String,
    /// Prevent scrolling when the dialog is open
    #[prop(default = true)]
    prevent_scroll: bool,
) -> impl IntoView {
    let root_ctx = RootContext::default();
    let ctx = DialogContext {
        root: create_rw_signal(root_ctx),
        prevent_scroll,
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

    create_effect(move |_| {
        if dialog_ctx.prevent_scroll {
            if dialog_ctx.open.get() {
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
                if let Some(doc) = document().body() {
                    let _ = doc.style().remove_property("overflow");
                    let _ = doc.style().remove_property("--scrollbar-width");
                    let _ = doc.style().remove_property("padding-right");
                }
            }
        }
    });

    children()
}
