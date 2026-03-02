use std::sync::{Arc, Mutex};
use std::time::Duration;

use leptos::{
    html::Div,
    leptos_dom::{self, helpers::TimeoutHandle},
    prelude::*,
};

#[component]
pub fn CustomAnimatedShow(
    /// The components Show wraps
    children: ChildrenFn,
    /// If the component should show or not
    #[prop(into)]
    when: Signal<bool>,
    /// Optional CSS class to apply if `when == true`
    #[prop(into, optional)]
    show_class: String,
    /// Optional CSS class to apply if `when == false`
    #[prop(into, optional)]
    hide_class: String,
    /// The timeout after which the component will be unmounted if `when == false`
    hide_delay: Duration,
    /// Optional CSS style to apply (static string)
    #[prop(into, optional)]
    style: String,
    /// Optional reactive CSS style signal (takes precedence over `style` when provided)
    #[prop(into, optional)]
    style_signal: Option<Signal<String>>,
    /// Optional node ref for the wrapper div
    #[prop(optional)]
    node_ref: Option<NodeRef<Div>>,
) -> impl IntoView {
    // Use Arc<Mutex<>> instead of StoredValue so that Leptos arena slot reuse
    // can never cause a stale effect from an old mount to corrupt a new mount's
    // timer handles.
    let show_handle: Arc<Mutex<Option<TimeoutHandle>>> = Arc::new(Mutex::new(None));
    let hide_handle: Arc<Mutex<Option<TimeoutHandle>>> = Arc::new(Mutex::new(None));

    let cls = RwSignal::new(if when.get_untracked() {
        show_class.clone()
    } else {
        hide_class.clone()
    });
    let show = RwSignal::new(when.get_untracked());

    let show_handle_eff = Arc::clone(&show_handle);
    let hide_handle_eff = Arc::clone(&hide_handle);

    let eff = RenderEffect::new(move |_| {
        let show_class = show_class.clone();

        if when.get() {
            // Cancel any in-flight timers from a previous transition.
            if let Some(h) = show_handle_eff.lock().unwrap().take() {
                h.clear();
            }
            if let Some(h) = hide_handle_eff.lock().unwrap().take() {
                h.clear();
            }

            // After 1 ms, swap from hide_class → show_class so CSS transitions
            // start from the hidden state.
            let sh = Arc::clone(&show_handle_eff);
            let h = leptos_dom::helpers::set_timeout_with_handle(
                move || {
                    cls.set(show_class.clone());
                    *sh.lock().unwrap() = None;
                },
                Duration::from_millis(1),
            )
            .expect("set timeout in AnimatedShow");
            *show_handle_eff.lock().unwrap() = Some(h);

            cls.set(hide_class.clone());
            show.set(true);
        } else {
            cls.set(hide_class.clone());

            // Cancel the pending show-class timer so a rapid true→false flip
            // doesn't cause a flash (the 1 ms timer would apply show_class even
            // though we're already hiding again).
            if let Some(h) = show_handle_eff.lock().unwrap().take() {
                h.clear();
            }

            // Cancel any previous hide timer before scheduling a new one,
            // otherwise duplicate timers from repeated same-value sets will pile up.
            if let Some(h) = hide_handle_eff.lock().unwrap().take() {
                h.clear();
            }

            // Unmount after hide_delay so the hide animation can play out.
            let hh = Arc::clone(&hide_handle_eff);
            let h = leptos_dom::helpers::set_timeout_with_handle(
                move || {
                    show.set(false);
                    *hh.lock().unwrap() = None;
                },
                hide_delay,
            )
            .expect("set timeout in AnimatedShow");
            *hide_handle_eff.lock().unwrap() = Some(h);
        }
    });

    on_cleanup(move || {
        if let Some(h) = show_handle.lock().unwrap().take() {
            h.clear();
        }
        if let Some(h) = hide_handle.lock().unwrap().take() {
            h.clear();
        }
        drop(eff);
    });

    let stored_style = StoredValue::new(style);
    // Build the computed style: prefer reactive signal, fall back to static string
    let computed_style = move || match style_signal {
        Some(sig) => sig.get(),
        None => stored_style.get_value(),
    };

    // Use the provided node_ref or create a default one
    let div_ref = node_ref.unwrap_or_default();

    view! {
        <Show when={move || show.get()} fallback={|| ()}>
            <div node_ref={div_ref} class={move || cls.get()} style={computed_style}>
                {children()}
            </div>
        </Show>
    }
}
