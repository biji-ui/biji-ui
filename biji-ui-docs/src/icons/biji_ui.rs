use leptos::prelude::*;

#[component]
pub fn BijiUI(#[prop(into, optional)] class: String) -> impl IntoView {
    view! {
        <span class={format!("inline-flex items-center gap-2 {class}")}>
            <img src="/logo.svg" alt="" aria-hidden="true" class="w-auto h-full" />
            <img src="/text-logo.svg" alt="Biji UI" class="w-auto h-full" />
        </span>
    }
}
