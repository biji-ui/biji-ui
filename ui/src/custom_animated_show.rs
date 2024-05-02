use std::time::Duration;

use leptos::{leptos_dom::helpers::TimeoutHandle, *};

#[component]
pub fn CustomAnimatedShow(
    /// The components Show wraps
    children: ChildrenFn,
    /// If the component should show or not
    #[prop(into)]
    when: MaybeSignal<bool>,
    /// Optional CSS class to apply if `when == true`
    #[prop(into, optional)]
    show_class: String,
    /// Optional CSS class to apply if `when == false`
    #[prop(into, optional)]
    hide_class: String,
    /// The timeout after which the component will be unmounted if `when == false`
    hide_delay: Duration,
) -> impl IntoView {
    let show_handle: StoredValue<Option<TimeoutHandle>> = store_value(None);
    let hide_handle: StoredValue<Option<TimeoutHandle>> = store_value(None);
    let cls = create_rw_signal(if when.get_untracked() {
        show_class.clone()
    } else {
        hide_class.clone()
    });
    let show = create_rw_signal(when.get_untracked());

    create_render_effect(move |_| {
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
    });

    view! {
        <Show when={move || show.get()} fallback={|| ()}>
            <div class={move || cls.get()}>{children()}</div>
        </Show>
    }
}
