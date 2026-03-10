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
    items::{Focus, ManageFocus, NavigateItems},
    utils::positioning::{AvoidCollisions, Positioning},
};

use super::context::{ComboboxContext, ComboboxItemContext};

static COMBOBOX_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn next_combobox_id() -> String {
    let id = COMBOBOX_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("biji-combobox-{id}")
}

#[component]
pub fn Root(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] value: Option<String>,
    #[prop(default = Positioning::BottomStart)] positioning: Positioning,
    #[prop(default = Duration::from_millis(200))] hide_delay: Duration,
    #[prop(default = AvoidCollisions::Flip)] avoid_collisions: AvoidCollisions,
    #[prop(optional)] on_value_change: Option<Callback<String>>,
    /// Set to `true` when using `InputTrigger` (the inline Headless-UI-style combobox).
    #[prop(default = false)] inline: bool,
) -> impl IntoView {
    let ctx = ComboboxContext {
        open: RwSignal::new(false),
        value: RwSignal::new(value),
        selected_label: RwSignal::new(None),
        query: RwSignal::new(String::new()),
        hide_delay,
        positioning,
        combobox_id: StoredValue::new(next_combobox_id()),
        avoid_collisions,
        on_value_change,
        inline_mode: inline,
        ..ComboboxContext::default()
    };

    // Resolve the initial label once items mount (same pattern as Select).
    Effect::new(move |_| {
        if ctx.selected_label.get_untracked().is_some() {
            return;
        }
        let Some(val) = ctx.value.get_untracked() else {
            return;
        };
        ctx.items.with(|m| {
            if let Some(item) = m.values().find(|i| i.value.with_value(|iv| *iv == val)) {
                let lbl = item.label.with_value(|l| l.clone());
                ctx.selected_label.set(Some(lbl));
            }
        });
    });

    view! {
        <Provider value={ctx}>
            <RootEvents>
                <div class={class}>{children()}</div>
            </RootEvents>
        </Provider>
    }
}

#[component]
fn RootEvents(children: Children) -> impl IntoView {
    let ctx = expect_context::<ComboboxContext>();

    let _ = use_event_listener(use_document(), keydown, move |evt| {
        if evt.key() == "Escape" && ctx.open.get() {
            ctx.close();
            if ctx.inline_mode {
                // Suppress the focus event from re-opening the dropdown.
                ctx.suppress_next_open.set_value(true);
                if let Some(input) = ctx.input_ref.get() {
                    let _ = input.focus();
                }
            } else if let Some(trigger) = ctx.trigger_ref.get() {
                let _ = trigger.focus();
            }
        }
    });

    let _ = on_click_outside(ctx.content_ref, move |evt| {
        if !ctx.open.get() {
            return;
        }
        let target_el = evt
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok());

        // Filter out clicks on the button trigger (standard mode).
        let is_trigger_click = target_el
            .as_ref()
            .zip(ctx.trigger_ref.get())
            .is_some_and(|(el, trigger)| {
                let trigger_node: &web_sys::Node = trigger.as_ref();
                el.is_same_node(Some(trigger_node)) || trigger.contains(Some(el))
            });

        // Filter out clicks on the input trigger (inline mode).
        let is_input_click = ctx.inline_mode
            && target_el
                .as_ref()
                .zip(ctx.input_ref.get())
                .is_some_and(|(el, input)| {
                    let input_node: &web_sys::Node = input.as_ref();
                    el.is_same_node(Some(input_node)) || input_node.contains(Some(el))
                });

        if !is_trigger_click && !is_input_click {
            ctx.close();
        }
    });

    children()
}

#[component]
pub fn Trigger(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    let ctx = expect_context::<ComboboxContext>();

    let _ = use_event_listener(ctx.trigger_ref, leptos::ev::click, move |_| {
        ctx.toggle();
    });

    let _ = use_event_listener(ctx.trigger_ref, keydown, move |evt| {
        match evt.key().as_str() {
            "ArrowDown" => {
                evt.prevent_default();
                if !ctx.open.get() {
                    ctx.open();
                }
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
            aria-controls={ctx.combobox_id.get_value()}
            data-state={move || if ctx.open.get() { "open" } else { "closed" }}
            class={class}
        >
            {children()}
        </button>
    }
}

#[component]
pub fn Value(#[prop(into, optional)] placeholder: String) -> impl IntoView {
    let ctx = expect_context::<ComboboxContext>();
    view! {
        <span>
            {move || ctx.selected_label.get().unwrap_or_else(|| placeholder.clone())}
        </span>
    }
}

#[component]
pub fn Content(
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] show_class: String,
    #[prop(into, optional)] hide_class: String,
) -> impl IntoView {
    let ctx = expect_context::<ComboboxContext>();
    let content_ref = ctx.content_ref;

    let UseElementBoundingReturn {
        width: content_width,
        height: content_height,
        ..
    } = use_element_bounding(content_ref);

    // Both NodeRefs are always called (hooks rule) — we pick the right one at runtime.
    let UseElementBoundingReturn {
        top: btn_top,
        left: btn_left,
        width: btn_width,
        height: btn_height,
        ..
    } = use_element_bounding(ctx.trigger_ref);

    let UseElementBoundingReturn {
        top: inp_top,
        left: inp_left,
        width: inp_width,
        height: inp_height,
        ..
    } = use_element_bounding(ctx.input_ref);

    let style_signal = Signal::derive(move || {
        let raw_cw = *content_width.read();
        let raw_ch = *content_height.read();
        let _ = ctx.open.get();
        // Subscribe to the correct anchor's bounds.
        if ctx.inline_mode {
            let _ = inp_top.read();
            let _ = inp_left.read();
            let _ = inp_width.read();
            let _ = inp_height.read();
        } else {
            let _ = btn_top.read();
            let _ = btn_left.read();
            let _ = btn_width.read();
            let _ = btn_height.read();
        }
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
        // Get bounding rect from the active anchor element.
        let anchor_el: Option<web_sys::Element> = if ctx.inline_mode {
            ctx.input_ref.get_untracked().and_then(|el| {
                let node: &web_sys::Node = el.as_ref();
                node.dyn_ref::<web_sys::Element>().cloned()
            })
        } else {
            ctx.trigger_ref.get_untracked().and_then(|btn| {
                let node: &web_sys::Node = btn.as_ref();
                node.dyn_ref::<web_sys::Element>().cloned()
            })
        };
        let Some(trigger_el) = anchor_el else {
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

    // Standard mode only: when the panel opens, clear the input's DOM value and focus it.
    // In inline mode, InputTrigger manages the input display and focus itself.
    let focus_handle: Arc<Mutex<Option<TimeoutHandle>>> = Arc::new(Mutex::new(None));
    let focus_handle_cleanup = Arc::clone(&focus_handle);
    let focus_eff = RenderEffect::new(move |_| {
        if let Some(h) = focus_handle.lock().unwrap().take() {
            h.clear();
        }
        if !ctx.inline_mode && ctx.open.get() {
            let fh = Arc::clone(&focus_handle);
            let h = leptos::leptos_dom::helpers::set_timeout_with_handle(
                move || {
                    *fh.lock().unwrap() = None;
                    if let Some(input) = ctx.input_ref.get() {
                        input.set_value("");
                        let _ = input.focus();
                    }
                },
                Duration::from_millis(10),
            )
            .expect("set_timeout in combobox focus");
            *focus_handle.lock().unwrap() = Some(h);
        }
    });

    on_cleanup(move || {
        if let Some(h) = focus_handle_cleanup.lock().unwrap().take() {
            h.clear();
        }
        drop(focus_eff);
    });

    // Keyboard navigation — arrow keys bubble up from the input or items.
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
            "Enter" => {
                evt.prevent_default();
                if let Some(focused_idx) = ctx.item_focus.get_untracked() {
                    let item = ctx.items.with_untracked(|m| m.get(&focused_idx).copied());
                    if let Some(item) = item {
                        if !item.disabled {
                            let val = item.value.with_value(|v| v.clone());
                            let lbl = item.label.with_value(|l| l.clone());
                            ctx.select(val, lbl);
                            if ctx.inline_mode {
                                ctx.suppress_next_open.set_value(true);
                                if let Some(input) = ctx.input_ref.get() {
                                    let _ = input.focus();
                                }
                            } else if let Some(trigger) = ctx.trigger_ref.get() {
                                let _ = trigger.focus();
                            }
                        }
                    }
                }
            }
            "Tab" => {
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
            attr:id={ctx.combobox_id.get_value()}
            attr:role="listbox"
            attr:tabindex="-1"
        >
            {children()}
        </CustomAnimatedShow>
    }
}

#[component]
pub fn Input(
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] placeholder: String,
) -> impl IntoView {
    let ctx = expect_context::<ComboboxContext>();

    let _ = use_event_listener(ctx.input_ref, leptos::ev::input, move |evt| {
        let val = event_target_value(&evt);
        ctx.query.set(val);
        // Reset item focus so next arrow key starts from first visible item.
        ctx.item_focus.set(None);
    });

    view! {
        <input
            node_ref={ctx.input_ref}
            type="text"
            placeholder={placeholder}
            autocomplete="off"
            class={class}
        />
    }
}

/// An input that acts as both the trigger and the search field (Headless UI style).
///
/// Place this outside `Content`, alongside the optional chevron `Trigger` button.
/// The input shows the selected label when closed, selects all text on open so the
/// user can immediately type to filter, and restores the label on close.
///
/// Requires `<combobox::Root inline=true>`.
#[component]
pub fn InputTrigger(
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] placeholder: String,
) -> impl IntoView {
    let ctx = expect_context::<ComboboxContext>();

    // On focus: open the dropdown and select all text so typing replaces the label.
    let _ = use_event_listener(ctx.input_ref, leptos::ev::focus, move |_| {
        if ctx.suppress_next_open.get_value() {
            ctx.suppress_next_open.set_value(false);
            return;
        }
        if !ctx.open.get() {
            ctx.query.set(String::new());
            ctx.open.set(true);
            if let Some(input) = ctx.input_ref.get_untracked() {
                let _ = input.select();
            }
        }
    });

    // On input: update the query and reset item focus.
    let _ = use_event_listener(ctx.input_ref, leptos::ev::input, move |evt| {
        let val = event_target_value(&evt);
        ctx.query.set(val);
        ctx.item_focus.set(None);
    });

    // Keyboard navigation — input is outside Content so events won't bubble there.
    let _ = use_event_listener(ctx.input_ref, keydown, move |evt| {
        match evt.key().as_str() {
            "ArrowDown" => {
                evt.prevent_default();
                if !ctx.open.get() {
                    ctx.query.set(String::new());
                    ctx.open.set(true);
                }
                if let Some(item) = ctx.navigate_next_item() {
                    item.focus();
                    ctx.set_focus(Some(item.index));
                }
            }
            "ArrowUp" => {
                evt.prevent_default();
                if !ctx.open.get() {
                    ctx.query.set(String::new());
                    ctx.open.set(true);
                }
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
            "Enter" => {
                evt.prevent_default();
                if let Some(focused_idx) = ctx.item_focus.get_untracked() {
                    let item = ctx.items.with_untracked(|m| m.get(&focused_idx).copied());
                    if let Some(item) = item {
                        if !item.disabled {
                            let val = item.value.with_value(|v| v.clone());
                            let lbl = item.label.with_value(|l| l.clone());
                            ctx.select(val, lbl);
                            // suppress_next_open not needed — we're already in the input
                        }
                    }
                }
            }
            "Escape" => {
                if ctx.open.get() {
                    ctx.close();
                    // suppress focus re-open; the input retains focus naturally
                    ctx.suppress_next_open.set_value(true);
                }
            }
            "Tab" => {
                ctx.close();
            }
            _ => {}
        }
    });

    // Manage the input's display value: show selected label when closed.
    let restore_eff = RenderEffect::new(move |_| {
        if !ctx.open.get() {
            if let Some(input) = ctx.input_ref.get() {
                let label = ctx.selected_label.get().unwrap_or_default();
                input.set_value(&label);
            }
        }
    });

    on_cleanup(move || drop(restore_eff));

    view! {
        <input
            node_ref={ctx.input_ref}
            type="text"
            placeholder={placeholder}
            autocomplete="off"
            class={class}
        />
    }
}

#[component]
pub fn Item(
    children: Children,
    #[prop(into)] value: String,
    #[prop(into, optional)] label: Option<String>,
    #[prop(into, optional)] class: String,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let ctx = expect_context::<ComboboxContext>();

    let index = ctx.next_index();
    let label_text = label.unwrap_or_else(|| value.clone());
    let item_ctx = ComboboxItemContext {
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

    // Item is hidden when it doesn't match the current query.
    let is_visible = Memo::new(move |_| {
        let q = ctx.query.get().to_lowercase();
        if q.is_empty() {
            return true;
        }
        item_ctx.label.with_value(|l| l.to_lowercase().contains(&q))
    });

    let _ = use_event_listener(item_ctx.item_ref, click, move |_| {
        if item_ctx.disabled {
            return;
        }
        let val = item_ctx.value.with_value(|v| v.clone());
        let lbl = item_ctx.label.with_value(|l| l.clone());
        ctx.select(val, lbl);
        if ctx.inline_mode {
            ctx.suppress_next_open.set_value(true);
            if let Some(input) = ctx.input_ref.get() {
                let _ = input.focus();
            }
        } else if let Some(trigger) = ctx.trigger_ref.get() {
            let _ = trigger.focus();
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

    let is_selected = Memo::new(move |_| {
        ctx.value.get().is_some_and(|v| item_ctx.value.with_value(|iv| v == *iv))
    });

    view! {
        <Provider value={item_ctx}>
            <div
                node_ref={item_ctx.item_ref}
                role="option"
                tabindex="-1"
                aria-selected={move || if is_selected.get() { "true" } else { "false" }}
                aria-hidden={move || if !is_visible.get() { Some("true") } else { None }}
                aria-disabled={if item_ctx.disabled { Some("true") } else { None }}
                data-state={move || if is_selected.get() { "checked" } else { "unchecked" }}
                data-disabled={item_ctx.disabled}
                data-highlighted={move || ctx.item_in_focus(item_ctx.index)}
                style={move || if is_visible.get() { "" } else { "display: none;" }}
                class={class}
            >
                {children()}
            </div>
        </Provider>
    }
}

#[component]
pub fn ItemText(children: Children) -> impl IntoView {
    view! {
        <span>{children()}</span>
    }
}

#[component]
pub fn ItemIndicator(children: ChildrenFn) -> impl IntoView {
    let ctx = expect_context::<ComboboxContext>();
    let item_ctx = expect_context::<ComboboxItemContext>();

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

#[component]
pub fn Empty(children: ChildrenFn) -> impl IntoView {
    let ctx = expect_context::<ComboboxContext>();
    let has_visible = Memo::new(move |_| !ctx.visible_items().is_empty());

    view! {
        <Show when={move || !has_visible.get()} fallback={|| ()}>
            {children()}
        </Show>
    }
}
