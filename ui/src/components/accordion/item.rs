use std::time::Duration;

use leptos::{
    ev::{click, focus, keydown},
    *,
};
use leptos_use::use_event_listener;

use crate::{
    cn,
    custom_animated_show::CustomAnimatedShow,
    items::{Focus, ManageFocus, NavigateItems, Toggle},
};

use super::context::{ItemContext, RootContext};

#[component]
pub fn Item(
    #[prop(default = false)] disabled: bool,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let root_ctx = expect_context::<RootContext>();

    let index = root_ctx.next_index();

    let item_ctx = ItemContext {
        index,
        disabled,
        ..Default::default()
    };

    root_ctx.upsert_item(index, item_ctx);

    on_cleanup(move || {
        root_ctx.remove_item(index);
    });

    view! {
        <Provider value={item_ctx}>
            <div
                class={class}
                data-index={item_ctx.index}
                data-state={move || item_ctx.data_state()}
                data-disabled={item_ctx.disabled}
                data-highlighted={move || root_ctx.item_in_focus(item_ctx.index)}
            >
                {children()}
            </div>
        </Provider>
    }
}

#[component]
pub fn ItemTrigger(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let root_ctx = expect_context::<RootContext>();
    let item_ctx = expect_context::<ItemContext>();

    let trigger_ref = item_ctx.trigger_ref;
    view! {
        <ItemTriggerEvents>
            <button
                node_ref={trigger_ref}
                class={class}
                data-index={item_ctx.index}
                data-state={move || item_ctx.data_state()}
                data-disabled={item_ctx.disabled}
                data-highlighted={move || root_ctx.item_in_focus(item_ctx.index)}
            >
                {children()}
            </button>
        </ItemTriggerEvents>
    }
}

#[component]
pub fn ItemTriggerEvents(children: Children) -> impl IntoView {
    let root_ctx = expect_context::<RootContext>();
    let item_ctx = expect_context::<ItemContext>();

    let _ = use_event_listener(item_ctx.trigger_ref, click, move |_| {
        item_ctx.toggle();
    });

    let _ = use_event_listener(item_ctx.trigger_ref, keydown, move |evt| {
        let key = evt.key();
        match key.as_str() {
            "ArrowDown" => {
                if let Some(item) = root_ctx.navigate_next_item() {
                    evt.prevent_default();
                    item.focus();
                }
            }
            "ArrowUp" => {
                if let Some(item) = root_ctx.navigate_previous_item() {
                    evt.prevent_default();
                    item.focus();
                }
            }
            _ => {}
        };
    });

    let _ = use_event_listener(item_ctx.trigger_ref, focus, move |_| {
        root_ctx.set_focus(Some(item_ctx.index));
    });

    children()
}

#[component]
pub fn ItemContent(
    children: ChildrenFn,
    /// Optional CSS class to apply to both show and hide classes
    #[prop(into, optional)]
    class: String,
    /// Optional CSS class to apply if `when == true`
    #[prop(into, optional)]
    show_class: String,
    /// Optional CSS class to apply if `when == false`
    #[prop(into, optional)]
    hide_class: String,
    /// The timeout after which the component will be unmounted if `when == false`
    #[prop(default = Duration::from_millis(200))]
    hide_delay: Duration,
) -> impl IntoView {
    let ctx = expect_context::<ItemContext>();

    let children = store_value(children);
    view! {
        <CustomAnimatedShow
            when={ctx.open}
            show_class={cn!(class, show_class)}
            hide_class={cn!(class, hide_class)}
            hide_delay={hide_delay}
        >
            {children()}
        </CustomAnimatedShow>
    }
}
