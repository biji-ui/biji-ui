use jspackages::highlight::{highlight, highlight_element};
use leptos::{html::Code as HtmlCode, *};

#[component]
pub fn Code(
    #[prop(into, optional)] class: &'static str,
    code: &'static str,
    language: &'static str,
) -> impl IntoView {
    let (highlighted, set_highlighted) = create_signal(String::new());

    let code_ref = create_node_ref::<HtmlCode>();

    create_effect(move |_| {
        set_highlighted.set(highlight(code, language));
    });

    create_effect(move |_| {
        if let Some(code_ref) = code_ref.get() {
            highlight_element(&code_ref);
        }
    });

    view! {
        <pre>
            <code class={class} _ref={code_ref} inner_html={highlighted}></code>
        </pre>
    }
}
