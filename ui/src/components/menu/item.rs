use leptos::*;

use super::root::MenuContext;

#[component]
pub fn Item(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] href: Option<String>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let ctx = expect_context::<MenuContext>();
    if let Some(href) = href {
        return view! {
            <a {..attrs} href={href} class={class}>
                {children()}
            </a>
        }
        .into_view();
    }
    view! {
        <div
            {..attrs}
            class={class}
            on:click={move |_| {
                ctx.open.set(false);
            }}
        >

            {children()}
        </div>
    }
    .into_view()
}
