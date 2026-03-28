use std::time::Duration;

use leptos::prelude::*;

use crate::components::{
    api_table::{
        DataAttrRow, DataAttrsTable, KeyboardRow, KeyboardTable, PropRow, PropsTable,
        SectionHeading,
    },
    code::Code,
};

const INSTALL_CODE: &str = concat!(
    "biji-ui = { version = \"",
    env!("CARGO_PKG_VERSION"),
    "\", features = [\"collapsible\"] }",
);

const USAGE_CODE: &str = r#"use std::time::Duration;
use leptos::prelude::*;
use biji_ui::components::collapsible;

#[component]
pub fn MyCollapsible() -> impl IntoView {
    view! {
        <collapsible::Root class="w-full">
            <collapsible::Trigger class="flex justify-between items-center py-3 w-full text-sm font-medium">
                "Toggle content"
            </collapsible::Trigger>
            <collapsible::Content
                class="text-sm"
                show_class="opacity-100 transition duration-150 ease-in"
                hide_class="opacity-0 transition duration-200 ease-out"
                hide_delay={Duration::from_millis(200)}
            >
                "Hidden content revealed on toggle."
            </collapsible::Content>
        </collapsible::Root>
    }
}"#;

const ROOT_WITH_CODE: &str = r#"use std::time::Duration;
use leptos::prelude::*;
use biji_ui::components::collapsible;

#[component]
pub fn MyCollapsible() -> impl IntoView {
    view! {
        <collapsible::RootWith class="w-full sm:max-w-[70%] space-y-2" let:c>
            <div class="flex justify-between items-center">
                <span class="text-sm font-semibold">"Starred repositories"</span>
                <div class="flex items-center gap-2">
                    <span class="text-xs text-muted-foreground">
                        {move || if c.open.get() { "Hide" } else { "Show" }}
                    </span>
                    <collapsible::Trigger class="flex items-center justify-center w-9 h-9 rounded-lg hover:bg-muted transition-colors outline-none focus:ring-2 focus:ring-ring data-[state=open]:[&>svg]:rotate-180">
                        // chevron icon
                    </collapsible::Trigger>
                </div>
            </div>
            <collapsible::Content
                show_class="opacity-100 transition duration-150 ease-in"
                hide_class="opacity-0 transition duration-200 ease-out"
                hide_delay={Duration::from_millis(200)}
            >
                // items
            </collapsible::Content>
        </collapsible::RootWith>
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
        name: "open",
        prop_type: "bool",
        default: "false",
        description: "The initial open state of the collapsible.",
    },
    PropRow {
        name: "disabled",
        prop_type: "bool",
        default: "false",
        description: "When true, prevents the collapsible from being toggled.",
    },
];

const TRIGGER_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the trigger button.",
}];

const CONTENT_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the content wrapper.",
    },
    PropRow {
        name: "show_class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied when the content is visible.",
    },
    PropRow {
        name: "hide_class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied while the content is hiding (during the transition out).",
    },
    PropRow {
        name: "hide_delay",
        prop_type: "Duration",
        default: "200ms",
        description: "How long to wait before unmounting the content after closing begins. Should match your CSS transition duration.",
    },
];

const DATA_ATTRS: &[DataAttrRow] = &[
    DataAttrRow {
        name: "data-state",
        description: "\"open\" when expanded; \"closed\" when collapsed. Present on Root and Trigger.",
    },
    DataAttrRow {
        name: "data-disabled",
        description: "Present on Root and Trigger when the collapsible is disabled.",
    },
];

const KEYBOARD: &[KeyboardRow] = &[KeyboardRow {
    key: "Enter / Space",
    description: "Toggles the collapsible open or closed when the trigger is focused.",
}];

#[component]
pub fn CollapsibleDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    view! {
        <DocPage title="Collapsible">
            <p class="mt-3 mb-11 text-base text-balance">
                "An interactive component that can be expanded or collapsed to show or hide content."
            </p>
            <DocPreview>
                <CollapsibleExample />
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
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"CollapsibleState"</code>
                " inside the children. The "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"let:c"</code>
                " binding exposes "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"c.open"</code>
                " as a reactive signal for driving UI that depends on the open state — without CSS data-attribute selectors."
            </p>
            <DocPreview>
                <CollapsibleRootWithExample />
            </DocPreview>
            <Code
                class="mt-4 [&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={ROOT_WITH_CODE}
                language="rust"
            />
            <SectionHeading title="API Reference" />
            <PropsTable title="Root / RootWith" rows={ROOT_PROPS} />
            <PropsTable title="Trigger" rows={TRIGGER_PROPS} />
            <PropsTable title="Content" rows={CONTENT_PROPS} />
            <DataAttrsTable rows={DATA_ATTRS} />
            <KeyboardTable rows={KEYBOARD} />
        </DocPage>
    }
}

#[component]
pub fn CollapsibleRootWithExample() -> impl IntoView {
    use crate::icons;
    use biji_ui::components::collapsible;

    view! {
        <collapsible::RootWith class="w-full sm:max-w-[70%] space-y-2" let:c>
            <div class="flex justify-between items-center">
                <span class="text-sm font-semibold">"Starred repositories"</span>
                <div class="flex items-center gap-3">
                    <span class="text-xs text-muted-foreground">
                        {move || if c.open.get() { "Hide" } else { "Show" }}
                    </span>
                    <collapsible::Trigger class="flex items-center justify-center w-9 h-9 rounded-lg hover:bg-muted transition-colors outline-none focus:ring-2 focus:ring-ring data-[state=open]:[&>svg]:rotate-180">
                        <icons::Caret class="transition-transform duration-200 size-4" />
                    </collapsible::Trigger>
                </div>
            </div>
            <div class="py-3 px-4 font-mono text-sm rounded-lg border border-border">
                "leptos-rs/leptos"
            </div>
            <collapsible::Content
                show_class="opacity-100 transition duration-150 ease-in"
                hide_class="opacity-0 transition duration-200 ease-out"
                hide_delay={Duration::from_millis(200)}
            >
                <div class="space-y-2">
                    <div class="py-3 px-4 font-mono text-sm rounded-lg border border-border">
                        "Synphonyte/leptos-use"
                    </div>
                    <div class="py-3 px-4 font-mono text-sm rounded-lg border border-border">
                        "tokio-rs/axum"
                    </div>
                </div>
            </collapsible::Content>
        </collapsible::RootWith>
    }
}

#[component]
pub fn CollapsibleExample() -> impl IntoView {
    use crate::icons;
    use biji_ui::components::collapsible;

    view! {
        <collapsible::Root class="w-full sm:max-w-[70%] space-y-2">
            <div class="flex justify-between items-center">
                <span class="text-sm font-semibold">"Starred repositories"</span>
                <collapsible::Trigger class="flex items-center justify-center w-9 h-9 rounded-lg hover:bg-muted transition-colors outline-none focus:ring-2 focus:ring-ring data-[state=open]:[&>svg]:rotate-180">
                    <icons::Caret class="transition-transform duration-200 size-4" />
                </collapsible::Trigger>
            </div>
            <div class="py-3 px-4 font-mono text-sm rounded-lg border border-border">
                "leptos-rs/leptos"
            </div>
            <collapsible::Content
                show_class="opacity-100 transition duration-150 ease-in"
                hide_class="opacity-0 transition duration-200 ease-out"
                hide_delay={Duration::from_millis(200)}
            >
                <div class="space-y-2">
                    <div class="py-3 px-4 font-mono text-sm rounded-lg border border-border">
                        "Synphonyte/leptos-use"
                    </div>
                    <div class="py-3 px-4 font-mono text-sm rounded-lg border border-border">
                        "tokio-rs/axum"
                    </div>
                </div>
            </collapsible::Content>
        </collapsible::Root>
    }
}
