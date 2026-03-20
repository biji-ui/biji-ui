use leptos::{context::Provider, prelude::*};

/// Reactive state for a progress bar. Available via [`use_progress`] or the `let:` binding on [`RootWith`].
///
/// All fields are `Copy`, so it is safe to pass this struct to child components as a prop.
#[derive(Copy, Clone)]
pub struct ProgressState {
    /// The raw progress value. `None` means indeterminate.
    pub value: Signal<Option<f64>>,
    /// The maximum value (denominator).
    pub max: f64,
    /// Clamped percentage (0–100). `None` when value is `None` or `max <= 0`.
    pub percentage: Signal<Option<f64>>,
    /// `"indeterminate"` | `"loading"` | `"complete"`.
    pub data_state: Signal<&'static str>,
}

impl ProgressState {
    fn new(value: Signal<Option<f64>>, max: f64) -> Self {
        let percentage = Signal::derive(move || {
            if max <= 0.0 {
                return None;
            }
            value.get().map(|v| (v / max * 100.0).clamp(0.0, 100.0))
        });
        let data_state = Signal::derive(move || match value.get() {
            None => "indeterminate",
            Some(v) if v >= max => "complete",
            _ => "loading",
        });
        Self {
            value,
            max,
            percentage,
            data_state,
        }
    }
}

/// Returns the [`ProgressState`] from the nearest [`Root`] or [`RootWith`] ancestor.
///
/// Call this inside any child component that needs access to progress state.
pub fn use_progress() -> ProgressState {
    expect_context::<ProgressState>()
}

/// The render-prop variant of [`Root`]. Use this when you need access to [`ProgressState`]
/// directly inside the children via the `let:` binding.
///
/// ```rust
/// <progress::RootWith value=value max=100.0 let:p>
///     <progress::Indicator class="h-full bg-primary" />
///     <span>{move || format!("{}%", p.percentage.get().unwrap_or(0.0) as u32)}</span>
/// </progress::RootWith>
/// ```
///
/// The `p: ProgressState` binding is `Copy`, so it can be passed to child components as a prop.
#[component]
pub fn RootWith<IV: IntoView + 'static>(
    children: impl Fn(ProgressState) -> IV + Send + Sync + 'static,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] value: Signal<Option<f64>>,
    #[prop(default = 100.0)] max: f64,
) -> impl IntoView {
    let state = ProgressState::new(value, max);

    view! {
        <Provider value={state}>
            <div
                role="progressbar"
                aria-valuemin="0"
                aria-valuemax={max.to_string()}
                aria-valuenow={move || state.value.get().map(|v| v.to_string())}
                data-state={move || state.data_state.get()}
                data-value={move || state.value.get().map(|v| v.to_string())}
                data-max={max.to_string()}
                class={class}
            >
                {children(state)}
            </div>
        </Provider>
    }
}

/// The standard progress root. Renders a `<div role="progressbar">` with ARIA attributes
/// and provides [`ProgressState`] to all descendants via context.
///
/// Use [`RootWith`] instead when you need to access [`ProgressState`] inline via `let:p`.
#[component]
pub fn Root(
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] value: Signal<Option<f64>>,
    #[prop(default = 100.0)] max: f64,
) -> impl IntoView {
    view! {
        <RootWith value=value max=max class=class let:_>
            {children()}
        </RootWith>
    }
}

/// The progress fill element. Automatically sets its `width` from [`ProgressState`] in context.
///
/// Must be rendered inside a [`Root`] or [`RootWith`].
#[component]
pub fn Indicator(
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let state = expect_context::<ProgressState>();

    view! {
        <div
            data-state={move || state.data_state.get()}
            data-value={move || state.value.get().map(|v| v.to_string())}
            data-max={state.max.to_string()}
            class={class}
            style=move || format!("width: {}%", state.percentage.get().unwrap_or(0.0))
        />
    }
}
