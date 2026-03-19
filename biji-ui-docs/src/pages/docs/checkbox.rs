use leptos::prelude::*;

use crate::components::{
    api_table::{DataAttrRow, DataAttrsTable, KeyboardRow, KeyboardTable, PropRow, PropsTable, SectionHeading},
    code::Code,
};

const INSTALL_CODE: &str = concat!(
    "biji-ui = { version = \"",
    env!("CARGO_PKG_VERSION"),
    "\", features = [\"checkbox\"] }",
);

const USAGE_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::checkbox;

#[component]
pub fn MyCheckbox() -> impl IntoView {
    view! {
        <checkbox::Root class="flex items-center justify-center w-5 h-5 rounded border border-border data-[state=checked]:bg-primary data-[state=checked]:border-primary">
            <checkbox::Indicator class="hidden data-[state=checked]:block text-primary-foreground">
                // Checkmark icon
            </checkbox::Indicator>
        </checkbox::Root>
    }
}"#;

const ROOT_WITH_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::checkbox;

#[component]
pub fn LabeledCheckbox() -> impl IntoView {
    let root_class = "flex items-center justify-center w-5 h-5 rounded border-2 border-border \
        transition-colors data-[state=checked]:bg-primary data-[state=checked]:border-primary";
    let indicator_class = "hidden data-[state=checked]:flex text-primary-foreground";

    view! {
        <label class="flex items-center gap-3 cursor-pointer select-none">
            <checkbox::RootWith class={root_class} let:cb>
                <checkbox::Indicator class={indicator_class}>
                    // Checkmark icon
                </checkbox::Indicator>
                <span class="sr-only">{move || cb.data_state.get()}</span>
            </checkbox::RootWith>
            <span class="text-sm font-medium">
                {move || /* use_checkbox() or pass cb as prop */ "Accept terms"}
            </span>
        </label>
    }
}"#;

const ROOT_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the checkbox button.",
    },
    PropRow {
        name: "checked",
        prop_type: "bool",
        default: "false",
        description: "The initial checked state of the checkbox.",
    },
    PropRow {
        name: "indeterminate",
        prop_type: "bool",
        default: "false",
        description: "When true, the checkbox starts in an indeterminate state (takes precedence over checked).",
    },
    PropRow {
        name: "disabled",
        prop_type: "bool",
        default: "false",
        description: "When true, prevents the checkbox from being toggled.",
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
        description: "\"checked\", \"unchecked\", or \"indeterminate\". Present on Root and Indicator.",
    },
    DataAttrRow {
        name: "data-disabled",
        description: "Present on Root and Indicator when the checkbox is disabled.",
    },
];

const KEYBOARD: &[KeyboardRow] = &[KeyboardRow {
    key: "Space",
    description: "Toggles the checkbox between checked and unchecked when focused.",
}];

#[component]
pub fn CheckboxDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    view! {
        <DocPage title="Checkbox">
            <p class="mt-3 mb-11 text-base text-balance">
                "A control that allows the user to toggle between checked, unchecked, and optionally indeterminate states."
            </p>
            <DocPreview>
                <CheckboxExample />
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
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"CheckboxState"</code>
                " inside the children. The "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"let:cb"</code>
                " binding exposes "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"cb.checked"</code>
                " and "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"cb.data_state"</code>
                " as reactive signals for custom rendering."
            </p>
            <DocPreview>
                <CheckboxRootWithExample />
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
            <KeyboardTable rows={KEYBOARD} />
        </DocPage>
    }
}

const CHECKMARK: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg>"#;

#[component]
pub fn CheckboxRootWithExample() -> impl IntoView {
    use biji_ui::components::checkbox;

    let root_class = "flex items-center justify-center w-5 h-5 rounded border-2 border-border \
        transition-colors outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 \
        focus:ring-offset-background data-[state=checked]:bg-primary data-[state=checked]:border-primary";
    let indicator_class = "hidden data-[state=checked]:flex text-primary-foreground";

    view! {
        <div class="flex items-center gap-3">
            <checkbox::RootWith class={root_class} let:cb>
                <checkbox::Indicator class={indicator_class}>
                    <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
                        <polyline points="20 6 9 17 4 12"></polyline>
                    </svg>
                </checkbox::Indicator>
                <span class="sr-only">{move || cb.data_state.get()}</span>
            </checkbox::RootWith>
            <span class="text-sm font-medium select-none">
                "Subscribe to newsletter"
            </span>
        </div>
    }
}

#[component]
pub fn CheckboxExample() -> impl IntoView {
    use biji_ui::components::checkbox;

    let root_class = "flex items-center justify-center w-5 h-5 rounded border-2 border-border \
        transition-colors outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 \
        focus:ring-offset-background data-[state=checked]:bg-primary data-[state=checked]:border-primary \
        data-[disabled]:opacity-50 data-[disabled]:cursor-not-allowed";
    let indicator_class = "hidden data-[state=checked]:flex text-primary-foreground";

    view! {
        <div class="flex flex-col gap-4">
            <label class="flex items-center gap-3 cursor-pointer select-none">
                <checkbox::Root class={root_class}>
                    <checkbox::Indicator class={indicator_class}>
                        <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="20 6 9 17 4 12"></polyline>
                        </svg>
                    </checkbox::Indicator>
                </checkbox::Root>
                <span class="text-sm font-medium">"Accept terms and conditions"</span>
            </label>
            <label class="flex items-center gap-3 cursor-pointer select-none">
                <checkbox::Root checked=true class={root_class}>
                    <checkbox::Indicator class={indicator_class}>
                        <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="20 6 9 17 4 12"></polyline>
                        </svg>
                    </checkbox::Indicator>
                </checkbox::Root>
                <span class="text-sm font-medium">"Subscribe to newsletter"</span>
            </label>
            <label class="flex items-center gap-3 cursor-not-allowed select-none">
                <checkbox::Root disabled=true class={root_class}>
                    <checkbox::Indicator class={indicator_class}>
                        <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="20 6 9 17 4 12"></polyline>
                        </svg>
                    </checkbox::Indicator>
                </checkbox::Root>
                <span class="text-sm font-medium text-muted-foreground">"Disabled option"</span>
            </label>
        </div>
    }
}
