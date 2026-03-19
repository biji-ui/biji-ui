use leptos::{html::Div, prelude::*};

/// Read-only state for a slider. Available via [`use_slider`](super::root::use_slider)
/// or the `let:` binding on [`RootWith`](super::root::RootWith).
///
/// All fields are `Copy`, so it is safe to pass this struct to child components as a prop.
#[derive(Copy, Clone)]
pub struct SliderState {
    /// The current slider value as a read-only signal.
    pub value: Signal<f64>,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub disabled: bool,
    /// Position of the thumb as a percentage (0–100), derived from value/min/max.
    pub percentage: Signal<f64>,
}

impl SliderState {
    pub(crate) fn new(ctx: SliderContext) -> Self {
        let percentage = Signal::derive(move || {
            let value = ctx.value.get();
            if !value.is_finite()
                || !ctx.min.is_finite()
                || !ctx.max.is_finite()
                || ctx.max <= ctx.min
            {
                return 0.0;
            }
            ((value - ctx.min) / (ctx.max - ctx.min) * 100.0).clamp(0.0, 100.0)
        });
        Self {
            value: ctx.value.into(),
            min: ctx.min,
            max: ctx.max,
            step: ctx.step,
            disabled: ctx.disabled,
            percentage,
        }
    }
}

#[derive(Copy, Clone)]
pub struct SliderContext {
    pub value: RwSignal<f64>,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub disabled: bool,
    pub track_ref: NodeRef<Div>,
}

impl SliderContext {
    pub fn percentage(&self) -> f64 {
        let value = self.value.get();
        if !value.is_finite()
            || !self.min.is_finite()
            || !self.max.is_finite()
            || self.max <= self.min
        {
            return 0.0;
        }
        ((value - self.min) / (self.max - self.min) * 100.0).clamp(0.0, 100.0)
    }

    pub fn set_value_from_pct(&self, pct: f64) {
        if !pct.is_finite() {
            return;
        }
        let raw = self.min + pct.clamp(0.0, 1.0) * (self.max - self.min);
        let stepped = if self.step.is_finite() && self.step > 0.0 {
            ((raw - self.min) / self.step).round() * self.step + self.min
        } else {
            raw
        };
        self.value.set(stepped.clamp(self.min, self.max));
    }

    pub fn data_state(&self) -> &'static str {
        if self.disabled { "disabled" } else { "enabled" }
    }
}
