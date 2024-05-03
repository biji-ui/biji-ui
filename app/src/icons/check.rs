use leptos::*;

#[component]
pub fn Check(#[prop(into, optional)] class: String) -> impl IntoView {
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
            <path d="M20 6 9 17l-5-5"></path>
        </svg>
    }
}
