use std::time::Duration;

use leptos::{
    context::Provider,
    ev::{blur, click, focus, keydown},
    prelude::*,
};
use leptos_use::use_event_listener;
use wasm_bindgen::JsCast;
use web_sys::{HtmlAnchorElement, HtmlButtonElement};

use crate::{
    cn,
    components::menubar::context::ItemData,
    custom_animated_show::CustomAnimatedShow,
    items::{Focus, GetIndex, ManageFocus, NavigateItems, Toggle},
};

use super::context::{MenuContext, RootContext};

#[component]
pub fn Item(
    #[prop(default = false)] disabled: bool,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let menu_ctx = expect_context::<MenuContext>();

    let item_ctx = use_context::<ItemData>();

    let trigger_ref = NodeRef::new();

    let index = menu_ctx.next_index();

    let item_ctx = ItemData::Item {
        index,
        disabled,
        trigger_ref,
        is_submenu: item_ctx.is_some(),
    };

    menu_ctx.upsert_item(index, item_ctx);

    on_cleanup(move || {
        menu_ctx.remove_item(index);
    });

    view! {
        <Provider value={item_ctx}>
            <ItemTriggerEvents>
                <div
                    node_ref={trigger_ref}
                    class={class}
                    tabindex=0
                    data-state={item_ctx.get_index()}
                    data-disabled={item_ctx.get_disabled()}
                    data-highlighted={move || menu_ctx.item_in_focus(item_ctx.get_index())}
                >
                    {children()}
                </div>
            </ItemTriggerEvents>
        </Provider>
    }
}

#[component]
pub fn ItemTriggerEvents(children: Children) -> impl IntoView {
    let root_ctx = expect_context::<RootContext>();
    let menu_ctx = expect_context::<MenuContext>();
    let item_ctx = expect_context::<ItemData>();

    let (key, set_key) = signal::<Option<String>>(None);

    let _ = use_event_listener(item_ctx.get_trigger_ref(), keydown, move |evt| {
        let key = evt.key();

        set_key(Some(key.clone()));

        match key.as_str() {
            "ArrowDown" => {
                evt.prevent_default();
                if let Some(item) = menu_ctx.navigate_next_item() {
                    item.focus();
                }
            }
            "ArrowUp" => {
                evt.prevent_default();
                if let Some(item) = menu_ctx.navigate_previous_item() {
                    item.focus();
                }
            }
            "ArrowRight" => {
                evt.prevent_default();
                match item_ctx {
                    ItemData::Item { .. } => {
                        if let Some(item) = root_ctx.navigate_next_item() {
                            root_ctx.close_all();
                            item.focus();
                            item.open();
                        }
                    }
                    ItemData::SubMenuItem { child_context, .. } => {
                        if !child_context.open.get_untracked() {
                            child_context.open();
                        } else {
                            if let Some(item) = child_context.navigate_first_item() {
                                item.focus();
                            } else {
                                menu_ctx.close();
                            }
                        }
                    }
                };
            }
            "ArrowLeft" => {
                evt.prevent_default();
                if item_ctx.is_submenu() {
                    menu_ctx.close();
                    menu_ctx.focus();
                    menu_ctx.item_focus.set(None);
                } else {
                    if let Some(item) = root_ctx.navigate_previous_item() {
                        item.focus();
                        item.open();
                        menu_ctx.close();
                    }
                }
            }
            "Enter" => {
                if let Some(trigger_ref) = item_ctx.get_trigger_ref().get() {
                    if let Some(child) = trigger_ref.children().get_with_index(0) {
                        if let Ok(child) = child.clone().dyn_into::<HtmlButtonElement>() {
                            let _ = child.click();
                        } else if let Ok(child) = child.dyn_into::<HtmlAnchorElement>() {
                            let _ = child.click();
                        }
                    }
                    match item_ctx {
                        ItemData::Item { .. } => {
                            root_ctx.close_all();
                            root_ctx.focus_active_item();
                        }
                        _ => {}
                    };
                }
            }
            "Escape" => {
                menu_ctx.close();
                menu_ctx.focus();
            }
            _ => {}
        };
    });

    match item_ctx {
        ItemData::Item { trigger_ref, .. } => {
            let _ = use_event_listener(trigger_ref, click, move |_| {
                root_ctx.close_all();
                root_ctx.focus_active_item();
            });
        }
        _ => {}
    }

    let _ = use_event_listener(item_ctx.get_trigger_ref(), focus, move |_| {
        menu_ctx.set_focus(Some(item_ctx.get_index()));
        match item_ctx {
            ItemData::SubMenuItem { child_context, .. } => {
                if !child_context.open.get_untracked() {
                    child_context.open();
                }
            }
            _ => {}
        }
    });

    let _ = use_event_listener(item_ctx.get_trigger_ref(), blur, move |_| {
        menu_ctx.set_focus(Some(item_ctx.get_index()));
        match item_ctx {
            ItemData::SubMenuItem { child_context, .. } => {
                if child_context.open.get_untracked() {
                    if let Some(key) = key.get() {
                        if key == "ArrowDown" || key == "ArrowUp" || key == "ArrowLeft" {
                            child_context.close();
                        }
                    }
                }
            }
            _ => {}
        }
    });

    children()
}

#[component]
pub fn SubMenuItem(
    #[prop(default = false)] disabled: bool,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let menu_ctx = expect_context::<MenuContext>();

    let item_ctx = use_context::<ItemData>();

    let index = menu_ctx.next_index();

    let sub_menu_ctx = MenuContext {
        index,
        disabled,
        allow_loop: menu_ctx.allow_loop,
        ..Default::default()
    };

    let item_ctx = ItemData::SubMenuItem {
        index,
        disabled,
        is_submenu: item_ctx.is_some(),
        parent_context: menu_ctx,
        child_context: sub_menu_ctx,
    };

    menu_ctx.upsert_item(index, item_ctx);

    on_cleanup(move || {
        menu_ctx.remove_item(index);
    });

    view! {
        <Provider value={item_ctx}>
            <div class={class}>
                <ItemTriggerEvents>
                    <Provider value={sub_menu_ctx}>{children()}</Provider>
                </ItemTriggerEvents>
            </div>
        </Provider>
    }
}

#[component]
pub fn SubMenuItemTrigger(
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let item_ctx = expect_context::<MenuContext>();
    let sub_menu_ctx = expect_context::<ItemData>();

    let trigger_ref = item_ctx.trigger_ref;

    view! {
        <SubMenuItemTriggerEvents>
            <div
                node_ref={trigger_ref}
                class={class}
                tabindex=0
                data-state={item_ctx.index}
                data-disabled={item_ctx.disabled}
                data-highlighted={move || match sub_menu_ctx {
                    ItemData::SubMenuItem { parent_context, .. } => {
                        parent_context.item_in_focus(item_ctx.index)
                    }
                    _ => false,
                }}
            >

                {children()}
            </div>
        </SubMenuItemTriggerEvents>
    }
}

#[component]
pub fn SubMenuItemTriggerEvents(children: Children) -> impl IntoView {
    let menu_ctx = expect_context::<MenuContext>();
    let item_ctx = expect_context::<ItemData>();

    let _ = use_event_listener(menu_ctx.trigger_ref, click, move |_| {
        menu_ctx.toggle();
    });

    let _ = use_event_listener(menu_ctx.trigger_ref, keydown, move |evt| {
        if evt.key() == "Enter" {
            menu_ctx.toggle();
            match item_ctx {
                ItemData::SubMenuItem { child_context, .. } => {
                    if let Some(item) = child_context.navigate_first_item() {
                        item.focus();
                    }
                }
                _ => {}
            };
        }
    });

    children()
}

#[component]
pub fn SubMenuItemContent(
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
    hide_delay: Duration,
) -> impl IntoView {
    let item_ctx = expect_context::<MenuContext>();

    view! {
        <CustomAnimatedShow
            when={item_ctx.open}
            show_class={cn!(class, show_class)}
            hide_class={cn!(class, hide_class)}
            hide_delay={hide_delay}
        >
            {children()}
        </CustomAnimatedShow>
    }
}
