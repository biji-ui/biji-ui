use leptos::{portal::Portal, prelude::*};

use crate::components::{
    api_table::{DataAttrRow, DataAttrsTable, PropRow, PropsTable, SectionHeading},
    code::Code,
};

const INSTALL_CODE: &str = concat!(
    "biji-ui = { version = \"",
    env!("CARGO_PKG_VERSION"),
    "\", features = [\"hover_card\"] }",
);

const USAGE_CODE: &str = r#"use leptos::{portal::Portal, prelude::*};
use biji_ui::components::hover_card;

#[component]
pub fn MyHoverCard() -> impl IntoView {
    view! {
        <hover_card::Root>
            <hover_card::Trigger class="inline-block underline decoration-dotted cursor-pointer">
                "@biji_ui"
            </hover_card::Trigger>
            <Portal>
                <hover_card::Content
                    class="z-50 w-64 rounded-lg border shadow-md transition bg-background \
                           border-border origin-[var(--biji-transform-origin)]"
                    show_class="opacity-100 scale-100 duration-150 ease-out"
                    hide_class="opacity-0 scale-95 duration-100 ease-in"
                >
                    <div class="p-4">
                        <p class="text-sm font-semibold">"Biji UI"</p>
                        <p class="text-xs text-muted-foreground mt-1">
                            "Headless UI components for Leptos."
                        </p>
                    </div>
                </hover_card::Content>
            </Portal>
        </hover_card::Root>
    }
}"#;

const ROOT_WITH_CODE: &str = r#"use leptos::{portal::Portal, prelude::*};
use biji_ui::components::hover_card;

#[component]
pub fn MyHoverCard() -> impl IntoView {
    view! {
        <hover_card::RootWith let:h>
            <p class="mb-1 text-sm text-muted-foreground">
                {move || if h.open.get() { "Card is open" } else { "Card is closed" }}
            </p>
            <hover_card::Trigger class="inline-block underline decoration-dotted cursor-pointer">
                "@biji_ui"
            </hover_card::Trigger>
            <Portal>
                <hover_card::Content
                    class="z-50 w-64 rounded-lg border shadow-md transition bg-background \
                           border-border origin-[var(--biji-transform-origin)]"
                    show_class="opacity-100 scale-100 duration-150 ease-out"
                    hide_class="opacity-0 scale-95 duration-100 ease-in"
                >
                    <div class="p-4">
                        <p class="text-sm font-semibold">"Biji UI"</p>
                        <p class="text-xs text-muted-foreground mt-1">
                            "Headless UI components for Leptos."
                        </p>
                    </div>
                </hover_card::Content>
            </Portal>
        </hover_card::RootWith>
    }
}"#;

const ROOT_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the wrapper `<span>`.",
    },
    PropRow {
        name: "positioning",
        prop_type: "Positioning",
        default: "Bottom",
        description: "Where to render the card relative to the trigger.",
    },
    PropRow {
        name: "avoid_collisions",
        prop_type: "AvoidCollisions",
        default: "Flip",
        description: "How the card reacts when it would overflow the viewport.",
    },
    PropRow {
        name: "open_delay",
        prop_type: "Duration",
        default: "700ms",
        description: "How long after the pointer enters the trigger before the card appears.",
    },
    PropRow {
        name: "close_delay",
        prop_type: "Duration",
        default: "300ms",
        description: "How long after the pointer leaves before the card hides.",
    },
    PropRow {
        name: "hide_delay",
        prop_type: "Duration",
        default: "200ms",
        description: "How long to wait before unmounting Content after closing (match your CSS transition).",
    },
    PropRow {
        name: "arrow_size",
        prop_type: "i32",
        default: "8",
        description: "Size in pixels of the optional Arrow element.",
    },
    PropRow {
        name: "open",
        prop_type: "bool",
        default: "false",
        description: "Initial open state.",
    },
    PropRow {
        name: "on_open_change",
        prop_type: "Option<Callback<bool>>",
        default: "None",
        description: "Called with `true` when the card opens and `false` when it closes.",
    },
];

const TRIGGER_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the `<span>` wrapper.",
}];

const CONTENT_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied in both open and closed states.",
    },
    PropRow {
        name: "show_class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied when the card is open.",
    },
    PropRow {
        name: "hide_class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied while the card is closing.",
    },
];

const ARROW_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the arrow `<div>`.",
}];

const DATA_ATTRS: &[DataAttrRow] = &[DataAttrRow {
    name: "data-state",
    description: "\"open\" when the card is visible; \"closed\" otherwise. Present on Trigger.",
}];

#[component]
pub fn HoverCardDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    view! {
        <DocPage title="Hover Card">
            <p class="mt-3 mb-11 text-base text-balance">
                "A card that appears when hovering over a trigger, with configurable open and close delays."
            </p>
            <DocPreview>
                <HoverCardExample />
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
            <p class="mb-5 text-sm text-muted-foreground">
                "Use "
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"RootWith"</code>
                " to access "
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"HoverCardState"</code>
                " inline via the "
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"let:"</code>
                " binding. The state is "
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"Copy"</code>
                " and safe to pass as a prop."
            </p>
            <DocPreview>
                <HoverCardRootWithExample />
            </DocPreview>
            <Code
                class="[&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={ROOT_WITH_CODE}
                language="rust"
            />
            <SectionHeading title="API Reference" />
            <PropsTable title="Root / RootWith" rows={ROOT_PROPS} />
            <PropsTable title="Trigger" rows={TRIGGER_PROPS} />
            <PropsTable title="Content" rows={CONTENT_PROPS} />
            <PropsTable title="Arrow" rows={ARROW_PROPS} />
            <DataAttrsTable rows={DATA_ATTRS} />
        </DocPage>
    }
}

#[component]
pub fn HoverCardRootWithExample() -> impl IntoView {
    use biji_ui::components::hover_card;

    view! {
        <div class="flex flex-col items-center gap-3 p-8">
            <hover_card::RootWith let:h>
                <p class="text-sm text-muted-foreground">
                    {move || if h.open.get() { "Card is open" } else { "Card is closed" }}
                </p>
                <hover_card::Trigger class="inline-block font-medium underline \
                                            decoration-dotted underline-offset-4 \
                                            cursor-default">
                    "@biji_ui"
                </hover_card::Trigger>
                <Portal>
                    <hover_card::Content
                        class="z-50 w-64 rounded-lg border shadow-md transition bg-background \
                               border-border origin-[var(--biji-transform-origin)]"
                        show_class="opacity-100 scale-100 duration-150 ease-out"
                        hide_class="opacity-0 scale-95 duration-100 ease-in"
                    >
                        <div class="p-4">
                            <p class="text-sm font-semibold">"Biji UI"</p>
                            <p class="text-xs text-muted-foreground mt-1">
                                "Headless UI components for Leptos."
                            </p>
                        </div>
                    </hover_card::Content>
                </Portal>
            </hover_card::RootWith>
        </div>
    }
}

#[component]
pub fn HoverCardExample() -> impl IntoView {
    use biji_ui::components::hover_card;

    view! {
        <div class="flex items-center justify-center gap-4 p-16">
            <p class="text-sm text-muted-foreground">
                "Built with "
                <hover_card::Root>
                    <hover_card::Trigger class="inline-block font-medium underline \
                                                decoration-dotted underline-offset-4 \
                                                cursor-default">
                        "@biji_ui"
                    </hover_card::Trigger>
                    <Portal>
                        <hover_card::Content
                            class="z-50 w-72 rounded-lg border shadow-lg transition \
                                   bg-background border-border \
                                   origin-[var(--biji-transform-origin)]"
                            show_class="opacity-100 scale-100 duration-150 ease-out"
                            hide_class="opacity-0 scale-95 duration-100 ease-in"
                        >
                            <div class="p-4 space-y-3">
                                <div class="flex items-center gap-3">
                                    <div class="w-10 h-10 rounded-full bg-primary/10 \
                                                flex items-center justify-center text-primary font-bold text-sm">
                                        "BU"
                                    </div>
                                    <div>
                                        <p class="text-sm font-semibold leading-none">"Biji UI"</p>
                                        <p class="text-xs text-muted-foreground mt-0.5">"@biji_ui"</p>
                                    </div>
                                </div>
                                <p class="text-sm text-muted-foreground">
                                    "Headless, accessible UI components for "
                                    <span class="font-medium text-foreground">"Leptos"</span>
                                    " — fully composable, zero default styles."
                                </p>
                                <div class="flex items-center gap-4 text-xs text-muted-foreground">
                                    <span>
                                        <span class="font-semibold text-foreground">"23"</span>
                                        " components"
                                    </span>
                                    <span>
                                        <span class="font-semibold text-foreground">"MIT"</span>
                                        " license"
                                    </span>
                                </div>
                            </div>
                        </hover_card::Content>
                    </Portal>
                </hover_card::Root>
                "."
            </p>
        </div>
    }
}
