use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

impl Orientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Orientation::Horizontal => "horizontal",
            Orientation::Vertical => "vertical",
        }
    }
}

#[component]
pub fn Root(
    #[prop(into, optional)] class: String,
    #[prop(default = Orientation::Horizontal)] orientation: Orientation,
    #[prop(default = true)] decorative: bool,
) -> impl IntoView {
    let aria_role = if decorative { "none" } else { "separator" };
    let aria_orientation = if decorative {
        None
    } else {
        Some(orientation.as_str())
    };

    view! {
        <div
            role={aria_role}
            aria-orientation={aria_orientation}
            data-orientation={orientation.as_str()}
            class={class}
        />
    }
}
