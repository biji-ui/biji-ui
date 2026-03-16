use std::sync::{Arc, Mutex};
use std::time::Duration;

use leptos::{
    html::Div,
    leptos_dom::helpers::TimeoutHandle,
    portal::Portal,
    prelude::*,
};
use leptos_use::use_event_listener;

use super::context::{PauseOnHover, ToastItem, ToastPosition, ToasterContext};

/// Returns the current time in milliseconds (from `performance.now()`).
/// Returns 0.0 on SSR — timers are no-ops there anyway.
fn now_ms() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now())
            .unwrap_or(0.0)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0.0
    }
}

/// Place `Toaster` once near your app root. It provides [`ToasterContext`] to all
/// descendants, and renders the toast stack in a fixed-position portal overlay.
///
/// Access the context anywhere in the tree:
/// ```rust
/// let toaster = expect_context::<ToasterContext>();
/// toaster.add("Saved!", None, Some("success".to_string()), None);
/// ```
#[component]
pub fn Toaster(
    /// App content. The context is available to all children.
    children: Children,
    /// How long each toast stays visible before auto-dismissing. Default: 4 s.
    #[prop(optional)]
    default_duration: Option<Duration>,
    /// Duration of the exit animation (time between `data-dismissed="true"` and DOM removal).
    /// Default: 300 ms.
    #[prop(optional)]
    hide_delay: Option<Duration>,
    /// Viewport anchor. Default: `BottomRight`.
    #[prop(default = ToastPosition::BottomRight)]
    position: ToastPosition,
    /// Maximum number of toasts shown simultaneously. Default: 5.
    #[prop(default = 5usize)]
    max_toasts: usize,
    /// Controls which toasts pause when hovered. Default: `Single`.
    #[prop(default = PauseOnHover::Single)]
    pause_on_hover: PauseOnHover,
    /// When `true`, renders a countdown progress bar inside each toast.
    /// Style it with `progress_class`.
    #[prop(default = false)]
    show_progress: bool,
    /// CSS class applied to the progress bar element. The bar's `width` is
    /// controlled by an inline `transition` style — use height, color, etc. here.
    #[prop(into, optional)]
    progress_class: String,
    /// Extra CSS class applied to the toast-stack container div.
    #[prop(into, optional)]
    class: String,
    /// CSS class applied to every individual toast `<div>`.
    /// Use `data-[type=…]:…`, `data-[dismissed=true]:…`, `data-[entering=true]:…`,
    /// `data-[paused=true]:…` arbitrary-variant selectors for styling.
    #[prop(into, optional)]
    toast_class: String,
) -> impl IntoView {
    let default_duration = default_duration.unwrap_or(Duration::from_millis(4000));
    let hide_delay = hide_delay.unwrap_or(Duration::from_millis(300));

    let ctx = ToasterContext::new(default_duration, hide_delay, pause_on_hover);
    provide_context(ctx);

    // Auto-dismiss toasts that overflow max_toasts so every toast always gets
    // a dismissal scheduled, even those not rendered in the visible window.
    Effect::new(move |_| {
        let v = ctx.toasts.get();
        if v.len() > max_toasts {
            let overflow = v.len() - max_toasts;
            for toast in &v[..overflow] {
                ctx.dismiss(toast.id);
            }
        }
    });

    let stored_toast_class = StoredValue::new(toast_class);
    let stored_progress_class = StoredValue::new(progress_class);

    let position_style = match position {
        ToastPosition::TopLeft => "position:fixed;top:1rem;left:1rem;z-index:9999;",
        ToastPosition::TopCenter => {
            "position:fixed;top:1rem;left:50%;transform:translateX(-50%);z-index:9999;"
        }
        ToastPosition::TopRight => "position:fixed;top:1rem;right:1rem;z-index:9999;",
        ToastPosition::BottomLeft => "position:fixed;bottom:1rem;left:1rem;z-index:9999;",
        ToastPosition::BottomCenter => {
            "position:fixed;bottom:1rem;left:50%;transform:translateX(-50%);z-index:9999;"
        }
        ToastPosition::BottomRight => "position:fixed;bottom:1rem;right:1rem;z-index:9999;",
    };

    // Bottom positions: use flex-col so the last (newest) item sits at the bottom
    // nearest the anchor. Top positions: use flex-col-reverse so newest is at top.
    let is_bottom = matches!(
        position,
        ToastPosition::BottomLeft | ToastPosition::BottomCenter | ToastPosition::BottomRight
    );
    let stack_dir = if is_bottom { "flex-col" } else { "flex-col-reverse" };
    let container_class = StoredValue::new(format!(
        "flex gap-2 w-[356px] max-w-[calc(100vw-2rem)] {stack_dir} {class}"
    ));

    view! {
        {children()}
        <Portal>
            <div
                style={position_style}
                class={move || container_class.get_value()}
                aria-live="polite"
                aria-atomic="false"
            >
                <For
                    each=move || ctx.toasts.get()
                    key=|t| t.id
                    children=move |toast| {
                        let toast_class = stored_toast_class.get_value();
                        let progress_class = stored_progress_class.get_value();
                        view! {
                            <ToastItemView
                                toast=toast
                                ctx=ctx
                                toast_class=toast_class
                                show_progress=show_progress
                                progress_class=progress_class
                            />
                        }
                    }
                />
            </div>
        </Portal>
    }
}

/// Tracks pause state and accumulated elapsed time for a single toast.
#[derive(Copy, Clone)]
struct Timing {
    is_paused: bool,
    /// Total ms elapsed in all previous runs (before the current unpaused run).
    elapsed_ms: f64,
}

#[component]
fn ToastItemView(
    toast: ToastItem,
    ctx: ToasterContext,
    #[prop(into, optional)] toast_class: String,
    #[prop(default = false)] show_progress: bool,
    #[prop(into, optional)] progress_class: String,
) -> impl IntoView {
    let id = toast.id;
    let total_ms = toast.duration.as_millis() as f64;
    let type_str = StoredValue::new(toast.toast_type);
    let pause_mode = ctx.pause_on_hover.get_value();

    // Combined pause state + accumulated elapsed time (updated atomically on pause/resume).
    let timing = RwSignal::new(Timing { is_paused: false, elapsed_ms: 0.0 });

    // JS timestamp when the current run started (non-reactive, updated on each resume).
    let run_start_ms = StoredValue::new(now_ms());

    // Whether THIS toast is currently being hovered.
    let toast_hovered = RwSignal::new(false);

    // Cancelable dismiss timeout handle.
    let dismiss_handle: StoredValue<Arc<Mutex<Option<TimeoutHandle>>>> =
        StoredValue::new(Arc::new(Mutex::new(None)));

    // Schedule the initial dismiss timeout.
    {
        let h = leptos::leptos_dom::helpers::set_timeout_with_handle(
            move || ctx.dismiss(id),
            toast.duration,
        )
        .expect("set_timeout");
        dismiss_handle.with_value(|arc| {
            *arc.lock().unwrap() = Some(h);
        });
    }

    // ── Pause logic ──────────────────────────────────────────────────────────
    let do_pause = move || {
        // Add elapsed time from current run before recording the pause.
        let elapsed = timing.get_untracked().elapsed_ms + (now_ms() - run_start_ms.get_value());
        timing.set(Timing { is_paused: true, elapsed_ms: elapsed });
        // Cancel in-flight dismiss timeout.
        dismiss_handle.with_value(|arc| {
            if let Some(h) = arc.lock().unwrap().take() {
                h.clear();
            }
        });
    };

    let do_resume = move || {
        let elapsed = timing.get_untracked().elapsed_ms;
        let remaining = (total_ms - elapsed).max(0.0);
        run_start_ms.set_value(now_ms());
        timing.update(|t| t.is_paused = false);
        if remaining <= 0.0 {
            ctx.dismiss(id);
            return;
        }
        let h = leptos::leptos_dom::helpers::set_timeout_with_handle(
            move || ctx.dismiss(id),
            Duration::from_millis(remaining as u64),
        )
        .expect("set_timeout");
        dismiss_handle.with_value(|arc| {
            *arc.lock().unwrap() = Some(h);
        });
    };

    // Watch hover signals and call pause/resume on transitions.
    let pause_eff = RenderEffect::new(move |prev_paused: Option<bool>| {
        let should_pause = match pause_mode {
            PauseOnHover::Single => toast_hovered.get(),
            PauseOnHover::All => ctx.hover_count.get() > 0,
            PauseOnHover::Disable => false,
        };
        match (prev_paused.unwrap_or(false), should_pause) {
            (false, true) => do_pause(),
            (true, false) => do_resume(),
            _ => {}
        }
        should_pause
    });

    // ── Hover event wiring ───────────────────────────────────────────────────
    let toast_ref = NodeRef::<Div>::new();

    let _ = use_event_listener(toast_ref, leptos::ev::pointerenter, move |_| {
        if !matches!(pause_mode, PauseOnHover::Disable) {
            toast_hovered.set(true);
        }
        if matches!(pause_mode, PauseOnHover::All) {
            ctx.hover_count.update(|c| *c += 1);
        }
    });

    let _ = use_event_listener(toast_ref, leptos::ev::pointerleave, move |_| {
        if !matches!(pause_mode, PauseOnHover::Disable) {
            toast_hovered.set(false);
        }
        if matches!(pause_mode, PauseOnHover::All) {
            ctx.hover_count.update(|c| *c = c.saturating_sub(1));
        }
    });

    // ── Cleanup ──────────────────────────────────────────────────────────────
    on_cleanup(move || {
        // Cancel any pending dismiss timeout.
        dismiss_handle.with_value(|arc| {
            if let Some(h) = arc.lock().unwrap().take() {
                h.clear();
            }
        });
        // In "All" mode, decrement hover_count if this toast was hovered on unmount.
        if matches!(pause_mode, PauseOnHover::All) && toast_hovered.get_untracked() {
            ctx.hover_count.update(|c| *c = c.saturating_sub(1));
        }
        drop(pause_eff);
    });

    // ── Progress bar CSS transition ──────────────────────────────────────────
    // Enter animation flips after 1 ms — this also kicks off the progress transition.
    let entering = RwSignal::new(true);
    set_timeout(move || entering.set(false), Duration::from_millis(1));

    // Strategy: when `entering` flips we set "width:0%;transition:{total}ms linear"
    // so CSS animates from the current 100% (set at mount) to 0%.
    // On pause we snap to the computed current %, then on resume continue to 0%
    // over the remaining duration. All smooth, no rAF needed.
    let progress_style = Signal::derive(move || {
        if total_ms <= 0.0 {
            return "width:0%;transition:width 0ms;".to_string();
        }
        let t = timing.get();
        if entering.get() {
            // Initial mount state: full bar, no transition.
            "width:100%;transition:width 0ms;".to_string()
        } else if t.is_paused {
            let pct = ((1.0 - t.elapsed_ms / total_ms) * 100.0).max(0.0).min(100.0);
            format!("width:{pct:.2}%;transition:width 0ms;")
        } else {
            let remaining = (total_ms - t.elapsed_ms).max(0.0);
            format!("width:0%;transition:width {remaining:.0}ms linear;")
        }
    });

    // ── Render ───────────────────────────────────────────────────────────────
    let is_dismissed = Signal::derive(move || ctx.dismissed.with(|s| s.contains(&id)));

    let title = toast.title.clone();
    let description = toast.description.clone();
    let stored_progress_class = StoredValue::new(progress_class);

    view! {
        <div
            node_ref=toast_ref
            class={toast_class}
            role="status"
            data-type={move || type_str.get_value()}
            data-entering={move || entering.get().to_string()}
            data-dismissed={move || is_dismissed.get().to_string()}
            data-paused={move || timing.get().is_paused.to_string()}
        >
            <div style="flex:1 1 auto;min-width:0;">
                <p>{title}</p>
                {description.map(|d| view! { <p>{d}</p> })}
            </div>
            <button
                type="button"
                on:click=move |_| ctx.dismiss(id)
                aria-label="Dismiss"
            >
                {"×"}
            </button>
            {show_progress.then(|| view! {
                <div
                    class={move || stored_progress_class.get_value()}
                    style={progress_style}
                />
            })}
        </div>
    }
}
