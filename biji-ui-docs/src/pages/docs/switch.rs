use leptos::prelude::*;

use crate::components::{
    api_table::{DataAttrRow, DataAttrsTable, KeyboardRow, KeyboardTable, PropRow, PropsTable, SectionHeading},
    code::Code,
};

const INSTALL_CODE: &str = concat!(
    "biji-ui = { version = \"",
    env!("CARGO_PKG_VERSION"),
    "\", features = [\"switch\"] }",
);

const USAGE_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::switch;

#[component]
pub fn MySwitch() -> impl IntoView {
    view! {
        <switch::Root class="relative inline-flex w-11 h-6 rounded-full border-2 border-transparent transition-colors bg-zinc-300 dark:bg-zinc-600 data-[state=checked]:bg-primary">
            <switch::Thumb class="block w-5 h-5 rounded-full bg-white data-[state=checked]:bg-primary-foreground shadow-md transition-transform data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0" />
        </switch::Root>
    }
}"#;

const ROOT_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the switch button.",
    },
    PropRow {
        name: "checked",
        prop_type: "bool",
        default: "false",
        description: "The initial checked (on) state of the switch.",
    },
    PropRow {
        name: "disabled",
        prop_type: "bool",
        default: "false",
        description: "When true, prevents the switch from being toggled.",
    },
    PropRow {
        name: "on_checked_change",
        prop_type: "Option<Callback<bool>>",
        default: "None",
        description: "Callback fired when the switch state changes. Receives true when on, false when off.",
    },
];

const THUMB_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the thumb span.",
}];

const DATA_ATTRS: &[DataAttrRow] = &[
    DataAttrRow {
        name: "data-state",
        description: "\"checked\" when on; \"unchecked\" when off. Present on Root and Thumb.",
    },
    DataAttrRow {
        name: "data-disabled",
        description: "Present on Root and Thumb when the switch is disabled.",
    },
];

const KEYBOARD: &[KeyboardRow] = &[KeyboardRow {
    key: "Space",
    description: "Toggles the switch between on and off when focused.",
}];

#[component]
pub fn SwitchDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    view! {
        <DocPage title="Switch">
            <p class="mt-3 mb-11 text-base text-balance">
                "A control that allows the user to toggle between on and off states."
            </p>
            <DocPreview>
                <SwitchExample />
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
            <SectionHeading title="API Reference" />
            <PropsTable title="Root" rows={ROOT_PROPS} />
            <PropsTable title="Thumb" rows={THUMB_PROPS} />
            <DataAttrsTable rows={DATA_ATTRS} />
            <KeyboardTable rows={KEYBOARD} />
        </DocPage>
    }
}

#[component]
pub fn SwitchExample() -> impl IntoView {
    use biji_ui::components::switch;

    view! {
        <div class="flex flex-col gap-6">
            <label class="flex items-center gap-3 cursor-pointer select-none">
                <switch::Root class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:ring-offset-background bg-zinc-300 dark:bg-zinc-600 data-[state=checked]:bg-primary data-[disabled]:cursor-not-allowed data-[disabled]:opacity-50">
                    <switch::Thumb class="pointer-events-none block h-5 w-5 rounded-full bg-white data-[state=checked]:bg-primary-foreground shadow-md ring-0 transition-transform data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0" />
                </switch::Root>
                <span class="text-sm font-medium">"Airplane mode"</span>
            </label>
            <label class="flex items-center gap-3 cursor-pointer select-none">
                <switch::Root
                    checked=true
                    class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:ring-offset-background bg-zinc-300 dark:bg-zinc-600 data-[state=checked]:bg-primary data-[disabled]:cursor-not-allowed data-[disabled]:opacity-50"
                >
                    <switch::Thumb class="pointer-events-none block h-5 w-5 rounded-full bg-white data-[state=checked]:bg-primary-foreground shadow-md ring-0 transition-transform data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0" />
                </switch::Root>
                <span class="text-sm font-medium">"Push notifications"</span>
            </label>
            <label class="flex items-center gap-3 cursor-not-allowed select-none">
                <switch::Root
                    disabled=true
                    class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:ring-offset-background bg-zinc-300 dark:bg-zinc-600 data-[state=checked]:bg-primary data-[disabled]:cursor-not-allowed data-[disabled]:opacity-50"
                >
                    <switch::Thumb class="pointer-events-none block h-5 w-5 rounded-full bg-white data-[state=checked]:bg-primary-foreground shadow-md ring-0 transition-transform data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0" />
                </switch::Root>
                <span class="text-sm font-medium text-muted-foreground">"Disabled option"</span>
            </label>
        </div>
    }
}
