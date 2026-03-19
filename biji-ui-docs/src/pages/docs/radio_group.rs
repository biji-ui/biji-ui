use leptos::prelude::*;

use crate::components::{
    api_table::{DataAttrRow, DataAttrsTable, KeyboardRow, KeyboardTable, PropRow, PropsTable, SectionHeading},
    code::Code,
};

const INSTALL_CODE: &str = concat!(
    "biji-ui = { version = \"",
    env!("CARGO_PKG_VERSION"),
    "\", features = [\"radio_group\"] }",
);

const USAGE_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::radio_group;

#[component]
pub fn MyRadioGroup() -> impl IntoView {
    view! {
        <radio_group::Root class="flex flex-col gap-2">
            <label class="flex items-center gap-2 cursor-pointer">
                <radio_group::Item
                    value="option-a"
                    class="flex items-center justify-center w-5 h-5 rounded-full border-2 border-border data-[state=checked]:border-primary"
                >
                    <radio_group::Indicator class="w-2.5 h-2.5 rounded-full bg-primary hidden data-[state=checked]:block" />
                </radio_group::Item>
                <span class="text-sm">"Option A"</span>
            </label>
            <label class="flex items-center gap-2 cursor-pointer">
                <radio_group::Item
                    value="option-b"
                    class="flex items-center justify-center w-5 h-5 rounded-full border-2 border-border data-[state=checked]:border-primary"
                >
                    <radio_group::Indicator class="w-2.5 h-2.5 rounded-full bg-primary hidden data-[state=checked]:block" />
                </radio_group::Item>
                <span class="text-sm">"Option B"</span>
            </label>
        </radio_group::Root>
    }
}"#;

const ROOT_WITH_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::radio_group;

#[component]
pub fn PlanSelector() -> impl IntoView {
    let item_class = "flex items-center justify-center w-5 h-5 rounded-full border-2 \
        border-border data-[state=checked]:border-primary";
    let indicator_class = "w-2.5 h-2.5 rounded-full bg-primary hidden data-[state=checked]:block";

    view! {
        <radio_group::RootWith default_value="pro".to_string() class="flex flex-col gap-3" let:rg>
            {["Free", "Pro", "Enterprise"].into_iter().map(|label| {
                let value = label.to_lowercase();
                view! {
                    <label class="flex items-center gap-3 cursor-pointer select-none">
                        <radio_group::Item value={value} class={item_class}>
                            <radio_group::Indicator class={indicator_class} />
                        </radio_group::Item>
                        <span class="text-sm font-medium">{label}</span>
                    </label>
                }
            }).collect_view()}
            <p class="text-xs text-muted-foreground mt-1">
                "Selected: " {move || rg.value.get().unwrap_or_default()}
            </p>
        </radio_group::RootWith>
    }
}"#;

const ROOT_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the radio group container.",
    },
    PropRow {
        name: "value",
        prop_type: "Option<String>",
        default: "None",
        description: "The initial selected value of the radio group.",
    },
    PropRow {
        name: "disabled",
        prop_type: "bool",
        default: "false",
        description: "When true, disables all radio items in the group.",
    },
];

const ITEM_PROPS: &[PropRow] = &[
    PropRow {
        name: "value",
        prop_type: "String",
        default: "required",
        description: "The value of this radio item. Used to identify the selected option.",
    },
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the radio item button.",
    },
    PropRow {
        name: "disabled",
        prop_type: "bool",
        default: "false",
        description: "When true, prevents this radio item from being selected.",
    },
];

const INDICATOR_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the indicator span.",
}];

const DATA_ATTRS: &[DataAttrRow] = &[
    DataAttrRow {
        name: "data-state",
        description: "\"checked\" when this item is selected; \"unchecked\" otherwise. Present on Item and Indicator.",
    },
    DataAttrRow {
        name: "data-disabled",
        description: "Present on Root when the group is disabled. Present on Item and Indicator when the item is disabled.",
    },
];

const KEYBOARD: &[KeyboardRow] = &[
    KeyboardRow {
        key: "ArrowDown / ArrowRight",
        description: "Moves focus and selection to the next radio item, wrapping around.",
    },
    KeyboardRow {
        key: "ArrowUp / ArrowLeft",
        description: "Moves focus and selection to the previous radio item, wrapping around.",
    },
];

#[component]
pub fn RadioGroupDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    view! {
        <DocPage title="Radio Group">
            <p class="mt-3 mb-11 text-base text-balance">
                "A set of checkable buttons where only one can be selected at a time."
            </p>
            <DocPreview>
                <RadioGroupExample />
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
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"RadioGroupState"</code>
                " inside the children. The "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"let:rg"</code>
                " binding exposes "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"rg.value"</code>
                " as a reactive signal for reading the current selection without callbacks."
            </p>
            <DocPreview>
                <RadioGroupRootWithExample />
            </DocPreview>
            <Code
                class="mt-4 [&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={ROOT_WITH_CODE}
                language="rust"
            />
            <SectionHeading title="API Reference" />
            <PropsTable title="Root / RootWith" rows={ROOT_PROPS} />
            <PropsTable title="Item" rows={ITEM_PROPS} />
            <PropsTable title="Indicator" rows={INDICATOR_PROPS} />
            <DataAttrsTable rows={DATA_ATTRS} />
            <KeyboardTable rows={KEYBOARD} />
        </DocPage>
    }
}

#[component]
pub fn RadioGroupRootWithExample() -> impl IntoView {
    use biji_ui::components::radio_group;

    let item_class = "flex items-center justify-center w-5 h-5 rounded-full border-2 border-border \
        transition-colors outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 \
        focus:ring-offset-background data-[state=checked]:border-primary \
        data-[disabled]:cursor-not-allowed data-[disabled]:opacity-50";
    let indicator_class = "block w-2.5 h-2.5 rounded-full bg-primary hidden data-[state=checked]:block";

    view! {
        <radio_group::RootWith default_value="pro".to_string() class="flex flex-col gap-3" let:rg>
            {["Free", "Pro", "Enterprise"].into_iter().map(|label| {
                let value = label.to_lowercase();
                view! {
                    <label class="flex items-center gap-3 cursor-pointer select-none">
                        <radio_group::Item value={value} class={item_class}>
                            <radio_group::Indicator class={indicator_class} />
                        </radio_group::Item>
                        <span class="text-sm font-medium">{label}</span>
                    </label>
                }
            }).collect_view()}
            <p class="text-xs text-muted-foreground mt-1">
                "Selected: " {move || rg.value.get().unwrap_or_default()}
            </p>
        </radio_group::RootWith>
    }
}

#[component]
pub fn RadioGroupExample() -> impl IntoView {
    use biji_ui::components::radio_group;

    let item_class = "flex items-center justify-center w-5 h-5 rounded-full border-2 border-border transition-colors outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:ring-offset-background data-[state=checked]:border-primary data-[disabled]:cursor-not-allowed data-[disabled]:opacity-50";
    let indicator_class = "block w-2.5 h-2.5 rounded-full bg-primary hidden data-[state=checked]:block";

    view! {
        <radio_group::Root
            default_value="comfortable".to_string()
            class="flex flex-col gap-3"
        >
            {["Default", "Comfortable", "Compact"]
                .into_iter()
                .map(|label| {
                    let value = label.to_lowercase();
                    view! {
                        <label class="flex items-center gap-3 cursor-pointer select-none">
                            <radio_group::Item
                                value={value}
                                class={item_class}
                            >
                                <radio_group::Indicator class={indicator_class} />
                            </radio_group::Item>
                            <span class="text-sm font-medium">{label}</span>
                        </label>
                    }
                })
                .collect_view()}
        </radio_group::Root>
    }
}
