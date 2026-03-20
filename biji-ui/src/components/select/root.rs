use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use leptos::{
    context::Provider,
    ev::{click, focus, keydown, mouseover},
    leptos_dom::helpers::TimeoutHandle,
    prelude::*,
};
use leptos_use::{
    UseElementBoundingReturn, on_click_outside, use_document, use_element_bounding,
    use_event_listener,
};
use wasm_bindgen::JsCast;

use crate::{
    cn,
    custom_animated_show::CustomAnimatedShow,
    items::{FilterActiveItems, Focus, ManageFocus, NavigateItems},
    utils::positioning::{AvoidCollisions, Positioning},
};

use super::context::{SelectItemContext, SelectState};

static SELECT_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn next_select_id() -> String {
    let id = SELECT_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("biji-select-{id}")
}

fn build_state(
    value: Option<RwSignal<Option<String>>>,
    default_value: Option<String>,
    positioning: Positioning,
    hide_delay: Duration,
    avoid_collisions: AvoidCollisions,
) -> SelectState {
    let open = RwSignal::new(false);
    let data_state = Signal::derive(move || if open.get() { "open" } else { "closed" });
    let value_sig = value.unwrap_or_else(|| RwSignal::new(default_value));
    let state = SelectState {
        trigger_ref: NodeRef::new(),
        content_ref: NodeRef::new(),
        open,
        value: value_sig,
        selected_label: RwSignal::new(None),
        data_state,
        item_focus: RwSignal::new(None),
        items: RwSignal::new(Default::default()),
        hide_delay,
        positioning,
        arrow_size: 0,
        select_id: StoredValue::new(next_select_id()),
        avoid_collisions,
        next_id: StoredValue::new(AtomicUsize::new(0)),
    };
    // Resolve the initial label once items mount. selected_label starts as None
    // because Item children haven't registered yet at Root construction time.
    // This effect tracks state.items and runs whenever items change; once the
    // label is resolved it bails out immediately on subsequent runs.
    Effect::new(move |_| {
        if state.selected_label.get_untracked().is_some() {
            return;
        }
        let Some(val) = state.value.get_untracked() else {
            return;
        };
        state.items.with(|m| {
            if let Some(item) = m.values().find(|i| i.value.with_value(|iv| *iv == val)) {
                let lbl = item.label.with_value(|l| l.clone());
                state.selected_label.set(Some(lbl));
            }
        });
    });
    state
}

pub fn use_select() -> SelectState {
    expect_context::<SelectState>()
}

#[component]
pub fn RootWith<IV: IntoView + 'static>(
    children: impl Fn(SelectState) -> IV + Send + Sync + 'static,
    #[prop(into, optional)] class: String,
    /// Controlled signal. When provided, the select reads and writes this signal directly.
    #[prop(into, default = None)]
    value: Option<RwSignal<Option<String>>>,
    #[prop(into, default = None)] default_value: Option<String>,
    #[prop(default = Positioning::BottomStart)] positioning: Positioning,
    #[prop(default = Duration::from_millis(200))] hide_delay: Duration,
    #[prop(default = AvoidCollisions::Flip)] avoid_collisions: AvoidCollisions,
) -> impl IntoView {
    let state = build_state(value, default_value, positioning, hide_delay, avoid_collisions);
    view! {
        <Provider value={state}>
            <RootEvents>
                <div class={class}>{children(state)}</div>
            </RootEvents>
        </Provider>
    }
}

#[component]
pub fn Root(
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
    #[prop(into, default = None)] value: Option<RwSignal<Option<String>>>,
    #[prop(into, default = None)] default_value: Option<String>,
    #[prop(default = Positioning::BottomStart)] positioning: Positioning,
    #[prop(default = Duration::from_millis(200))] hide_delay: Duration,
    #[prop(default = AvoidCollisions::Flip)] avoid_collisions: AvoidCollisions,
) -> impl IntoView {
    view! {
        <RootWith
            value={value}
            default_value={default_value}
            positioning={positioning}
            hide_delay={hide_delay}
            avoid_collisions={avoid_collisions}
            class={class}
            let:_
        >
            {children()}
        </RootWith>
    }
}

#[component]
fn RootEvents(children: Children) -> impl IntoView {
    let ctx = expect_context::<SelectState>();

    let _ = use_event_listener(use_document(), keydown, move |evt| {
        if evt.key() == "Escape" && ctx.open.get() {
            ctx.close();
            if let Some(trigger) = ctx.trigger_ref.get() {
                let _ = trigger.focus();
            }
        }
    });

    let _ = on_click_outside(ctx.content_ref, move |evt| {
        if !ctx.open.get() {
            return;
        }
        let is_trigger_click = evt
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            .zip(ctx.trigger_ref.get())
            .is_some_and(|(el, trigger)| {
                let trigger_node: &web_sys::Node = trigger.as_ref();
                el.is_same_node(Some(trigger_node)) || trigger.contains(Some(&el))
            });
        if !is_trigger_click {
            ctx.close();
        }
    });

    children()
}

#[component]
pub fn Trigger(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let ctx = expect_context::<SelectState>();

    let _ = use_event_listener(ctx.trigger_ref, click, move |_| {
        ctx.toggle();
    });

    let _ = use_event_listener(ctx.trigger_ref, keydown, move |evt| {
        match evt.key().as_str() {
            "ArrowDown" => {
                evt.prevent_default();
                if !ctx.open.get() {
                    ctx.open();
                }
                // Focus first or next item after a brief delay to allow Content to mount
                leptos::leptos_dom::helpers::set_timeout(
                    move || {
                        if let Some(item) = ctx.navigate_first_item() {
                            item.focus();
                            ctx.set_focus(Some(item.index));
                        }
                    },
                    Duration::from_millis(10),
                );
            }
            "ArrowUp" => {
                evt.prevent_default();
                if !ctx.open.get() {
                    ctx.open();
                }
                leptos::leptos_dom::helpers::set_timeout(
                    move || {
                        if let Some(item) = ctx.navigate_last_item() {
                            item.focus();
                            ctx.set_focus(Some(item.index));
                        }
                    },
                    Duration::from_millis(10),
                );
            }
            _ => {}
        }
    });

    view! {
        <button
            node_ref={ctx.trigger_ref}
            type="button"
            role="combobox"
            aria-expanded={move || if ctx.open.get() { "true" } else { "false" }}
            aria-haspopup="listbox"
            aria-controls={ctx.select_id.get_value()}
            data-state={move || if ctx.open.get() { "open" } else { "closed" }}
            class={class}
        >
            {children()}
        </button>
    }
}

#[component]
pub fn Value(#[prop(into, optional)] placeholder: String) -> impl IntoView {
    let ctx = expect_context::<SelectState>();
    view! { <span>{move || { ctx.selected_label.get().unwrap_or_else(|| placeholder.clone()) }}</span> }
}

#[component]
pub fn Content(
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] show_class: String,
    #[prop(into, optional)] hide_class: String,
) -> impl IntoView {
    let ctx = expect_context::<SelectState>();
    let content_ref = ctx.content_ref;

    let UseElementBoundingReturn {
        width: content_width,
        height: content_height,
        ..
    } = use_element_bounding(content_ref);

    let UseElementBoundingReturn {
        top,
        left,
        width,
        height,
        ..
    } = use_element_bounding(ctx.trigger_ref);

    let style_signal = Signal::derive(move || {
        let raw_cw = *content_width.read();
        let raw_ch = *content_height.read();
        let _ = ctx.open.get();
        let _ = top.read();
        let _ = left.read();
        let _ = width.read();
        let _ = height.read();
        let hidden = || {
            format!(
                "position: fixed; top: 0; left: 0; visibility: hidden; --biji-transform-origin: {};",
                ctx.positioning.transform_origin()
            )
        };
        if raw_cw == 0.0 && raw_ch == 0.0 {
            return hidden();
        }
        let Some(content_div) = content_ref.get_untracked() else {
            return hidden();
        };
        let content_node: &web_sys::Node = content_div.as_ref();
        let Some(content_html) = content_node.dyn_ref::<web_sys::HtmlElement>() else {
            return hidden();
        };
        let cw = content_html.offset_width() as f64;
        let ch = content_html.offset_height() as f64;
        if cw == 0.0 && ch == 0.0 {
            return hidden();
        }
        let Some(trigger) = ctx.trigger_ref.get_untracked() else {
            return hidden();
        };
        let trigger_node: &web_sys::Node = trigger.as_ref();
        let Some(trigger_el) = trigger_node.dyn_ref::<web_sys::Element>() else {
            return hidden();
        };
        let rect = trigger_el.get_bounding_client_rect();
        let (t_top, t_left, t_width, t_height) =
            (rect.top(), rect.left(), rect.width(), rect.height());
        let vp_w = web_sys::window()
            .and_then(|w| w.inner_width().ok())
            .and_then(|v| v.as_f64())
            .unwrap_or(1920.0);
        let vp_h = web_sys::window()
            .and_then(|w| w.inner_height().ok())
            .and_then(|v| v.as_f64())
            .unwrap_or(1080.0);
        let eff = ctx.positioning.effective_positioning(
            cw,
            ch,
            t_top,
            t_left,
            t_width,
            t_height,
            ctx.arrow_size as f64,
            vp_w,
            vp_h,
            ctx.avoid_collisions,
        );
        eff.calculate_position_style(
            t_top,
            t_left,
            t_width,
            t_height,
            ch,
            cw,
            ctx.arrow_size as f64,
            ctx.arrow_size as f64,
        )
    });

    // Auto-focus the selected item (or first item) when content opens.
    let focus_handle: Arc<Mutex<Option<TimeoutHandle>>> = Arc::new(Mutex::new(None));
    let focus_handle_cleanup = Arc::clone(&focus_handle);
    let focus_eff = RenderEffect::new(move |_| {
        if let Some(h) = focus_handle.lock().unwrap().take() {
            h.clear();
        }
        if ctx.open.get() {
            let fh = Arc::clone(&focus_handle);
            let h = leptos::leptos_dom::helpers::set_timeout_with_handle(
                move || {
                    *fh.lock().unwrap() = None;
                    let current_val = ctx.value.get_untracked();
                    // Try to find and focus the currently selected item.
                    let selected = current_val.and_then(|v| {
                        ctx.items.with_untracked(|m| {
                            m.values()
                                .find(|item| item.value.with_value(|iv| *iv == v))
                                .copied()
                        })
                    });
                    if let Some(item) = selected {
                        item.focus();
                        ctx.item_focus.set(Some(item.index));
                    } else if let Some(item) = ctx.filter_active_items().into_iter().next() {
                        item.focus();
                        ctx.item_focus.set(Some(item.index));
                    }
                },
                Duration::from_millis(10),
            )
            .expect("set_timeout in select focus");
            *focus_handle.lock().unwrap() = Some(h);
        }
    });

    on_cleanup(move || {
        if let Some(h) = focus_handle_cleanup.lock().unwrap().take() {
            h.clear();
        }
        drop(focus_eff);
    });

    // Keyboard navigation: events bubble from focused item divs up to content_ref.
    let _ = use_event_listener(content_ref, keydown, move |evt| {
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
            "Enter" | " " => {
                evt.prevent_default();
                if let Some(focused_idx) = ctx.item_focus.get_untracked() {
                    let item = ctx.items.with_untracked(|m| m.get(&focused_idx).copied());
                    if let Some(item) = item {
                        if !item.disabled {
                            let val = item.value.with_value(|v| v.clone());
                            let lbl = item.label.with_value(|l| l.clone());
                            ctx.select(val, lbl);
                            if let Some(trigger) = ctx.trigger_ref.get() {
                                let _ = trigger.focus();
                            }
                        }
                    }
                }
            }
            "Tab" => {
                // Close without selecting on Tab; let focus move naturally.
                ctx.close();
            }
            _ => {}
        }
    });

    view! {
        <CustomAnimatedShow
            when={ctx.open}
            show_class={cn!(class, show_class)}
            hide_class={cn!(class, hide_class)}
            hide_delay={ctx.hide_delay}
            style_signal={style_signal}
            node_ref={content_ref}
            attr:id={ctx.select_id.get_value()}
            attr:role="listbox"
            attr:tabindex="-1"
        >
            {children()}
        </CustomAnimatedShow>
    }
}

#[component]
pub fn Item(
    children: Children,
    #[prop(into)] value: String,
    /// Display text shown in the trigger when this item is selected.
    /// Defaults to `value` if not provided.
    #[prop(into, optional)]
    label: Option<String>,
    #[prop(into, optional)] class: String,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let ctx = expect_context::<SelectState>();

    let index = ctx.next_index();
    let label_text = label.unwrap_or_else(|| value.clone());
    let item_ctx = SelectItemContext {
        index,
        value: StoredValue::new(value),
        label: StoredValue::new(label_text),
        disabled,
        item_ref: NodeRef::new(),
    };

    ctx.upsert_item(index, item_ctx);

    on_cleanup(move || {
        ctx.remove_item(index);
    });

    let _ = use_event_listener(item_ctx.item_ref, click, move |_| {
        if item_ctx.disabled {
            return;
        }
        let val = item_ctx.value.with_value(|v| v.clone());
        let lbl = item_ctx.label.with_value(|l| l.clone());
        ctx.select(val, lbl);
        if let Some(trigger) = ctx.trigger_ref.get() {
            let _ = trigger.focus();
        }
    });

    let _ = use_event_listener(item_ctx.item_ref, mouseover, move |_| {
        if !item_ctx.disabled {
            ctx.set_focus(Some(item_ctx.index));
        }
    });

    let _ = use_event_listener(item_ctx.item_ref, focus, move |_| {
        ctx.set_focus(Some(item_ctx.index));
    });

    let is_selected = Memo::new(move |_| {
        ctx.value
            .get()
            .is_some_and(|v| item_ctx.value.with_value(|iv| v == *iv))
    });

    view! {
        <Provider value={item_ctx}>
            <div
                node_ref={item_ctx.item_ref}
                role="option"
                tabindex="-1"
                aria-selected={move || if is_selected.get() { "true" } else { "false" }}
                aria-disabled={if item_ctx.disabled { Some("true") } else { None }}
                data-state={move || if is_selected.get() { "checked" } else { "unchecked" }}
                data-disabled={item_ctx.disabled}
                data-highlighted={move || ctx.item_in_focus(item_ctx.index)}
                class={class}
            >
                {children()}
            </div>
        </Provider>
    }
}

#[component]
pub fn ItemText(children: Children) -> impl IntoView {
    view! { <span>{children()}</span> }
}

#[component]
pub fn ItemIndicator(children: ChildrenFn) -> impl IntoView {
    let ctx = expect_context::<SelectState>();
    let item_ctx = expect_context::<SelectItemContext>();

    view! {
        {move || {
            let is_selected = ctx
                .value
                .get()
                .is_some_and(|v| item_ctx.value.with_value(|iv| v == *iv));
            if is_selected { Some(children()) } else { None }
        }}
    }
}
