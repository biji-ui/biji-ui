use std::sync::atomic::{AtomicUsize, Ordering};

use leptos::{
    context::Provider,
    ev::{click, focus, keydown},
    prelude::*,
};
use leptos_use::use_event_listener;

use crate::items::{FilterActiveItems, Focus, ManageFocus, NavigateItems};

use super::context::{ActivationMode, Orientation, TabItemContext, TabsContext};

static TABS_ROOT_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn next_root_id() -> usize {
    TABS_ROOT_COUNTER.fetch_add(1, Ordering::Relaxed)
}

fn tab_ids(root_id: usize, index: usize) -> (String, String) {
    (
        format!("biji-tab-trigger-{root_id}-{index}"),
        format!("biji-tab-panel-{root_id}-{index}"),
    )
}

#[component]
pub fn Root(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] value: Option<String>,
    #[prop(default = Orientation::Horizontal)] orientation: Orientation,
    #[prop(default = ActivationMode::Automatic)] activation_mode: ActivationMode,
    #[prop(optional)] on_value_change: Option<Callback<String>>,
) -> impl IntoView {
    let ctx = TabsContext {
        value: RwSignal::new(value),
        item_focus: RwSignal::new(None),
        items: RwSignal::new(Default::default()),
        orientation,
        activation_mode,
        on_value_change,
        next_id: StoredValue::new(AtomicUsize::new(0)),
        root_id: next_root_id(),
    };

    view! {
        <Provider value={ctx}>
            <div
                class={class}
                data-orientation={if orientation == Orientation::Horizontal {
                    "horizontal"
                } else {
                    "vertical"
                }}
            >
                {children()}
            </div>
        </Provider>
    }
}

#[component]
pub fn List(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let ctx = expect_context::<TabsContext>();
    view! {
        <div
            role="tablist"
            aria-orientation={if ctx.orientation == Orientation::Horizontal {
                "horizontal"
            } else {
                "vertical"
            }}
            class={class}
        >
            {children()}
        </div>
    }
}

#[component]
pub fn Trigger(
    children: Children,
    #[prop(into)] value: String,
    #[prop(into, optional)] class: String,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let ctx = expect_context::<TabsContext>();

    let index = ctx.next_index();
    let (trigger_id, panel_id) = tab_ids(ctx.root_id, index);

    let item_ctx = TabItemContext {
        index,
        value: StoredValue::new(value),
        disabled,
        trigger_ref: NodeRef::new(),
        trigger_id: StoredValue::new(trigger_id),
        panel_id: StoredValue::new(panel_id),
    };

    ctx.upsert_item(index, item_ctx);

    on_cleanup(move || {
        ctx.remove_item(index);
    });

    let _ = use_event_listener(item_ctx.trigger_ref, click, move |_| {
        if item_ctx.disabled {
            return;
        }
        let val = item_ctx.value.with_value(|v| v.clone());
        ctx.select(val);
    });

    let _ = use_event_listener(item_ctx.trigger_ref, focus, move |_| {
        ctx.set_focus(Some(item_ctx.index));
    });

    let _ = use_event_listener(item_ctx.trigger_ref, keydown, move |evt| {
        if item_ctx.disabled {
            return;
        }
        let is_horizontal = ctx.orientation == Orientation::Horizontal;
        let (forward_key, backward_key) = if is_horizontal {
            ("ArrowRight", "ArrowLeft")
        } else {
            ("ArrowDown", "ArrowUp")
        };

        match evt.key().as_str() {
            k if k == forward_key => {
                evt.prevent_default();
                if let Some(next) = ctx.navigate_next_item() {
                    next.focus();
                    if ctx.activation_mode == ActivationMode::Automatic {
                        let val = next.value.with_value(|v| v.clone());
                        ctx.select(val);
                    }
                }
            }
            k if k == backward_key => {
                evt.prevent_default();
                if let Some(prev) = ctx.navigate_previous_item() {
                    prev.focus();
                    if ctx.activation_mode == ActivationMode::Automatic {
                        let val = prev.value.with_value(|v| v.clone());
                        ctx.select(val);
                    }
                }
            }
            "Home" => {
                evt.prevent_default();
                if let Some(first) = ctx.navigate_first_item() {
                    first.focus();
                    if ctx.activation_mode == ActivationMode::Automatic {
                        let val = first.value.with_value(|v| v.clone());
                        ctx.select(val);
                    }
                }
            }
            "End" => {
                evt.prevent_default();
                if let Some(last) = ctx.navigate_last_item() {
                    last.focus();
                    if ctx.activation_mode == ActivationMode::Automatic {
                        let val = last.value.with_value(|v| v.clone());
                        ctx.select(val);
                    }
                }
            }
            "Enter" | " " => {
                evt.prevent_default();
                let val = item_ctx.value.with_value(|v| v.clone());
                ctx.select(val);
            }
            _ => {}
        }
    });

    let is_selected =
        Memo::new(move |_| ctx.value.get().is_some_and(|v| item_ctx.value.with_value(|iv| v == *iv)));

    view! {
        <button
            node_ref={item_ctx.trigger_ref}
            type="button"
            role="tab"
            id={item_ctx.trigger_id.get_value()}
            aria-selected={move || if is_selected.get() { "true" } else { "false" }}
            aria-controls={item_ctx.panel_id.get_value()}
            aria-disabled={if item_ctx.disabled { Some("true") } else { None }}
            data-state={move || if is_selected.get() { "active" } else { "inactive" }}
            data-disabled={item_ctx.disabled}
            data-orientation={if ctx.orientation == Orientation::Horizontal {
                "horizontal"
            } else {
                "vertical"
            }}
            tabindex={move || {
                if item_ctx.disabled {
                    "-1"
                } else if is_selected.get() {
                    "0"
                } else if ctx.value.get().is_none()
                    && ctx
                        .filter_active_items()
                        .into_iter()
                        .next()
                        .map(|i| i.index)
                        == Some(item_ctx.index)
                {
                    // If nothing is selected, first active tab gets tabindex=0.
                    "0"
                } else {
                    "-1"
                }
            }}
            class={class}
        >
            {children()}
        </button>
    }
}

#[component]
pub fn Content(
    children: Children,
    #[prop(into)] value: String,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let ctx = expect_context::<TabsContext>();
    let tab_value = StoredValue::new(value);

    // Single scan: returns both IDs or None if no matching Trigger is registered yet.
    // Attributes are omitted (not set to "") when None.
    let ids: Memo<Option<(String, String)>> = Memo::new(move |_| {
        ctx.items.with(|m| {
            m.values()
                .find(|item| item.value.with_value(|v| tab_value.with_value(|tv| v == tv)))
                .map(|item| {
                    (
                        item.trigger_id.with_value(|s| s.clone()),
                        item.panel_id.with_value(|s| s.clone()),
                    )
                })
        })
    });

    let is_selected =
        Memo::new(move |_| ctx.value.get().is_some_and(|v| tab_value.with_value(|tv| v == *tv)));

    view! {
        <div
            role="tabpanel"
            id={move || ids.get().as_ref().map(|(_, p)| p.clone())}
            aria-labelledby={move || ids.get().as_ref().map(|(t, _)| t.clone())}
            tabindex="0"
            data-state={move || if is_selected.get() { "active" } else { "inactive" }}
            data-orientation={if ctx.orientation == Orientation::Horizontal {
                "horizontal"
            } else {
                "vertical"
            }}
            class={class}
            style={move || if is_selected.get() { "" } else { "display: none;" }}
        >
            {children()}
        </div>
    }
}
