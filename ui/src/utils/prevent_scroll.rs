use std::time::Duration;

use leptos::{
    leptos_dom::{self, helpers::TimeoutHandle},
    prelude::*,
};

/// A utility that prevents scrolling on the document body when a condition is true.
///
/// This is useful for modals, dialogs, and other overlay components that should
/// prevent background scrolling. It also compensates for scrollbar width to prevent
/// layout shift.
///
/// # Example
///
/// ```rust
/// let is_open = RwSignal::new(false);
///
/// let _effect = use_prevent_scroll(
///     move || is_open.get(),
///     Duration::from_millis(200)
/// );
/// ```
pub fn use_prevent_scroll<F>(should_prevent: F, hide_delay: Duration) -> RenderEffect<()>
where
    F: Fn() -> bool + 'static,
{
    let hide_handle: StoredValue<Option<TimeoutHandle>> = StoredValue::new(None);

    RenderEffect::new(move |_| {
        if should_prevent() {
            // Clear any pending hide timeout
            if let Some(h) = hide_handle.get_value() {
                h.clear();
            }

            // Apply prevent scroll styles
            if let Some(doc) = document().body() {
                let client_width = f64::from(doc.client_width());
                let inner_width = window()
                    .inner_width()
                    .unwrap()
                    .as_f64()
                    .unwrap_or(client_width);
                let scrollbar_width = inner_width - client_width;

                let _ = doc.style().set_property("overflow", "hidden");
                let _ = doc
                    .style()
                    .set_property("--scrollbar-width", &format!("{}px", scrollbar_width));
                let _ = doc
                    .style()
                    .set_property("padding-right", &format!("calc({}px)", scrollbar_width));
            }
        } else {
            // Schedule removal of prevent scroll styles
            let h = leptos_dom::helpers::set_timeout_with_handle(
                move || {
                    if let Some(doc) = document().body() {
                        let _ = doc.style().remove_property("overflow");
                        let _ = doc.style().remove_property("--scrollbar-width");
                        let _ = doc.style().remove_property("padding-right");
                    }
                },
                hide_delay,
            )
            .expect("set timeout in use_prevent_scroll");
            hide_handle.set_value(Some(h));
        }
    })
}
