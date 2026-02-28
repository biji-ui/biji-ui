use std::time::Duration;

use leptos::{
    attribute_interceptor::AttributeInterceptor,
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
    let show_handle: StoredValue<Option<TimeoutHandle>> = StoredValue::new(None);
    let hide_handle: StoredValue<Option<TimeoutHandle>> = StoredValue::new(None);
    let cls = RwSignal::new(if when.get_untracked() {
        show_class.clone()
    } else {
        hide_class.clone()
    });
    let show = RwSignal::new(when.get_untracked());

    let eff = RenderEffect::new(move |_| {
        let show_class = show_class.clone();
        if when.get() {
            // clear any possibly active timer
            if let Some(h) = show_handle.get_value() {
                h.clear();
            }
            if let Some(h) = hide_handle.get_value() {
                h.clear();
            }

            let h = leptos_dom::helpers::set_timeout_with_handle(
                move || cls.set(show_class.clone()),
                Duration::from_millis(1),
            )
            .expect("set timeout in AnimatedShow");
            show_handle.set_value(Some(h));

            cls.set(hide_class.clone());
            show.set(true);
        } else {
            cls.set(hide_class.clone());

            let h =
                leptos_dom::helpers::set_timeout_with_handle(move || show.set(false), hide_delay)
                    .expect("set timeout in AnimatedShow");
            hide_handle.set_value(Some(h));
        }
    });

    on_cleanup(move || {
        if let Some(Some(h)) = show_handle.try_get_value() {
            h.clear();
        }
        if let Some(Some(h)) = hide_handle.try_get_value() {
            h.clear();
        }
        drop(eff);
    });

    let stored_style = StoredValue::new(style);
    let children = StoredValue::new(children);

    // Build the computed style: prefer reactive signal, fall back to static string
    let computed_style = move || match style_signal {
        Some(sig) => sig.get(),
        None => stored_style.get_value(),
    };

    // Use the provided node_ref or create a default one
    let div_ref = node_ref.unwrap_or_default();

    view! {
        <Show when={move || show.get()} fallback={|| ()}>
            <AttributeInterceptor let:attrs>
                <div
                    {..attrs}
                    node_ref={div_ref}
                    class={move || cls.get()}
                    style={computed_style}
                >
                    {children.with_value(|c| c())}
                </div>
            </AttributeInterceptor>
        </Show>
    }
}
