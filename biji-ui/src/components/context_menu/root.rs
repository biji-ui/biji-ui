use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use leptos::{
    context::Provider,
    ev::{focus, keydown, mouseover},
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
    utils::prevent_scroll::use_prevent_scroll,
};

use super::context::{ContextMenuItemContext, ContextMenuState};

static CONTEXT_MENU_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn next_menu_id() -> String {
    let id = CONTEXT_MENU_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("biji-context-menu-{id}")
}

fn build_state(allow_loop: bool, hide_delay: Duration) -> ContextMenuState {
    let open = RwSignal::new(false);
    let data_state = Signal::derive(move || if open.get() { "open" } else { "closed" });
    ContextMenuState {
        trigger_ref: NodeRef::new(),
        content_ref: NodeRef::new(),
        open,
        pointer_x: RwSignal::new(0.0),
        pointer_y: RwSignal::new(0.0),
        data_state,
        item_focus: RwSignal::new(None),
        items: RwSignal::new(Default::default()),
        allow_loop,
        hide_delay,
        menu_id: StoredValue::new(next_menu_id()),
        next_id: StoredValue::new(AtomicUsize::new(0)),
    }
}

pub fn use_context_menu() -> ContextMenuState {
    expect_context::<ContextMenuState>()
}

#[component]
pub fn RootWith<IV: IntoView + 'static>(
    children: impl Fn(ContextMenuState) -> IV + Send + Sync + 'static,
    #[prop(into, optional)] class: String,
    #[prop(default = true)] allow_loop: bool,
    #[prop(default = Duration::from_millis(200))] hide_delay: Duration,
) -> impl IntoView {
    let state = build_state(allow_loop, hide_delay);
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
    #[prop(default = true)] allow_loop: bool,
    #[prop(default = Duration::from_millis(200))] hide_delay: Duration,
) -> impl IntoView {
    view! {
        <RootWith allow_loop={allow_loop} hide_delay={hide_delay} class={class} let:_>
            {children()}
        </RootWith>
    }
}

#[component]
fn RootEvents(children: Children) -> impl IntoView {
    let ctx = expect_context::<ContextMenuState>();

    let ps_eff = use_prevent_scroll(move || ctx.open.get(), ctx.hide_delay);
    on_cleanup(move || drop(ps_eff));

    let _ = use_event_listener(use_document(), keydown, move |evt| {
        if evt.key() == "Escape" && ctx.open.get() {
            ctx.close();
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
    let ctx = expect_context::<ContextMenuState>();

    let _ = use_event_listener(ctx.trigger_ref, leptos::ev::contextmenu, move |evt| {
        evt.prevent_default();
        let x = evt.client_x() as f64;
        let y = evt.client_y() as f64;
        ctx.open_at(x, y);
    });

    view! {
        <div
            node_ref={ctx.trigger_ref}
            data-state={move || if ctx.open.get() { "open" } else { "closed" }}
            class={class}
        >
            {children()}
        </div>
    }
}

#[component]
pub fn Content(
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] show_class: String,
    #[prop(into, optional)] hide_class: String,
) -> impl IntoView {
    let ctx = expect_context::<ContextMenuState>();
    let content_ref = ctx.content_ref;

    let UseElementBoundingReturn {
        width: content_width,
        height: content_height,
        ..
    } = use_element_bounding(content_ref);

    // Position the panel at the pointer coordinates, clamped to the viewport.
    let style_signal = Signal::derive(move || {
        let raw_cw = *content_width.read();
        let raw_ch = *content_height.read();
        let _ = ctx.open.get();

        let hidden = || {
            "position: fixed; top: 0; left: 0; visibility: hidden; --biji-transform-origin: top left;".to_string()
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

        let mut x = ctx.pointer_x.get_untracked();
        let mut y = ctx.pointer_y.get_untracked();

        let vp_w = web_sys::window()
            .and_then(|w| w.inner_width().ok())
            .and_then(|v| v.as_f64())
            .unwrap_or(1920.0);
        let vp_h = web_sys::window()
            .and_then(|w| w.inner_height().ok())
            .and_then(|v| v.as_f64())
            .unwrap_or(1080.0);

        if x + cw > vp_w {
            x = vp_w - cw;
        }
        if y + ch > vp_h {
            y = vp_h - ch;
        }

        format!("position: fixed; top: {y}px; left: {x}px; --biji-transform-origin: top left;")
    });

    // Auto-focus the first item after the panel opens.
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
                    if let Some(item) = ctx.filter_active_items().into_iter().next() {
                        item.focus();
                        ctx.item_focus.set(Some(item.index));
                    }
                },
                Duration::from_millis(10),
            )
            .expect("set_timeout in context_menu focus");
            *focus_handle.lock().unwrap() = Some(h);
        }
    });

    on_cleanup(move || {
        if let Some(h) = focus_handle_cleanup.lock().unwrap().take() {
            h.clear();
        }
        drop(focus_eff);
    });

    let _ = use_event_listener(content_ref, keydown, move |evt| match evt.key().as_str() {
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
                        if let Some(cb) = item.on_select {
                            cb.run(());
                        }
                        ctx.close();
                    }
                }
            }
        }
        "Tab" => {
            ctx.close();
        }
        _ => {}
    });

    view! {
        <CustomAnimatedShow
            when={ctx.open}
            show_class={cn!(class, show_class)}
            hide_class={cn!(class, hide_class)}
            hide_delay={ctx.hide_delay}
            style_signal={style_signal}
            node_ref={content_ref}
            attr:id={ctx.menu_id.get_value()}
            attr:role="menu"
            attr:tabindex="-1"
        >
            {children()}
        </CustomAnimatedShow>
    }
}

#[component]
pub fn Item(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] on_select: Option<Callback<()>>,
) -> impl IntoView {
    let ctx = expect_context::<ContextMenuState>();

    let index = ctx.next_index();
    let item_ctx = ContextMenuItemContext {
        index,
        disabled,
        item_ref: NodeRef::new(),
        on_select,
    };

    ctx.upsert_item(index, item_ctx);

    on_cleanup(move || {
        ctx.remove_item(index);
    });

    let _ = use_event_listener(item_ctx.item_ref, leptos::ev::click, move |_| {
        if item_ctx.disabled {
            return;
        }
        if let Some(cb) = item_ctx.on_select {
            cb.run(());
        }
        ctx.close();
    });

    let _ = use_event_listener(item_ctx.item_ref, mouseover, move |_| {
        if !item_ctx.disabled {
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
            role="menuitem"
            tabindex="-1"
            aria-disabled={if disabled { Some("true") } else { None }}
            data-disabled={disabled}
            data-highlighted={move || ctx.item_in_focus(item_ctx.index)}
            class={class}
        >
            {children()}
        </div>
    }
}

#[component]
pub fn Separator(#[prop(into, optional)] class: String) -> impl IntoView {
    view! { <div role="separator" aria-orientation="horizontal" class={class} /> }
}

#[component]
pub fn Label(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    view! { <div class={class}>{children()}</div> }
}
