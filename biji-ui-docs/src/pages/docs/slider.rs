use leptos::prelude::*;

use crate::components::{
    api_table::{DataAttrRow, DataAttrsTable, KeyboardRow, KeyboardTable, PropRow, PropsTable, SectionHeading},
    code::Code,
};

const INSTALL_CODE: &str = concat!(
    "biji-ui = { version = \"",
    env!("CARGO_PKG_VERSION"),
    "\", features = [\"slider\"] }",
);

const ROOT_WITH_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::slider;

#[component]
pub fn LabeledSlider() -> impl IntoView {
    let thumb_class = "absolute block w-5 h-5 -translate-x-1/2 rounded-full border-2 border-primary bg-background shadow transition outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:ring-offset-background cursor-grab active:cursor-grabbing";

    view! {
        <slider::RootWith
            default_value=50.0
            min=0.0
            max=100.0
            class="flex flex-col gap-2 w-full"
            let:s
        >
            <div class="flex justify-between items-center text-sm">
                <span>"Volume"</span>
                <span class="tabular-nums font-medium">{move || s.value.get() as u32}</span>
            </div>
            <div class="relative flex items-center w-full h-5 touch-none select-none">
                <slider::Track class="relative h-2 w-full grow overflow-hidden rounded-full bg-secondary">
                    <slider::Range class="absolute h-full bg-primary" />
                </slider::Track>
                <slider::Thumb class={thumb_class} />
            </div>
        </slider::RootWith>
    }
}"#;

const USAGE_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::slider;

#[component]
pub fn MySlider() -> impl IntoView {
    view! {
        <slider::Root
            default_value=50.0
            class="relative flex items-center w-full h-5 touch-none select-none"
        >
            <slider::Track class="relative flex-1 h-2 overflow-hidden rounded-full bg-secondary">
                <slider::Range class="absolute h-full bg-primary" />
            </slider::Track>
            <slider::Thumb class="absolute block w-5 h-5 -translate-x-1/2 rounded-full border-2 border-primary bg-background shadow transition outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:ring-offset-background" />
        </slider::Root>
    }
}"#;

const ROOT_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the slider root element.",
    },
    PropRow {
        name: "value",
        prop_type: "f64",
        default: "0.0",
        description: "The initial value of the slider.",
    },
    PropRow {
        name: "min",
        prop_type: "f64",
        default: "0.0",
        description: "The minimum value of the slider.",
    },
    PropRow {
        name: "max",
        prop_type: "f64",
        default: "100.0",
        description: "The maximum value of the slider.",
    },
    PropRow {
        name: "step",
        prop_type: "f64",
        default: "1.0",
        description: "The step increment for keyboard navigation and snapping.",
    },
    PropRow {
        name: "disabled",
        prop_type: "bool",
        default: "false",
        description: "When true, prevents the slider from being interacted with.",
    },
];

const TRACK_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the track element.",
}];

const RANGE_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the range fill element.",
}];

const THUMB_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the thumb element.",
}];

const DATA_ATTRS: &[DataAttrRow] = &[
    DataAttrRow {
        name: "data-state",
        description: "\"enabled\" when the slider is interactive; \"disabled\" when disabled. Present on Root.",
    },
    DataAttrRow {
        name: "data-disabled",
        description: "Present on Root, Track, Range, and Thumb when the slider is disabled.",
    },
    DataAttrRow {
        name: "data-orientation",
        description: "Always \"horizontal\". Present on Root, Track, and Range.",
    },
];

const KEYBOARD: &[KeyboardRow] = &[
    KeyboardRow {
        key: "ArrowRight / ArrowUp",
        description: "Increases the value by one step.",
    },
    KeyboardRow {
        key: "ArrowLeft / ArrowDown",
        description: "Decreases the value by one step.",
    },
    KeyboardRow {
        key: "Page Up",
        description: "Increases the value by ten steps.",
    },
    KeyboardRow {
        key: "Page Down",
        description: "Decreases the value by ten steps.",
    },
    KeyboardRow {
        key: "Home",
        description: "Sets the value to the minimum.",
    },
    KeyboardRow {
        key: "End",
        description: "Sets the value to the maximum.",
    },
];

#[component]
pub fn SliderDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    view! {
        <DocPage title="Slider">
            <p class="mt-3 mb-11 text-base text-balance">
                "An input where the user selects a value from within a given range."
            </p>
            <DocPreview>
                <SliderExample />
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
            <SectionHeading title="RootWith" />
            <p class="mb-4 text-sm text-muted-foreground">
                "Use "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"<RootWith>"</code>
                " when you need direct access to "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"SliderState"</code>
                " inside the children. The "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"let:s"</code>
                " binding exposes "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"s.value"</code>
                " and "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"s.percentage"</code>
                " as reactive signals for custom rendering."
            </p>
            <DocPreview>
                <SliderRootWithExample />
            </DocPreview>
            <Code
                class="mt-4 [&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={ROOT_WITH_CODE}
                language="rust"
            />
            <SectionHeading title="API Reference" />
            <PropsTable title="Root / RootWith" rows={ROOT_PROPS} />
            <PropsTable title="Track" rows={TRACK_PROPS} />
            <PropsTable title="Range" rows={RANGE_PROPS} />
            <PropsTable title="Thumb" rows={THUMB_PROPS} />
            <DataAttrsTable rows={DATA_ATTRS} />
            <KeyboardTable rows={KEYBOARD} />
        </DocPage>
    }
}

#[component]
pub fn SliderRootWithExample() -> impl IntoView {
    use biji_ui::components::slider;

    let thumb_class = "absolute block h-5 w-5 -translate-x-1/2 rounded-full border-2 border-primary bg-background shadow transition-colors outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:ring-offset-background cursor-grab active:cursor-grabbing";

    view! {
        <div class="w-full max-w-sm">
            <slider::RootWith
                default_value=50.0
                min=0.0
                max=100.0
                class="flex flex-col gap-2 w-full"
                let:s
            >
                <div class="flex justify-between items-center text-sm">
                    <span>"Volume"</span>
                    <span class="tabular-nums font-medium">{move || s.value.get() as u32}</span>
                </div>
                <div class="relative flex items-center w-full h-5 touch-none select-none">
                    <slider::Track class="relative h-2 w-full grow overflow-hidden rounded-full bg-secondary">
                        <slider::Range class="absolute h-full bg-primary" />
                    </slider::Track>
                    <slider::Thumb class={thumb_class} />
                </div>
            </slider::RootWith>
        </div>
    }
}

#[component]
pub fn SliderExample() -> impl IntoView {
    use biji_ui::components::slider;

    let thumb_class = "absolute block h-5 w-5 -translate-x-1/2 rounded-full border-2 border-primary bg-background shadow transition-colors outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:ring-offset-background data-[disabled]:cursor-not-allowed data-[disabled]:border-muted-foreground cursor-grab active:cursor-grabbing";

    view! {
        <div class="flex flex-col gap-8 w-full max-w-sm">
            <div class="flex flex-col gap-2">
                <span class="text-sm font-medium">"Volume"</span>
                <slider::Root
                    default_value=60.0
                    class="relative flex items-center w-full h-5 touch-none select-none"
                >
                    <slider::Track class="relative h-2 w-full grow overflow-hidden rounded-full bg-secondary">
                        <slider::Range class="absolute h-full bg-primary" />
                    </slider::Track>
                    <slider::Thumb class={thumb_class} />
                </slider::Root>
            </div>
            <div class="flex flex-col gap-2">
                <span class="text-sm font-medium">"Temperature (step: 5)"</span>
                <slider::Root
                    default_value=20.0
                    min=0.0
                    max=100.0
                    step=5.0
                    class="relative flex items-center w-full h-5 touch-none select-none"
                >
                    <slider::Track class="relative h-2 w-full grow overflow-hidden rounded-full bg-secondary">
                        <slider::Range class="absolute h-full bg-primary" />
                    </slider::Track>
                    <slider::Thumb class={thumb_class} />
                </slider::Root>
            </div>
            <div class="flex flex-col gap-2">
                <span class="text-sm font-medium text-muted-foreground">"Disabled"</span>
                <slider::Root
                    default_value=40.0
                    disabled=true
                    class="relative flex items-center w-full h-5 touch-none select-none"
                >
                    <slider::Track class="relative h-2 w-full grow overflow-hidden rounded-full bg-secondary opacity-50">
                        <slider::Range class="absolute h-full bg-primary" />
                    </slider::Track>
                    <slider::Thumb class={thumb_class} />
                </slider::Root>
            </div>
        </div>
    }
}
