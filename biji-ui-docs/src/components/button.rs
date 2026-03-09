use leptos::prelude::*;

/// Button visual variants for use in doc examples.
///
/// Use [`btn`] to get the class string for a given variant, useful when passing
/// a class to a biji_ui trigger component (e.g. `dialog::Trigger class={btn(Variant::Default)}`).
///
/// Use the [`Button`] component directly for standalone `<button>` elements.
#[derive(Copy, Clone, Default, PartialEq)]
pub enum Variant {
    /// Filled with the primary theme colour.
    #[default]
    Default,
    /// Filled red — for destructive / irreversible actions.
    Destructive,
    /// Bordered, transparent background.
    Outline,
    /// No background; only shows on hover.
    Ghost,
}

const DEFAULT_CLS: &str = "inline-flex items-center justify-center rounded-md px-4 py-2 text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90";
const DESTRUCTIVE_CLS: &str = "inline-flex items-center justify-center rounded-md px-4 py-2 text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-red-600 text-white hover:bg-red-500 dark:bg-red-700 dark:hover:bg-red-600";
const OUTLINE_CLS: &str = "inline-flex items-center justify-center rounded-md px-4 py-2 text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-border bg-background text-foreground hover:bg-accent hover:text-accent-foreground";
const GHOST_CLS: &str = "inline-flex items-center justify-center rounded-md px-4 py-2 text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 text-foreground hover:bg-accent hover:text-accent-foreground";

/// Returns the full Tailwind class string for the given button variant.
///
/// This is a `const fn`, so it can be used in `const` definitions:
/// ```rust
/// const MY_BTN: &str = btn(Variant::Default);
/// ```
///
/// Intended for use with biji_ui trigger components that accept a `class` prop,
/// e.g. `<dialog::Trigger class={btn(Variant::Default)}>`.
pub const fn btn(variant: Variant) -> &'static str {
    match variant {
        Variant::Default => DEFAULT_CLS,
        Variant::Destructive => DESTRUCTIVE_CLS,
        Variant::Outline => OUTLINE_CLS,
        Variant::Ghost => GHOST_CLS,
    }
}

/// A standalone styled button for use in doc page examples.
///
/// For biji_ui trigger components, use [`btn`] directly to get the class string.
#[component]
pub fn Button(
    children: Children,
    #[prop(default = Variant::Default)] variant: Variant,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let base_cls = btn(variant);
    let full_class = if class.is_empty() {
        base_cls.to_string()
    } else {
        format!("{base_cls} {class}")
    };
    view! {
        <button type="button" class={full_class}>
            {children()}
        </button>
    }
}
