use std::time::Duration;

use leptos::{context::Provider, prelude::*};

use crate::components::dialog::context::{DialogState, RootContext};
use crate::utils::prevent_scroll::use_prevent_scroll;

fn build_state(prevent_scroll: bool, hide_delay: Duration) -> DialogState {
    let open = RwSignal::new(false);
    DialogState {
        open,
        data_state: Signal::derive(move || if open.get() { "open" } else { "closed" }),
        trigger_ref: NodeRef::new(),
        root: RwSignal::new(RootContext::default()),
        prevent_scroll,
        hide_delay,
    }
}

/// Returns the [`DialogState`] from the nearest [`Root`] or [`RootWith`] ancestor.
///
/// Call this inside any child component that needs access to dialog state.
pub fn use_dialog() -> DialogState {
    expect_context::<DialogState>()
}

/// The render-prop variant of [`Root`]. Use this when you need access to [`DialogState`]
/// directly inside the children via the `let:` binding.
///
/// ```rust
/// <dialog::RootWith let:d>
///     <p>{move || if d.open.get() { "Open" } else { "Closed" }}</p>
///     <dialog::Trigger>"Open"</dialog::Trigger>
///     <dialog::Overlay />
///     <dialog::Content>
///         <dialog::Close>"Close"</dialog::Close>
///     </dialog::Content>
/// </dialog::RootWith>
/// ```
#[component]
pub fn RootWith<IV: IntoView + 'static>(
    children: impl Fn(DialogState) -> IV + Send + Sync + 'static,
    #[prop(into, optional)] class: String,
    /// Prevent scrolling when the dialog is open
    #[prop(default = true)]
    prevent_scroll: bool,
    /// The timeout after which the component will be unmounted if `when == false`
    #[prop(default = Duration::from_millis(200))]
    hide_delay: Duration,
) -> impl IntoView {
    let root_ctx = RootContext::default();
    let state = build_state(prevent_scroll, hide_delay);

    view! {
        <Provider value={state}>
            <Provider value={root_ctx}>
                <RootEvents>
                    <div class={class}>
                        {children(state)}
                    </div>
                </RootEvents>
            </Provider>
        </Provider>
    }
}

/// The standard dialog root. Use [`RootWith`] instead when you need to access
/// [`DialogState`] inline via `let:d`.
#[component]
pub fn Root(
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
    /// Prevent scrolling when the dialog is open
    #[prop(default = true)]
    prevent_scroll: bool,
    /// The timeout after which the component will be unmounted if `when == false`
    #[prop(default = Duration::from_millis(200))]
    hide_delay: Duration,
) -> impl IntoView {
    view! {
        <RootWith prevent_scroll hide_delay class let:_>
            {children()}
        </RootWith>
    }
}

#[component]
pub fn RootEvents(children: Children) -> impl IntoView {
    let dialog_state = expect_context::<DialogState>();

    let eff = use_prevent_scroll(
        move || dialog_state.prevent_scroll && dialog_state.open.get(),
        dialog_state.hide_delay,
    );

    on_cleanup(move || {
        drop(eff);
    });

    children()
}
