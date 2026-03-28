use leptos::{
    context::Provider,
    ev::{focus, keydown, paste},
    prelude::*,
};
use leptos_use::use_event_listener;

use crate::utils::props::StringProp;

use super::context::PinInputState;

/// Returns the [`PinInputState`] from the nearest [`Root`] or [`RootWith`] ancestor.
///
/// Call this inside any child component that needs access to PIN input state.
pub fn use_pin_input() -> PinInputState {
    expect_context::<PinInputState>()
}

/// The render-prop variant of [`Root`]. Use this when you need access to [`PinInputState`]
/// directly inside the children via the `let:` binding.
///
/// ```rust
/// <pin_input::RootWith length=4 class="flex flex-col items-center gap-4" let:s>
///     <div class="flex gap-2">
///         <pin_input::Cell index=0 class="..." />
///         <pin_input::Cell index=1 class="..." />
///         <pin_input::Cell index=2 class="..." />
///         <pin_input::Cell index=3 class="..." />
///     </div>
///     <p class="text-xs text-muted-foreground">
///         {move || if s.is_complete.get() {
///             format!("Code: {}", s.value.get())
///         } else {
///             format!("{}/4 digits entered", s.value.get().len())
///         }}
///     </p>
/// </pin_input::RootWith>
/// ```
///
/// The `s: PinInputState` binding is `Copy`, so it can be passed to child components as a prop.
#[component]
pub fn RootWith<IV: IntoView + 'static>(
    children: impl Fn(PinInputState) -> IV + Send + Sync + 'static,
    #[prop(into, optional)] class: String,
    /// Controlled signal for the per-cell values. When provided, the PIN input reads and writes
    /// this signal directly (the `Vec` must have at least `length` elements).
    #[prop(into, default = None)]
    value: Option<RwSignal<Vec<String>>>,
    #[prop(default = 4)] length: usize,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = StringProp::from("○"))] placeholder: StringProp,
    #[prop(into, default = None)] on_complete: Option<Callback<String>>,
) -> impl IntoView {
    let state = PinInputState::new(value, length, disabled, placeholder, on_complete);

    view! {
        <Provider value={state}>
            <div
                data-disabled={if disabled { Some("true") } else { None }}
                class={class}
            >
                {children(state)}
            </div>
        </Provider>
    }
}

/// The standard PIN input root. Renders a wrapper `<div>` and provides [`PinInputState`]
/// to all descendants via context.
///
/// Use [`RootWith`] instead when you need to access [`PinInputState`] inline via `let:s`.
#[component]
pub fn Root(
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
    #[prop(into, default = None)] value: Option<RwSignal<Vec<String>>>,
    #[prop(default = 4)] length: usize,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = StringProp::from("○"))] placeholder: StringProp,
    #[prop(into, default = None)] on_complete: Option<Callback<String>>,
) -> impl IntoView {
    view! {
        <RootWith
            value=value length=length disabled=disabled placeholder=placeholder
            on_complete=on_complete class=class
            let:_
        >
            {children()}
        </RootWith>
    }
}

#[component]
pub fn Cell(
    #[prop()] index: usize,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let state = expect_context::<PinInputState>();

    let cell_ref = state.cell_refs.with_value(|refs| refs.get(index).copied());
    let Some(cell_ref) = cell_ref else {
        leptos::logging::warn!(
            "pin_input::Cell index={index} out of bounds (length={})",
            state.length
        );
        return view! { <input type="text" disabled=true /> }.into_any();
    };

    // Sync DOM value when signal changes (handles external paste updates for non-focused cells)
    let sync_eff = RenderEffect::new(move |_| {
        let val = state.values.with(|v| v.get(index).cloned().unwrap_or_default());
        if let Some(el) = cell_ref.get() {
            el.set_value(&val);
        }
    });
    on_cleanup(move || drop(sync_eff));

    // Character input: keep last char only, advance focus
    let _ = use_event_listener(cell_ref, leptos::ev::input, move |evt| {
        let raw = event_target_value(&evt);
        let ch = raw.chars().last().map(|c| c.to_string()).unwrap_or_default();
        if let Some(el) = cell_ref.get_untracked() {
            el.set_value(&ch);
        }
        state.set_cell(index, ch.clone());
        if !ch.is_empty() && index + 1 < state.length {
            state.focus_cell(index + 1);
        }
    });

    // Backspace: clear current or retreat to previous
    let _ = use_event_listener(cell_ref, keydown, move |evt| {
        match evt.key().as_str() {
            "Backspace" => {
                let current =
                    state.values.with(|v| v.get(index).cloned().unwrap_or_default());
                if current.is_empty() && index > 0 {
                    state.set_cell(index - 1, String::new());
                    state.focus_cell(index - 1);
                } else {
                    state.set_cell(index, String::new());
                }
            }
            "ArrowLeft" => {
                evt.prevent_default();
                if index > 0 {
                    state.focus_cell(index - 1);
                }
            }
            "ArrowRight" => {
                evt.prevent_default();
                if index + 1 < state.length {
                    state.focus_cell(index + 1);
                }
            }
            _ => {}
        }
    });

    // Paste: distribute chars across cells starting at this index
    let _ = use_event_listener(cell_ref, paste, move |evt| {
        evt.prevent_default();
        let pasted = evt
            .clipboard_data()
            .and_then(|d| d.get_data("text/plain").ok())
            .unwrap_or_default();
        let chars: Vec<char> = pasted.chars().collect();
        let mut last_filled = index;
        state.values.update(|v| {
            for (offset, ch) in chars.iter().enumerate() {
                let cell_idx = index + offset;
                if cell_idx >= v.len() {
                    break;
                }
                v[cell_idx] = ch.to_string();
                last_filled = cell_idx;
            }
        });
        state.cell_refs.with_value(|refs| {
            for (offset, ch) in chars.iter().enumerate() {
                let cell_idx = index + offset;
                if cell_idx >= refs.len() {
                    break;
                }
                if let Some(el) = refs[cell_idx].get_untracked() {
                    el.set_value(&ch.to_string());
                }
            }
        });
        if state.is_complete.get() {
            if let Some(cb) = state.on_complete {
                cb.run(state.value.get());
            }
        }
        let next_focus = (last_filled + 1).min(state.length - 1);
        state.focus_cell(next_focus);
    });

    // Select all on focus so typing replaces the current char
    let _ = use_event_listener(cell_ref, focus, move |_| {
        if let Some(el) = cell_ref.get_untracked() {
            let _ = el.select();
        }
    });

    let is_filled = Memo::new(move |_| {
        state.values.with(|v| !v.get(index).map(|s| s.is_empty()).unwrap_or(true))
    });

    let length = state.length;
    let cell_id = state.root_id.with_value(|id| format!("{id}-{index}"));

    view! {
        <input
            node_ref={cell_ref}
            id={cell_id.clone()}
            name={cell_id}
            type="text"
            inputmode="numeric"
            maxlength="1"
            placeholder={move || state.placeholder.with_value(|p| p.get())}
            disabled={state.disabled}
            autocomplete="one-time-code"
            aria-label={format!("Digit {} of {}", index + 1, length)}
            data-index={index.to_string()}
            data-filled={move || if is_filled.get() { Some("true") } else { None }}
            data-disabled={if state.disabled { Some("true") } else { None }}
            class={class}
        />
    }
    .into_any()
}
