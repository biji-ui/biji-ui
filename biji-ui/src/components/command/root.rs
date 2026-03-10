use leptos::{
    context::Provider,
    ev::{click, focus, keydown, mouseover},
    prelude::*,
};
use leptos_use::use_event_listener;

use crate::items::{Focus, ManageFocus, NavigateItems};

use super::context::{CommandContext, CommandGroupContext, CommandItemContext};

#[component]
pub fn Root(
    children: Children,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let ctx = CommandContext::default();

    view! {
        <Provider value={ctx}>
            <RootEvents>
                <div node_ref={ctx.root_ref} class={class}>
                    {children()}
                </div>
            </RootEvents>
        </Provider>
    }
}

#[component]
fn RootEvents(children: Children) -> impl IntoView {
    let ctx = expect_context::<CommandContext>();

    let _ = use_event_listener(ctx.root_ref, keydown, move |evt| {
        match evt.key().as_str() {
            "ArrowDown" => {
                evt.prevent_default();
                if let Some(item) = ctx.navigate_next_item() {
                    item.focus();
                    ctx.set_focus(Some(item.index));
                }
            }
            "ArrowUp" => {
                evt.prevent_default();
                if let Some(item) = ctx.navigate_previous_item() {
                    item.focus();
                    ctx.set_focus(Some(item.index));
                }
            }
            "Home" => {
                evt.prevent_default();
                if let Some(item) = ctx.navigate_first_item() {
                    item.focus();
                    ctx.set_focus(Some(item.index));
                }
            }
            "End" => {
                evt.prevent_default();
                if let Some(item) = ctx.navigate_last_item() {
                    item.focus();
                    ctx.set_focus(Some(item.index));
                }
            }
            _ => {}
        }
    });

    children()
}

#[component]
pub fn Input(
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] placeholder: String,
) -> impl IntoView {
    let ctx = expect_context::<CommandContext>();

    let _ = use_event_listener(ctx.input_ref, leptos::ev::input, move |evt| {
        let val = event_target_value(&evt);
        ctx.query.set(val);
        ctx.item_focus.set(None);
    });

    view! {
        <input
            node_ref={ctx.input_ref}
            type="text"
            role="combobox"
            aria-expanded="true"
            aria-autocomplete="list"
            aria-controls={ctx.list_id.get_value()}
            placeholder={placeholder}
            autocomplete="off"
            class={class}
        />
    }
}

#[component]
pub fn List(
    children: Children,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let ctx = expect_context::<CommandContext>();
    view! {
        <div
            id={ctx.list_id.get_value()}
            role="listbox"
            class={class}
        >
            {children()}
        </div>
    }
}

#[component]
pub fn Group(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] label: Option<String>,
    #[prop(into, optional)] label_class: String,
) -> impl IntoView {
    let group_ctx = CommandGroupContext {
        visible_count: RwSignal::new(0),
    };

    view! {
        <Provider value={group_ctx}>
            <div
                class={class}
                style={move || if group_ctx.visible_count.get() == 0 { "display:none;" } else { "" }}
                role="group"
            >
                {label.map(|lbl| view! {
                    <div class={label_class} aria-hidden="true">{lbl}</div>
                })}
                {children()}
            </div>
        </Provider>
    }
}

#[component]
pub fn Item(
    children: Children,
    #[prop(into)] value: String,
    #[prop(into, optional)] label: Option<String>,
    #[prop(into, optional)] class: String,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] on_select: Option<Callback<String>>,
) -> impl IntoView {
    let ctx = expect_context::<CommandContext>();
    let group_ctx = use_context::<CommandGroupContext>();

    let index = ctx.next_index();
    let label_text = label.unwrap_or_else(|| value.clone());
    let item_ctx = CommandItemContext {
        index,
        value: StoredValue::new(value),
        label: StoredValue::new(label_text),
        disabled,
        item_ref: NodeRef::new(),
    };

    ctx.upsert_item(index, item_ctx);
    on_cleanup(move || ctx.remove_item(index));

    let is_visible = Memo::new(move |_| ctx.is_item_visible(index));

    // Keep group counter in sync
    if let Some(gctx) = group_ctx {
        if is_visible.get_untracked() {
            gctx.visible_count.update(|c| *c += 1);
        }
        Effect::new(move |prev: Option<bool>| {
            let visible = is_visible.get();
            if let Some(was_visible) = prev {
                if visible && !was_visible {
                    gctx.visible_count.update(|c| *c += 1);
                } else if !visible && was_visible {
                    gctx.visible_count.update(|c| {
                        if *c > 0 {
                            *c -= 1;
                        }
                    });
                }
            }
            visible
        });
        on_cleanup(move || {
            if is_visible.get_untracked() {
                gctx.visible_count.update(|c| {
                    if *c > 0 {
                        *c -= 1;
                    }
                });
            }
        });
    }

    let _ = use_event_listener(item_ctx.item_ref, click, move |_| {
        if item_ctx.disabled || !is_visible.get_untracked() {
            return;
        }
        if let Some(cb) = on_select {
            let val = item_ctx.value.with_value(|v| v.clone());
            cb.run(val);
        }
    });

    let _ = use_event_listener(item_ctx.item_ref, keydown, move |evt| {
        if evt.key() == "Enter" {
            evt.prevent_default();
            if !item_ctx.disabled {
                if let Some(cb) = on_select {
                    let val = item_ctx.value.with_value(|v| v.clone());
                    cb.run(val);
                }
            }
        }
    });

    let _ = use_event_listener(item_ctx.item_ref, mouseover, move |_| {
        if !item_ctx.disabled && is_visible.get_untracked() {
            ctx.set_focus(Some(item_ctx.index));
        }
    });

    let _ = use_event_listener(item_ctx.item_ref, focus, move |_| {
        if !item_ctx.disabled {
            ctx.set_focus(Some(item_ctx.index));
        }
    });

    view! {
        <div
            node_ref={item_ctx.item_ref}
            role="option"
            tabindex="-1"
            aria-disabled={if item_ctx.disabled { Some("true") } else { None }}
            aria-selected={move || if ctx.item_in_focus(item_ctx.index) { "true" } else { "false" }}
            data-disabled={item_ctx.disabled}
            data-highlighted={move || ctx.item_in_focus(item_ctx.index)}
            style={move || if is_visible.get() { "" } else { "display:none;" }}
            class={class}
        >
            {children()}
        </div>
    }
}

#[component]
pub fn Empty(children: ChildrenFn) -> impl IntoView {
    let ctx = expect_context::<CommandContext>();
    let has_visible = Memo::new(move |_| !ctx.visible_items().is_empty());

    view! {
        <Show when={move || !has_visible.get()} fallback={|| ()}>
            {children()}
        </Show>
    }
}
