use leptos::{html::Div, prelude::*};

/// Reactive state for a slider. Available via [`use_slider`](super::root::use_slider)
/// or the `let:` binding on [`RootWith`](super::root::RootWith).
///
/// All fields are `Copy`, so it is safe to pass this struct to child components as a prop.
#[derive(Copy, Clone)]
pub struct SliderState {
    pub value: RwSignal<f64>,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub disabled: bool,
    /// Position of the thumb as a percentage (0–100), derived from value/min/max.
    pub percentage: Signal<f64>,
    pub(crate) track_ref: NodeRef<Div>,
}

impl SliderState {
    pub(crate) fn new(value: f64, min: f64, max: f64, step: f64, disabled: bool) -> Self {
        let value = RwSignal::new(value.clamp(min, max));
        let percentage = Signal::derive(move || {
            let v = value.get();
            if !v.is_finite() || !min.is_finite() || !max.is_finite() || max <= min {
                return 0.0;
            }
            ((v - min) / (max - min) * 100.0).clamp(0.0, 100.0)
        });
        Self { value, min, max, step, disabled, percentage, track_ref: NodeRef::new() }
    }

    pub fn data_state(&self) -> &'static str {
        if self.disabled { "disabled" } else { "enabled" }
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
}
