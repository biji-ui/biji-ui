//! # Biji UI
//!
//! Effortless headless UI components for your [Leptos](https://leptos.dev/) projects.
//!
//! Biji UI provides a collection of accessible, unstyled UI components that integrate
//! seamlessly with any CSS framework. Focus on building the core functionality of your
//! application while Biji UI handles the UI foundation.
//!
//! ## Components
//!
//! - **Accordion** – Collapsible content sections with keyboard navigation.
//! - **Dialog** – Modal dialogs with focus trapping and overlay support.
//! - **Menu** – Standalone dropdown menus with nested submenus.
//! - **Menubar** – Horizontal menu bars with keyboard-driven navigation.
//! - **Tooltip** – Hover/focus tooltips with configurable positioning.
//!
//! ## Quick Start
//!
//! ```rust
//! use leptos::prelude::*;
//! use biji_ui::components::accordion;
//!
//! #[component]
//! pub fn Example() -> impl IntoView {
//!     view! {
//!         <accordion::Root>
//!             <accordion::Item>
//!                 <accordion::Toggle>"Toggle"</accordion::Toggle>
//!                 <accordion::Content>"Content"</accordion::Content>
//!             </accordion::Item>
//!         </accordion::Root>
//!     }
//! }
//! ```

/// UI component modules (accordion, dialog, menu, menubar, tooltip).
pub mod components;

/// Custom animated show/hide wrapper component with CSS class transitions.
pub mod custom_animated_show;

/// Traits and helpers for item navigation, focus management, and toggling.
pub mod items;

/// Utility modules for positioning, scroll prevention, and polygon math.
pub mod utils;

/// A trait for converting a value into a CSS class string.
///
/// This trait is used by the [`cn!`] macro to normalize different string types
/// into a single `String` suitable for use as a CSS class list.
///
/// # Implementations
///
/// - `String` – returns itself.
/// - `&str` – converts to an owned `String`.
/// - `Option<String>` – returns the inner value or an empty string.
pub trait Style {
    /// Produce a CSS class string from this value.
    fn style(&self) -> String;
}

impl Style for String {
    fn style(&self) -> String {
        self.to_string()
    }
}

impl Style for Option<String> {
    fn style(&self) -> String {
        self.as_ref().map(|s| s.style()).unwrap_or_default()
    }
}

impl Style for &str {
    fn style(&self) -> String {
        self.to_string()
    }
}

/// Concatenate multiple CSS class expressions into a single space-separated string.
///
/// Each argument must implement the [`Style`] trait. Empty values are automatically
/// filtered out so you never end up with leading/trailing/double spaces.
///
/// # Examples
///
/// ```rust
/// use biji_ui::cn;
///
/// let base = "px-4 py-2";
/// let active = String::from("bg-blue-500");
/// let empty: Option<String> = None;
///
/// // Produces "px-4 py-2 bg-blue-500" (empty option is skipped)
/// let classes = cn!(base, active, empty);
/// ```
#[macro_export]
macro_rules! cn {
    ($($styles: expr),*) => {
        {
            use $crate::Style;
            let parts: Vec<String> = vec![$($styles.style()),*];
            parts.into_iter()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join(" ")
        }
    };
}
