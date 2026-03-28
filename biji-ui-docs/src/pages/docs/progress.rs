use leptos::prelude::*;

use crate::components::{
    api_table::{DataAttrRow, DataAttrsTable, PropRow, PropsTable, SectionHeading},
    code::Code,
};

const INSTALL_CODE: &str = concat!(
    "biji-ui = { version = \"",
    env!("CARGO_PKG_VERSION"),
    "\", features = [\"progress\"] }",
);

const USAGE_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::progress;

#[component]
pub fn MyProgress() -> impl IntoView {
    view! {
        <progress::Root
            class="overflow-hidden relative w-full h-2 rounded-full bg-secondary"
            value=Some(75.0_f64)
            max=100.0
        >
            <progress::Indicator class="h-full transition-all bg-primary" />
        </progress::Root>
    }
}"#;

const REACTIVE_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::progress;

#[component]
pub fn ReactiveProgress() -> impl IntoView {
    let value = RwSignal::new(Some(50.0_f64));
    let steps: &[f64] = &[0.0, 25.0, 50.0, 75.0, 100.0];

    view! {
        <div class="space-y-6">
            <progress::Root
                class="overflow-hidden relative w-full h-3 rounded-full bg-secondary"
                value=value
                max=100.0
            >
                <progress::Indicator class="h-full transition-all duration-500 ease-in-out bg-primary" />
            </progress::Root>
            <div class="flex gap-2 justify-between">
                {steps
                    .iter()
                    .map(|&s| view! {
                        <button
                            class="py-1.5 px-3 text-sm rounded-md border transition-colors outline-none focus:ring-2 border-border data-[active]:bg-muted data-[active]:font-medium hover:bg-muted focus:ring-ring"
                            data-active={move || value.get() == Some(s)}
                            on:click={move |_| value.set(Some(s))}
                        >
                            {format!("{}%", s as u32)}
                        </button>
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
}"#;

const ROOT_WITH_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::progress;

#[component]
pub fn LabeledProgress() -> impl IntoView {
    let value = RwSignal::new(Some(50.0_f64));

    view! {
        <progress::RootWith
            class="w-full"
            value=value
            max=100.0
            let:p
        >
            <div class="flex justify-between items-center mb-1.5 text-xs">
                <span class="text-muted-foreground">"Progress"</span>
                <span class="font-medium tabular-nums">
                    {move || {
                        p.percentage.get()
                            .map(|pct| format!("{}%", pct as u32))
                            .unwrap_or_else(|| "–".to_string())
                    }}
                </span>
            </div>
            <div class="overflow-hidden w-full h-3 rounded-full bg-secondary">
                <progress::Indicator class="h-full transition-all duration-500 ease-in-out bg-primary" />
            </div>
        </progress::RootWith>
    }
}"#;

const ROOT_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the root element.",
    },
    PropRow {
        name: "value",
        prop_type: "Signal<Option<f64>>",
        default: "None",
        description: "The current progress value. Accepts a static Some(f64) or a reactive signal. When None, the progress is indeterminate.",
    },
    PropRow {
        name: "max",
        prop_type: "f64",
        default: "100.0",
        description: "The maximum progress value.",
    },
];

const INDICATOR_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the indicator element.",
}];

const DATA_ATTRS: &[DataAttrRow] = &[
    DataAttrRow {
        name: "data-state",
        description: "\"indeterminate\" when value is None; \"loading\" while in progress; \"complete\" when value is greater than or equal to max. Present on Root and Indicator.",
    },
    DataAttrRow {
        name: "data-value",
        description: "The current numeric value. Present on Root and Indicator when value is set.",
    },
    DataAttrRow {
        name: "data-max",
        description: "The maximum value. Present on Root and Indicator.",
    },
];

#[component]
pub fn ProgressDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    view! {
        <DocPage title="Progress">
            <p class="mt-3 mb-11 text-base text-balance">
                "Displays an indicator showing the completion progress of a task."
            </p>
            <DocPreview>
                <ProgressExample />
            </DocPreview>
            <SectionHeading title="Installation" />
            <Code
                class="[&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={INSTALL_CODE}
                language="toml"
            />
            <SectionHeading title="Usage" />
            <Code
                class="[&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={USAGE_CODE}
                language="rust"
            />
            <SectionHeading title="Example" />
            <p class="mb-4 text-sm text-muted-foreground">
                "Drive the progress bar reactively with a signal. Place a reactive "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"<div>"</code>
                " inside the root as the indicator and update its width via an inline style closure."
            </p>
            <DocPreview>
                <ReactiveProgressExample />
            </DocPreview>
            <Code
                class="mt-4 [&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={REACTIVE_CODE}
                language="rust"
            />
            <SectionHeading title="RootWith" />
            <p class="mb-4 text-sm text-muted-foreground">
                "Use "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"<RootWith>"</code>
                " when you need direct access to "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"ProgressState"</code>
                " inside the children. The "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"let:p"</code>
                " binding exposes derived signals like "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"p.percentage"</code>
                " and "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"p.data_state"</code>
                " for custom rendering."
            </p>
            <DocPreview>
                <RootWithExample />
            </DocPreview>
            <Code
                class="mt-4 [&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={ROOT_WITH_CODE}
                language="rust"
            />
            <SectionHeading title="API Reference" />
            <PropsTable title="Root / RootWith" rows={ROOT_PROPS} />
            <PropsTable title="Indicator" rows={INDICATOR_PROPS} />
            <DataAttrsTable rows={DATA_ATTRS} />
        </DocPage>
    }
}

#[component]
pub fn ProgressExample() -> impl IntoView {
    use biji_ui::components::progress;

    view! {
        <div class="w-full sm:max-w-[70%] space-y-4">
            <div class="space-y-1">
                <p class="text-xs text-muted-foreground">"25%"</p>
                <progress::Root
                    class="overflow-hidden relative w-full h-3 rounded-full bg-secondary"
                    value={Some(25.0_f64)}
                    max=100.0
                >
                    <progress::Indicator class="h-full transition-all bg-primary" />
                </progress::Root>
            </div>
            <div class="space-y-1">
                <p class="text-xs text-muted-foreground">"60%"</p>
                <progress::Root
                    class="overflow-hidden relative w-full h-3 rounded-full bg-secondary"
                    value={Some(60.0_f64)}
                    max=100.0
                >
                    <progress::Indicator class="h-full transition-all bg-primary" />
                </progress::Root>
            </div>
            <div class="space-y-1">
                <p class="text-xs text-muted-foreground">"Complete"</p>
                <progress::Root
                    class="overflow-hidden relative w-full h-3 rounded-full bg-secondary"
                    value={Some(100.0_f64)}
                    max=100.0
                >
                    <progress::Indicator class="h-full transition-all bg-primary" />
                </progress::Root>
            </div>
        </div>
    }
}

#[component]
pub fn RootWithExample() -> impl IntoView {
    use biji_ui::components::progress;

    let value = RwSignal::new(Some(50.0_f64));
    let steps: &[f64] = &[0.0, 25.0, 50.0, 75.0, 100.0];

    view! {
        <div class="w-full sm:max-w-[70%] space-y-6">
            <progress::RootWith
                class="w-full"
                value={value}
                max=100.0
                let:p
            >
                <div class="flex justify-between items-center mb-1.5 text-xs">
                    <span class="text-muted-foreground">"Progress"</span>
                    <span class="font-medium tabular-nums">
                        {move || {
                            p.percentage.get()
                                .map(|pct| format!("{}%", pct as u32))
                                .unwrap_or_else(|| "–".to_string())
                        }}
                    </span>
                </div>
                <div class="overflow-hidden w-full h-3 rounded-full bg-secondary">
                    <progress::Indicator class="h-full transition-all duration-500 ease-in-out bg-primary" />
                </div>
            </progress::RootWith>
            <div class="flex gap-2 justify-between">
                {steps
                    .iter()
                    .map(|&s| {
                        view! {
                            <button
                                class="py-1.5 px-3 text-sm rounded-md border transition-colors outline-none focus:ring-2 border-border data-[active]:bg-muted data-[active]:font-medium hover:bg-muted focus:ring-ring"
                                data-active={move || value.get() == Some(s)}
                                on:click={move |_| value.set(Some(s))}
                            >
                                {format!("{}%", s as u32)}
                            </button>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[component]
pub fn ReactiveProgressExample() -> impl IntoView {
    use biji_ui::components::progress;

    let value = RwSignal::new(Some(50.0_f64));
    let steps: &[f64] = &[0.0, 25.0, 50.0, 75.0, 100.0];

    view! {
        <div class="w-full sm:max-w-[70%] space-y-6">
            <progress::Root
                class="overflow-hidden relative w-full h-3 rounded-full bg-secondary"
                value={value}
                max=100.0
            >
                <progress::Indicator class="h-full transition-all duration-500 ease-in-out bg-primary" />
            </progress::Root>
            <div class="flex gap-2 justify-between">
                {steps
                    .iter()
                    .map(|&s| {
                        view! {
                            <button
                                class="py-1.5 px-3 text-sm rounded-md border transition-colors outline-none focus:ring-2 border-border data-[active]:bg-muted data-[active]:font-medium hover:bg-muted focus:ring-ring"
                                data-active={move || value.get() == Some(s)}
                                on:click={move |_| value.set(Some(s))}
                            >
                                {format!("{}%", s as u32)}
                            </button>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
}
