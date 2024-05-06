use jspackages::shiki::code_to_html;
use leptos::*;
use leptos_use::{use_color_mode, ColorMode, UseColorModeReturn};

#[component]
pub fn Code(
    #[prop(into, optional)] class: &'static str,
    code: &'static str,
    language: &'static str,
) -> impl IntoView {
    let UseColorModeReturn { mode, .. } = use_color_mode();

    let (highlighted, set_highlighted) = create_signal(String::new());

    create_effect(move |_| {
        let theme = match mode.get() {
            ColorMode::Dark => "vesper",
            ColorMode::Light => "solarized-light",
            _ => "vesper",
        };
        let code = code_to_html(code, language, theme);

        set_highlighted.set(code);
    });

    view! { <code inner_html={highlighted} class={class}></code> }
}
