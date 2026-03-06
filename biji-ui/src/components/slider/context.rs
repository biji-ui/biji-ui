use leptos::{html::Div, prelude::*};

#[derive(Copy, Clone)]
pub struct SliderContext {
    pub value: RwSignal<f64>,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub disabled: bool,
    pub track_ref: NodeRef<Div>,
    pub(crate) on_value_change: Option<Callback<f64>>,
}

impl SliderContext {
    pub fn percentage(&self) -> f64 {
        let value = self.value.get();
        let min = self.min;
        let max = self.max;
        if !value.is_finite() || !min.is_finite() || !max.is_finite() || max <= min {
            return 0.0;
        }
        ((value - min) / (max - min) * 100.0).clamp(0.0, 100.0)
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
