use std::time::Duration;

use leptos::{
    ev::{click, focus, keydown},
    html::Div,
    *,
};
use leptos_use::use_event_listener;
use wasm_bindgen::JsCast;
use web_sys::{HtmlAnchorElement, HtmlButtonElement};

use crate::{cn, components::menubar::context::ItemData, custom_animated_show::CustomAnimatedShow};

use super::context::{
    Focus, FocusActiveItem, GetIndex, ManageFocus, MenuContext, NavigateActiveItems, RootContext,
    Toggle,
};

#[component]
pub fn Item(
    #[prop(default = false)] disabled: bool,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let menu_ctx = expect_context::<MenuContext>();

    let item_ctx = use_context::<ItemData>();

    let trigger_ref = create_node_ref::<Div>();

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
                    _ref={trigger_ref}
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

    let _ = use_event_listener(item_ctx.get_trigger_ref(), keydown, move |evt| {
        let key = evt.key();

        if key == "ArrowDown" {
            if let Some(item) = menu_ctx.next_active_item() {
                item.focus();
            }
        } else if key == "ArrowUp" {
            if let Some(item) = menu_ctx.previous_active_item() {
                item.focus();
            }
        } else if key == "ArrowRight" {
            match item_ctx {
                ItemData::Item { .. } => {
                    if let Some(item) = root_ctx.next_active_item() {
                        item.focus();
                        item.open();
                    }
                }
                ItemData::SubMenuItem { child_context, .. } => {
                    child_context.open();
                    if let Some(item) = child_context.first_active() {
                        item.focus();
                    }
                }
            };
        } else if key == "ArrowLeft" {
            if item_ctx.is_submenu() {
                menu_ctx.close();
                menu_ctx.focus();
            } else {
                if let Some(item) = root_ctx.previous_active_item() {
                    item.focus();
                    item.open();
                    menu_ctx.close();
                }
            }
        } else if key == "Enter" {
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
        } else if key == "Escape" {
            menu_ctx.close();
            menu_ctx.focus();
        }
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

    let _trigger_ref = item_ctx.trigger_ref;

    view! {
        <SubMenuItemTriggerEvents>
            <div
                _ref={_trigger_ref}
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
                    if let Some(item) = child_context.first_active() {
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

    let children = store_value(children);
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
