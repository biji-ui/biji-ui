use std::time::Duration;

use leptos::{context::Provider, ev::click, prelude::*};
use leptos_use::use_event_listener;

use crate::{cn, custom_animated_show::CustomAnimatedShow};

use super::context::CollapsibleState;

/// Returns the [`CollapsibleState`] from the nearest [`Root`] or [`RootWith`] ancestor.
///
/// Call this inside any child component that needs access to collapsible state.
pub fn use_collapsible() -> CollapsibleState {
    expect_context::<CollapsibleState>()
}

/// The render-prop variant of [`Root`]. Use this when you need access to [`CollapsibleState`]
/// directly inside the children via the `let:` binding.
///
/// ```rust
/// <collapsible::RootWith class="w-full" let:c>
///     <div class="flex justify-between items-center">
///         <span class="text-sm font-semibold">"Starred repositories"</span>
///         <div class="flex items-center gap-2">
///             <span class="text-xs text-muted-foreground">
///                 {move || if c.open.get() { "Hide" } else { "Show" }}
///             </span>
///             <collapsible::Trigger class="..."><icons::Caret /></collapsible::Trigger>
///         </div>
///     </div>
///     <collapsible::Content ...>/* items */</collapsible::Content>
/// </collapsible::RootWith>
/// ```
///
/// The `c: CollapsibleState` binding is `Copy`, so it can be passed to child components as a prop.
#[component]
pub fn RootWith<IV: IntoView + 'static>(
    children: impl Fn(CollapsibleState) -> IV + Send + Sync + 'static,
    #[prop(into, optional)] class: String,
    #[prop(default = false)] open: bool,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let state = CollapsibleState::new(open, disabled);

    view! {
        <Provider value={state}>
            <div
                class={class}
                data-state={move || state.data_state.get()}
                data-disabled={state.disabled}
            >
                {children(state)}
            </div>
        </Provider>
    }
}

/// The standard collapsible root. Renders a `<div>` with data attributes and provides
/// [`CollapsibleState`] to all descendants via context.
///
/// Use [`RootWith`] instead when you need to access [`CollapsibleState`] inline via `let:c`.
#[component]
pub fn Root(
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
    #[prop(default = false)] open: bool,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    view! {
        <RootWith open=open disabled=disabled class=class let:_>
            {children()}
        </RootWith>
    }
}

#[component]
pub fn Trigger(
    children: Children,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let state = expect_context::<CollapsibleState>();

    let _ = use_event_listener(state.trigger_ref, click, move |_| {
        state.toggle();
    });

    view! {
        <button
            node_ref={state.trigger_ref}
            class={class}
            aria-expanded={move || if state.open.get() { "true" } else { "false" }}
            aria-disabled={if state.disabled { Some("true") } else { None }}
            data-state={move || state.data_state.get()}
            data-disabled={state.disabled}
        >
            {children()}
        </button>
    }
}

#[component]
pub fn Content(
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] show_class: String,
    #[prop(into, optional)] hide_class: String,
    #[prop(default = Duration::from_millis(200))] hide_delay: Duration,
) -> impl IntoView {
    let state = expect_context::<CollapsibleState>();

    view! {
        <CustomAnimatedShow
            when={state.open}
            show_class={cn!(class, show_class)}
            hide_class={cn!(class, hide_class)}
            hide_delay={hide_delay}
        >
            {children()}
        </CustomAnimatedShow>
    }
}
