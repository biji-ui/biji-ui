use leptos::{context::Provider, prelude::*};

#[derive(Copy, Clone)]
pub struct ProgressContext {
    pub value: Option<f64>,
    pub max: f64,
}

impl ProgressContext {
    pub fn data_state(&self) -> &'static str {
        match self.value {
            None => "indeterminate",
            Some(v) if v >= self.max => "complete",
            _ => "loading",
        }
    }

    pub fn percentage(&self) -> Option<f64> {
        if self.max <= 0.0 {
            return None;
        }
        self.value.map(|v| (v / self.max * 100.0).clamp(0.0, 100.0))
    }
}

#[component]
pub fn Root(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(optional)] value: Option<f64>,
    #[prop(default = 100.0)] max: f64,
) -> impl IntoView {
    let ctx = ProgressContext { value, max };

    view! {
        <Provider value={ctx}>
            <div
                role="progressbar"
                aria-valuemin="0"
                aria-valuemax={max.to_string()}
                aria-valuenow={value.map(|v| v.to_string())}
                data-state={ctx.data_state()}
                data-value={value.map(|v| v.to_string())}
                data-max={max.to_string()}
                class={class}
            >
                {children()}
            </div>
        </Provider>
    }
}

#[component]
pub fn Indicator(
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] style: String,
) -> impl IntoView {
    let ctx = expect_context::<ProgressContext>();

    view! {
        <div
            data-state={ctx.data_state()}
            data-value={ctx.value.map(|v| v.to_string())}
            data-max={ctx.max.to_string()}
            class={class}
            style={style}
        />
    }
}
