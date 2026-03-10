use leptos::{
    context::Provider,
    ev::{focus, keydown, paste},
    prelude::*,
};
use leptos_use::use_event_listener;

use super::context::{PinInputContext, next_pin_input_id};

#[component]
pub fn Root(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(default = 4)] length: usize,
    #[prop(default = false)] disabled: bool,
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(optional)] on_complete: Option<Callback<String>>,
    #[prop(optional)] on_change: Option<Callback<String>>,
) -> impl IntoView {
    let cell_refs: Vec<NodeRef<leptos::html::Input>> = (0..length).map(|_| NodeRef::new()).collect();

    let ctx = PinInputContext {
        values: RwSignal::new(vec![String::new(); length]),
        length,
        cell_refs: StoredValue::new(cell_refs),
        disabled,
        placeholder: StoredValue::new(placeholder.unwrap_or_else(|| String::from("○"))),
        root_id: StoredValue::new(next_pin_input_id()),
        on_complete,
        on_change,
    };

    view! {
        <Provider value={ctx}>
            <div
                data-disabled={if disabled { Some("true") } else { None }}
                class={class}
            >
                {children()}
            </div>
        </Provider>
    }
}

#[component]
pub fn Cell(
    #[prop()] index: usize,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let ctx = expect_context::<PinInputContext>();

    // Bounds check
    let cell_ref = ctx.cell_refs.with_value(|refs| {
        refs.get(index).copied()
    });
    let Some(cell_ref) = cell_ref else {
        leptos::logging::warn!("pin_input::Cell index={index} out of bounds (length={})", ctx.length);
        return view! { <input type="text" disabled=true /> }.into_any();
    };

    // Sync DOM value when signal changes (handles external paste updates for non-focused cells)
    let sync_eff = RenderEffect::new(move |_| {
        let val = ctx.values.with(|v| v.get(index).cloned().unwrap_or_default());
        if let Some(el) = cell_ref.get() {
            el.set_value(&val);
        }
    });
    on_cleanup(move || drop(sync_eff));

    // Character input: keep last char only, advance focus
    let _ = use_event_listener(cell_ref, leptos::ev::input, move |evt| {
        let raw = event_target_value(&evt);
        let ch = raw.chars().last().map(|c| c.to_string()).unwrap_or_default();
        // Sync DOM immediately so the display is correct before signal update
        if let Some(el) = cell_ref.get_untracked() {
            el.set_value(&ch);
        }
        ctx.set_cell(index, ch.clone());
        if !ch.is_empty() && index + 1 < ctx.length {
            ctx.focus_cell(index + 1);
        }
    });

    // Backspace: clear current or retreat to previous
    let _ = use_event_listener(cell_ref, keydown, move |evt| {
        match evt.key().as_str() {
            "Backspace" => {
                let current = ctx.values.with(|v| v.get(index).cloned().unwrap_or_default());
                if current.is_empty() && index > 0 {
                    ctx.set_cell(index - 1, String::new());
                    ctx.focus_cell(index - 1);
                } else {
                    ctx.set_cell(index, String::new());
                }
            }
            "ArrowLeft" => {
                evt.prevent_default();
                if index > 0 {
                    ctx.focus_cell(index - 1);
                }
            }
            "ArrowRight" => {
                evt.prevent_default();
                if index + 1 < ctx.length {
                    ctx.focus_cell(index + 1);
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
        // Update signal in one batch
        ctx.values.update(|v| {
            for (offset, ch) in chars.iter().enumerate() {
                let cell_idx = index + offset;
                if cell_idx >= v.len() {
                    break;
                }
                v[cell_idx] = ch.to_string();
                last_filled = cell_idx;
            }
        });
        // Sync DOM values for cells that were filled by this paste
        ctx.cell_refs.with_value(|refs| {
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
        // Fire callbacks
        let full = ctx.current_value();
        if let Some(cb) = ctx.on_change {
            cb.run(full.clone());
        }
        if ctx.is_complete() {
            if let Some(cb) = ctx.on_complete {
                cb.run(full);
            }
        }
        // Focus next unfilled cell or last filled
        let next_focus = (last_filled + 1).min(ctx.length - 1);
        ctx.focus_cell(next_focus);
    });

    // Select all on focus so typing replaces the current char
    let _ = use_event_listener(cell_ref, focus, move |_| {
        if let Some(el) = cell_ref.get_untracked() {
            let _ = el.select();
        }
    });

    let placeholder_str = ctx.placeholder.get_value();
    let is_filled = Memo::new(move |_| {
        ctx.values.with(|v| !v.get(index).map(|s| s.is_empty()).unwrap_or(true))
    });

    let length = ctx.length;
    let cell_id = ctx.root_id.with_value(|id| format!("{id}-{index}"));

    view! {
        <input
            node_ref={cell_ref}
            id={cell_id.clone()}
            name={cell_id}
            type="text"
            inputmode="numeric"
            maxlength="1"
            placeholder={placeholder_str}
            disabled={ctx.disabled}
            autocomplete="one-time-code"
            aria-label={format!("Digit {} of {}", index + 1, length)}
            data-index={index.to_string()}
            data-filled={move || if is_filled.get() { Some("true") } else { None }}
            data-disabled={if ctx.disabled { Some("true") } else { None }}
            class={class}
        />
    }.into_any()
}
