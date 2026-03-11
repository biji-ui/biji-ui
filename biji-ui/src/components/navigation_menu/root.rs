use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use leptos::{
    context::Provider,
    ev::{click, focus, keydown, pointerenter, pointerleave},
    prelude::*,
};
use leptos_use::{
    UseElementBoundingReturn, use_document, use_element_bounding, use_event_listener,
};
use wasm_bindgen::JsCast;

use crate::{
    cn,
    custom_animated_show::CustomAnimatedShow,
    items::{FilterActiveItems, Focus, ManageFocus, NavigateItems},
    utils::positioning::{AvoidCollisions, Positioning},
};

use super::context::{NavMenuContext, NavMenuItemContext};

static NAV_MENU_ROOT_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Marker placed in context by `Content` so that `Link` can distinguish
/// "inside a panel" (don't register as nav item) from "top-level nav link".
#[derive(Copy, Clone)]
struct InsideNavContent;

fn next_root_id() -> usize {
    NAV_MENU_ROOT_COUNTER.fetch_add(1, Ordering::Relaxed)
}

fn item_ids(root_id: usize, index: usize) -> (String, String) {
    (
        format!("biji-navmenu-trigger-{root_id}-{index}"),
        format!("biji-navmenu-content-{root_id}-{index}"),
    )
}

#[component]
pub fn Root(
    children: Children,
    #[prop(into, optional)] class: String,
    /// Optional `aria-label` for the `<nav>` element.
    #[prop(into, optional)]
    aria_label: Option<String>,
    #[prop(default = Positioning::BottomStart)] positioning: Positioning,
    #[prop(default = AvoidCollisions::Flip)] avoid_collisions: AvoidCollisions,
    /// How long after the pointer leaves before the open panel closes.
    #[prop(default = Duration::from_millis(200))]
    close_delay: Duration,
    /// How long `CustomAnimatedShow` waits before unmounting the content.
    #[prop(default = Duration::from_millis(200))]
    hide_delay: Duration,
) -> impl IntoView {
    let close_timer = Arc::new(Mutex::new(None));
    let cleanup_timer = Arc::clone(&close_timer);

    let ctx = NavMenuContext {
        open_value: RwSignal::new(None),
        item_focus: RwSignal::new(None),
        items: RwSignal::new(Default::default()),
        positioning,
        avoid_collisions,
        close_delay,
        hide_delay,
        close_timer: StoredValue::new(close_timer),
        root_id: next_root_id(),
        next_id: StoredValue::new(AtomicUsize::new(0)),
    };

    on_cleanup(move || {
        if let Some(h) = cleanup_timer.lock().unwrap().take() {
            h.clear();
        }
    });

    view! {
        <Provider value={ctx}>
            <RootEvents>
                <nav aria-label={aria_label} class={class}>
                    {children()}
                </nav>
            </RootEvents>
        </Provider>
    }
}

#[component]
fn RootEvents(children: Children) -> impl IntoView {
    let ctx = expect_context::<NavMenuContext>();

    // Escape: close immediately and return focus to the previously-open trigger.
    let _ = use_event_listener(use_document(), keydown, move |evt| {
        if evt.key() != "Escape" || !ctx.any_open() {
            return;
        }
        let open_val = ctx.open_value.get_untracked();
        ctx.close_immediate();
        if let Some(val) = open_val {
            ctx.items.with_untracked(|items| {
                for item in items.values() {
                    if item.value.with_value(|v| *v == val) {
                        item.focus();
                        break;
                    }
                }
            });
        }
    });

    // Document click: close if the click lands outside all triggers and content panels.
    let _ = use_event_listener(use_document(), click, move |evt| {
        if !ctx.any_open() {
            return;
        }
        let target = evt
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok());
        let Some(target) = target else { return };
        let inside = ctx.items.with_untracked(|items| {
            items.values().any(|item| {
                let t = target.as_ref();
                if let Some(el) = item.trigger_ref.get() {
                    let node: &web_sys::Node = el.as_ref();
                    if node.contains(Some(t)) {
                        return true;
                    }
                }
                if let Some(el) = item.link_ref.get() {
                    let node: &web_sys::Node = el.as_ref();
                    if node.contains(Some(t)) {
                        return true;
                    }
                }
                if let Some(el) = item.content_ref.get() {
                    let node: &web_sys::Node = el.as_ref();
                    if node.contains(Some(t)) {
                        return true;
                    }
                }
                false
            })
        });
        if !inside {
            ctx.close_immediate();
        }
    });

    children()
}

#[component]
pub fn List(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    view! {
        <ul class={class}>
            {children()}
        </ul>
    }
}

#[component]
pub fn Item(
    children: Children,
    /// A unique string that identifies this item.  Must match the `value` prop
    /// of the corresponding `Content` component.
    #[prop(into)]
    value: String,
    #[prop(into, optional)] class: String,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let ctx = expect_context::<NavMenuContext>();

    let index = ctx.next_index();
    let (trigger_id, content_id) = item_ids(ctx.root_id, index);

    let item_ctx = NavMenuItemContext {
        index,
        value: StoredValue::new(value),
        disabled,
        trigger_ref: NodeRef::new(),
        link_ref: NodeRef::new(),
        content_ref: NodeRef::new(),
        trigger_id: StoredValue::new(trigger_id),
        content_id: StoredValue::new(content_id),
        has_content: RwSignal::new(false),
    };

    ctx.upsert_item(index, item_ctx);

    on_cleanup(move || {
        ctx.remove_item(index);
    });

    view! {
        <Provider value={item_ctx}>
            <li class={class} data-disabled={disabled}>
                {children()}
            </li>
        </Provider>
    }
}

#[component]
pub fn Trigger(
    children: Children,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let ctx = expect_context::<NavMenuContext>();
    let item_ctx = expect_context::<NavMenuItemContext>();

    let is_open = Memo::new(move |_| item_ctx.value.with_value(|v| ctx.is_open(v)));

    let tabindex = move || {
        if item_ctx.disabled {
            return "-1";
        }
        if is_open.get() || ctx.item_in_focus(item_ctx.index) {
            return "0";
        }
        // When nothing is open and no item has roving focus, keep the first
        // active item in the tab order so Tab lands somewhere sensible.
        if ctx.open_value.get().is_none()
            && ctx.item_focus.get().is_none()
            && ctx
                .filter_active_items()
                .into_iter()
                .next()
                .map(|i| i.index)
                == Some(item_ctx.index)
        {
            return "0";
        }
        "-1"
    };

    view! {
        <TriggerEvents>
            <button
                node_ref={item_ctx.trigger_ref}
                type="button"
                id={item_ctx.trigger_id.get_value()}
                aria-expanded={move || if is_open.get() { "true" } else { "false" }}
                aria-controls={move || item_ctx.has_content.get().then(|| item_ctx.content_id.get_value())}
                aria-haspopup={move || item_ctx.has_content.get().then_some("true")}
                aria-disabled={if item_ctx.disabled { Some("true") } else { None }}
                data-state={move || if is_open.get() { "open" } else { "closed" }}
                data-disabled={item_ctx.disabled}
                data-highlighted={move || ctx.item_in_focus(item_ctx.index)}
                tabindex={tabindex}
                class={class}
            >
                {children()}
            </button>
        </TriggerEvents>
    }
}

#[component]
fn TriggerEvents(children: Children) -> impl IntoView {
    let ctx = expect_context::<NavMenuContext>();
    let item_ctx = expect_context::<NavMenuItemContext>();

    // Hover open
    let _ = use_event_listener(item_ctx.trigger_ref, pointerenter, move |_| {
        if item_ctx.disabled {
            return;
        }
        let value = item_ctx.value.with_value(|v| v.clone());
        ctx.open(value);
    });

    // Hover leave: start close timer
    let _ = use_event_listener(item_ctx.trigger_ref, pointerleave, move |_| {
        ctx.schedule_close();
    });

    // Click: toggle
    let _ = use_event_listener(item_ctx.trigger_ref, click, move |_| {
        if item_ctx.disabled {
            return;
        }
        let value = item_ctx.value.with_value(|v| v.clone());
        if ctx.is_open(&value) {
            ctx.close_immediate();
        } else {
            ctx.open(value);
        }
    });

    // Focus: update roving tabindex state
    let _ = use_event_listener(item_ctx.trigger_ref, focus, move |_| {
        ctx.set_focus(Some(item_ctx.index));
    });

    let _ = use_event_listener(item_ctx.trigger_ref, keydown, move |evt| {
        if item_ctx.disabled {
            return;
        }
        let value = item_ctx.value.with_value(|v| v.clone());
        match evt.key().as_str() {
            "Enter" | " " => {
                evt.prevent_default();
                if ctx.is_open(&value) {
                    ctx.close_immediate();
                } else {
                    ctx.open(value);
                }
            }
            "ArrowRight" => {
                evt.prevent_default();
                if let Some(next) = ctx.navigate_next_item() {
                    next.focus();
                    if ctx.any_open() {
                        let next_val = next.value.with_value(|v| v.clone());
                        ctx.open(next_val);
                    }
                }
            }
            "ArrowLeft" | "ArrowUp" => {
                evt.prevent_default();
                if let Some(prev) = ctx.navigate_previous_item() {
                    prev.focus();
                    if ctx.any_open() {
                        let prev_val = prev.value.with_value(|v| v.clone());
                        ctx.open(prev_val);
                    }
                }
            }
            "Home" => {
                evt.prevent_default();
                if let Some(first) = ctx.navigate_first_item() {
                    first.focus();
                    if ctx.any_open() {
                        let first_val = first.value.with_value(|v| v.clone());
                        ctx.open(first_val);
                    }
                }
            }
            "End" => {
                evt.prevent_default();
                if let Some(last) = ctx.navigate_last_item() {
                    last.focus();
                    if ctx.any_open() {
                        let last_val = last.value.with_value(|v| v.clone());
                        ctx.open(last_val);
                    }
                }
            }
            "ArrowDown" => {
                evt.prevent_default();
                // Open the panel and focus its first focusable element
                if item_ctx.has_content.get_untracked() {
                    ctx.open(value);
                    let content_ref = item_ctx.content_ref;
                    set_timeout(
                        move || {
                            if let Some(el) = content_ref.get() {
                                focus_first_in_content(&el);
                            }
                        },
                        Duration::from_millis(10),
                    );
                }
            }
            // Tab (forward only): if a panel is open, move focus into it instead of
            // skipping past it (Content is in a portal, so it's not in natural tab order).
            "Tab" => {
                if !evt.shift_key()
                    && ctx.is_open(&value)
                    && item_ctx.has_content.get_untracked()
                {
                    evt.prevent_default();
                    let content_ref = item_ctx.content_ref;
                    set_timeout(
                        move || {
                            if let Some(el) = content_ref.get() {
                                focus_first_in_content(&el);
                            }
                        },
                        Duration::from_millis(0),
                    );
                }
            }
            _ => {}
        }
    });

    children()
}

fn focus_first_in_content(container: &web_sys::HtmlElement) {
    let selector = r#"a[href], button:not([disabled]), textarea:not([disabled]), input:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex="-1"])"#;
    let Ok(node_list) = container.query_selector_all(selector) else {
        return;
    };
    for i in 0..node_list.length() {
        if let Some(node) = node_list.get(i) {
            if let Ok(el) = node.dyn_into::<web_sys::HtmlElement>() {
                let _ = el.focus();
                return;
            }
        }
    }
    // No focusable child — focus the container itself.
    let _ = container.focus();
}

#[component]
pub fn Content(
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] show_class: String,
    #[prop(into, optional)] hide_class: String,
) -> impl IntoView {
    let ctx = expect_context::<NavMenuContext>();
    let item_ctx = expect_context::<NavMenuItemContext>();

    // Tell the trigger that this item has a content panel.
    item_ctx.has_content.set(true);
    on_cleanup(move || {
        item_ctx.has_content.set(false);
    });

    // Mark descendants so top-level Links don't mistakenly register as nav items.
    provide_context(InsideNavContent);

    let value = StoredValue::new(item_ctx.value.with_value(|v| v.clone()));
    let is_open = Signal::derive(move || {
        value.with_value(|v| ctx.open_value.get().as_deref() == Some(v))
    });

    // Keep panel open while cursor is inside; start close timer on leave.
    let _ = use_event_listener(item_ctx.content_ref, pointerenter, move |_| {
        ctx.cancel_close_timer();
    });
    let _ = use_event_listener(item_ctx.content_ref, pointerleave, move |_| {
        ctx.schedule_close();
    });

    // Positioning — identical to menubar::MenuContent pattern.
    let UseElementBoundingReturn {
        width: content_width,
        height: content_height,
        ..
    } = use_element_bounding(item_ctx.content_ref);

    let UseElementBoundingReturn {
        top: trigger_top,
        left: trigger_left,
        width: trigger_width,
        height: trigger_height,
        ..
    } = use_element_bounding(item_ctx.trigger_ref);

    let style_signal = Signal::derive(move || {
        let raw_cw = *content_width.read();
        let raw_ch = *content_height.read();
        let _ = is_open.get();
        let _ = trigger_top.read();
        let _ = trigger_left.read();
        let _ = trigger_width.read();
        let _ = trigger_height.read();
        let hidden = || {
            format!(
                "position: fixed; top: 0; left: 0; visibility: hidden; --biji-transform-origin: {};",
                ctx.positioning.transform_origin()
            )
        };
        if raw_cw == 0.0 && raw_ch == 0.0 {
            return hidden();
        }
        let Some(content_div) = item_ctx.content_ref.get_untracked() else {
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
        let Some(trigger) = item_ctx.trigger_ref.get_untracked() else {
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
            0.0,
            vp_w,
            vp_h,
            ctx.avoid_collisions,
        );
        eff.calculate_position_style_simple(t_top, t_left, t_width, t_height, ch, cw, 0.0)
    });

    view! {
        <CustomAnimatedShow
            when={is_open}
            show_class={cn!(class, show_class)}
            hide_class={cn!(class, hide_class)}
            hide_delay={ctx.hide_delay}
            style_signal={style_signal}
            node_ref={item_ctx.content_ref}
            attr:id={item_ctx.content_id.get_value()}
            attr:role="region"
            attr:aria-labelledby={item_ctx.trigger_id.get_value()}
            attr:tabindex="-1"
        >
            {children()}
        </CustomAnimatedShow>
    }
}

/// A plain navigation link.  Can appear directly inside `Item` (for items
/// without a content panel) or nested inside `Content` (for links within a
/// panel).  When `close_on_click` is `true` (the default), clicking the link
/// closes any open content panel.
#[component]
pub fn Link(
    children: Children,
    #[prop(into)] href: String,
    #[prop(into, optional)] class: String,
    #[prop(default = false)] disabled: bool,
    #[prop(default = true)] close_on_click: bool,
) -> impl IntoView {
    let ctx = use_context::<NavMenuContext>();
    let item_ctx = use_context::<NavMenuItemContext>();
    let in_content = use_context::<InsideNavContent>().is_some();

    // When this link is a top-level nav item (directly inside `Item`, not inside
    // `Content`), attach it to the item's link_ref so Arrow-key navigation can
    // focus it.  Otherwise use a local throwaway ref.
    let is_nav_link = item_ctx.is_some() && !in_content;
    let link_ref = item_ctx
        .filter(|_| is_nav_link)
        .map(|ic| ic.link_ref)
        .unwrap_or_else(NodeRef::new);

    // Update roving-tabindex state when this top-level link receives focus,
    // so Arrow-key navigation knows which item is current.
    if is_nav_link {
        if let (Some(ctx), Some(ic)) = (ctx, item_ctx) {
            let _ = use_event_listener(link_ref, focus, move |_| {
                ctx.set_focus(Some(ic.index));
            });
        }
    }

    view! {
        <a
            node_ref={link_ref}
            href={href}
            class={class}
            tabindex={if disabled { "-1" } else { "0" }}
            aria-disabled={if disabled { Some("true") } else { None }}
            data-disabled={disabled}
            on:click={move |evt| {
                if disabled {
                    evt.prevent_default();
                    evt.stop_propagation();
                    return;
                }
                if close_on_click {
                    if let Some(ctx) = ctx {
                        ctx.close_immediate();
                    }
                }
            }}
        >
            {children()}
        </a>
    }
}
