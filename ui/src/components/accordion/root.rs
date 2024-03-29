use leptos::*;

#[component]
pub fn Root(children: ChildrenFn, #[prop(into, optional)] class: String) -> impl IntoView {
    view! { <div class={class}>{children()}</div> }
}
