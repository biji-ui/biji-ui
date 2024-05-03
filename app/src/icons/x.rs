use leptos::*;

#[component]
pub fn X(#[prop(into, optional)] class: String) -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class={class}
        >
            <path d="M18 6 6 18"></path>
            <path d="m6 6 12 12"></path>
        </svg>
    }
}
