use leptos::*;

#[component]
pub fn BookText(#[prop(into, optional)] class: String) -> impl IntoView {
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
            <path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20"></path>
            <path d="M8 7h6"></path>
            <path d="M8 11h8"></path>
        </svg>
    }
}
