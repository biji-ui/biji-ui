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
            class="relative h-2 w-full overflow-hidden rounded-full bg-secondary"
            value=75.0
            max=100.0
        >
            <progress::Indicator
                class="h-full w-full flex-1 bg-primary transition-all"
                style="transform: translateX(-25%)"
            />
        </progress::Root>
    }
}"#;

const REACTIVE_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::progress;

#[component]
pub fn ReactiveProgress() -> impl IntoView {
    let value = RwSignal::new(50.0_f64);
    let steps: &[f64] = &[0.0, 25.0, 50.0, 75.0, 100.0];

    view! {
        <div class="space-y-6">
            <progress::Root
                class="relative h-3 w-full overflow-hidden rounded-full bg-secondary"
                max=100.0
            >
                <div
                    class="h-full bg-primary transition-all duration-500 ease-in-out"
                    style={move || format!("width: {}%", value.get())}
                />
            </progress::Root>
            <div class="flex gap-2 justify-between">
                {steps
                    .iter()
                    .map(|&s| view! {
                        <button
                            class="rounded-md px-3 py-1.5 text-sm border border-border hover:bg-muted transition-colors data-[active]:bg-muted data-[active]:font-medium outline-none focus:ring-2 focus:ring-ring"
                            data-active={move || value.get() == s}
                            on:click={move |_| value.set(s)}
                        >
                            {format!("{}%", s as u32)}
                        </button>
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
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
        prop_type: "Option<f64>",
        default: "None",
        description: "The current progress value. When None, the progress is indeterminate.",
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
    description: "CSS class applied to the indicator element. Use CSS transform to control the fill width.",
}];

const DATA_ATTRS: &[DataAttrRow] = &[
    DataAttrRow {
        name: "data-state",
        description: "\"indeterminate\" when value is None; \"loading\" while in progress; \"complete\" when value equals max. Present on Root and Indicator.",
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
                <code class="rounded bg-muted px-1 py-0.5 font-mono text-xs">"<div>"</code>
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
            <SectionHeading title="API Reference" />
            <PropsTable title="Root" rows={ROOT_PROPS} />
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
                    class="relative h-3 w-full overflow-hidden rounded-full bg-secondary"
                    value=25.0
                    max=100.0
                >
                    <progress::Indicator
                        class="h-full w-full flex-1 bg-primary transition-all"
                        style="transform: translateX(-75%)"
                    />
                </progress::Root>
            </div>
            <div class="space-y-1">
                <p class="text-xs text-muted-foreground">"60%"</p>
                <progress::Root
                    class="relative h-3 w-full overflow-hidden rounded-full bg-secondary"
                    value=60.0
                    max=100.0
                >
                    <progress::Indicator
                        class="h-full w-full flex-1 bg-primary transition-all"
                        style="transform: translateX(-40%)"
                    />
                </progress::Root>
            </div>
            <div class="space-y-1">
                <p class="text-xs text-muted-foreground">"Complete"</p>
                <progress::Root
                    class="relative h-3 w-full overflow-hidden rounded-full bg-secondary"
                    value=100.0
                    max=100.0
                >
                    <progress::Indicator
                        class="h-full w-full flex-1 bg-primary transition-all"
                        style="transform: translateX(0%)"
                    />
                </progress::Root>
            </div>
        </div>
    }
}

#[component]
pub fn ReactiveProgressExample() -> impl IntoView {
    use biji_ui::components::progress;

    let value = RwSignal::new(50.0_f64);
    let steps: &[f64] = &[0.0, 25.0, 50.0, 75.0, 100.0];

    view! {
        <div class="w-full sm:max-w-[70%] space-y-6">
            <progress::Root
                class="relative h-3 w-full overflow-hidden rounded-full bg-secondary"
                max=100.0
            >
                <div
                    class="h-full bg-primary transition-all duration-500 ease-in-out"
                    style={move || format!("width: {}%", value.get())}
                />
            </progress::Root>
            <div class="flex gap-2 justify-between">
                {steps
                    .iter()
                    .map(|&s| {
                        view! {
                            <button
                                class="rounded-md px-3 py-1.5 text-sm border border-border hover:bg-muted transition-colors data-[active]:bg-muted data-[active]:font-medium outline-none focus:ring-2 focus:ring-ring"
                                data-active={move || value.get() == s}
                                on:click={move |_| value.set(s)}
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
