use leptos::*;

#[component]
pub fn ChevronRight(#[prop(into, optional)] class: String) -> impl IntoView {
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
            <path d="m9 18 6-6-6-6"></path>
        </svg>
    }
}
