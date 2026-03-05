use leptos::prelude::*;

use crate::components::{
    api_table::{DataAttrRow, DataAttrsTable, PropRow, PropsTable, SectionHeading},
    code::Code,
};

const INSTALL_CODE: &str = concat!(
    "biji-ui = { version = \"",
    env!("CARGO_PKG_VERSION"),
    "\", features = [\"separator\"] }",
);

const USAGE_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::separator;

#[component]
pub fn MyComponent() -> impl IntoView {
    view! {
        <div>
            <p>"Above"</p>
            <separator::Root class="my-4 h-px bg-border" />
            <p>"Below"</p>
        </div>
    }
}"#;

const ROOT_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the separator element.",
    },
    PropRow {
        name: "orientation",
        prop_type: "Orientation",
        default: "Horizontal",
        description: "The orientation of the separator. Use Horizontal for a horizontal line, Vertical for a vertical line.",
    },
    PropRow {
        name: "decorative",
        prop_type: "bool",
        default: "true",
        description: "When true, the separator is purely visual and hidden from assistive technologies (role=\"none\"). When false, it is semantic (role=\"separator\").",
    },
];

const DATA_ATTRS: &[DataAttrRow] = &[DataAttrRow {
    name: "data-orientation",
    description: "\"horizontal\" or \"vertical\", matching the orientation prop.",
}];

#[component]
pub fn SeparatorDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    view! {
        <DocPage title="Separator">
            <p class="mt-3 mb-11 text-base text-balance">
                "Visually or semantically separates content."
            </p>
            <DocPreview>
                <SeparatorExample />
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
            <DataAttrsTable rows={DATA_ATTRS} />
        </DocPage>
    }
}

#[component]
pub fn SeparatorExample() -> impl IntoView {
    use biji_ui::components::separator;

    view! {
        <div class="w-full max-w-sm space-y-4 text-sm">
            <div class="space-y-1">
                <h4 class="font-medium leading-none">"Biji UI"</h4>
                <p class="text-muted-foreground">"Headless components for Leptos."</p>
            </div>
            <separator::Root class="bg-border h-px w-full" />
            <div class="flex items-center gap-4">
                <span>"Blog"</span>
                <separator::Root
                    class="bg-border w-px h-4"
                    orientation={separator::Orientation::Vertical}
                />
                <span>"Docs"</span>
                <separator::Root
                    class="bg-border w-px h-4"
                    orientation={separator::Orientation::Vertical}
                />
                <span>"Source"</span>
            </div>
        </div>
    }
}
