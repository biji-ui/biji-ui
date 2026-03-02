use leptos::{portal::Portal, prelude::*};

use crate::components::{
    api_table::{PropRow, PropsTable, SectionHeading},
    code::Code,
};

const USAGE_CODE: &str = r#"use leptos::{portal::Portal, prelude::*};
use biji_ui::components::tooltip;

#[component]
pub fn MyTooltip() -> impl IntoView {
    view! {
        <tooltip::Root positioning={tooltip::Positioning::Top}>
            <tooltip::Trigger class="rounded border px-3 py-1.5 text-sm">
                "Hover me"
            </tooltip::Trigger>
            <Portal>
                <tooltip::Content
                    class="rounded-lg bg-gray-900 px-3 py-2 text-sm text-white"
                    show_class="opacity-100"
                    hide_class="opacity-0"
                >
                    <tooltip::Arrow class="border-t border-l border-slate-500" />
                    "Tooltip content"
                </tooltip::Content>
            </Portal>
        </tooltip::Root>
    }
}"#;

const ROOT_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the root wrapper element.",
    },
    PropRow {
        name: "positioning",
        prop_type: "Positioning",
        default: "Top",
        description: "Where to render the tooltip content relative to the trigger. See the Positioning enum below.",
    },
    PropRow {
        name: "hide_delay",
        prop_type: "Duration",
        default: "200ms",
        description: "How long to wait before unmounting the content after the pointer leaves. Should match your CSS transition duration.",
    },
];

const TRIGGER_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the trigger button. The trigger opens the tooltip on hover and on focus.",
}];

const CONTENT_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied in both visible and hidden states.",
    },
    PropRow {
        name: "show_class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied when the tooltip is visible.",
    },
    PropRow {
        name: "hide_class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied while the tooltip is hiding.",
    },
];

const ARROW_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the arrow indicator element. Position and rotation are handled automatically.",
}];

const POSITIONING_PROPS: &[PropRow] = &[
    PropRow { name: "TopStart", prop_type: "Positioning", default: "", description: "Above the trigger, aligned to its left edge." },
    PropRow { name: "Top", prop_type: "Positioning", default: "default", description: "Above the trigger, centered." },
    PropRow { name: "TopEnd", prop_type: "Positioning", default: "", description: "Above the trigger, aligned to its right edge." },
    PropRow { name: "RightStart", prop_type: "Positioning", default: "", description: "To the right of the trigger, aligned to its top edge." },
    PropRow { name: "Right", prop_type: "Positioning", default: "", description: "To the right of the trigger, centered." },
    PropRow { name: "RightEnd", prop_type: "Positioning", default: "", description: "To the right of the trigger, aligned to its bottom edge." },
    PropRow { name: "BottomEnd", prop_type: "Positioning", default: "", description: "Below the trigger, aligned to its right edge." },
    PropRow { name: "Bottom", prop_type: "Positioning", default: "", description: "Below the trigger, centered." },
    PropRow { name: "BottomStart", prop_type: "Positioning", default: "", description: "Below the trigger, aligned to its left edge." },
    PropRow { name: "LeftEnd", prop_type: "Positioning", default: "", description: "To the left of the trigger, aligned to its bottom edge." },
    PropRow { name: "Left", prop_type: "Positioning", default: "", description: "To the left of the trigger, centered." },
    PropRow { name: "LeftStart", prop_type: "Positioning", default: "", description: "To the left of the trigger, aligned to its top edge." },
];

#[component]
pub fn TooltipDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    view! {
        <DocPage title="Tooltip">
            <p class="mt-3 mb-11 text-base text-balance">
                "A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it."
            </p>
            <DocPreview>
                <TooltipExample />
            </DocPreview>
            <SectionHeading title="Usage" />
            <Code
                class="[&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={USAGE_CODE}
                language="rust"
            />
            <SectionHeading title="API Reference" />
            <PropsTable title="Root" rows={ROOT_PROPS} />
            <PropsTable title="Trigger" rows={TRIGGER_PROPS} />
            <PropsTable title="Content" rows={CONTENT_PROPS} />
            <PropsTable title="Arrow" rows={ARROW_PROPS} />
            <PropsTable title="Positioning" rows={POSITIONING_PROPS} />
        </DocPage>
    }
}

#[component]
pub fn Content() -> impl IntoView {
    use biji_ui::components::tooltip;
    view! {
        <tooltip::Content
            class="inline-block w-max rounded-lg border border-slate-500 bg-gray-900 px-3 py-2 text-sm font-medium text-white shadow-sm transition-opacity duration-200 dark:bg-gray-700"
            hide_class="opacity-0"
            show_class="opacity-100"
        >
            <tooltip::Arrow class="rounded-[2px] border-t border-l border-slate-500 border-dark-10"></tooltip::Arrow>
            "Hello,"
            <br />
            "Massive World!"
        </tooltip::Content>
    }
}

pub const BUTTON_BASE_STYLE: &str = "inline-flex h-10 w-full items-center justify-center rounded-md bg-primary px-4 py-2 text-sm font-medium whitespace-nowrap text-primary-foreground ring-offset-background transition-colors focus-visible:outline-none hover:bg-primary/90 focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";

#[component]
pub fn TooltipExample() -> impl IntoView {
    use biji_ui::components::tooltip;
    view! {
        <div class="grid grid-cols-3 gap-2">
            <tooltip::Root positioning={tooltip::Positioning::TopStart}>
                <tooltip::Trigger class={BUTTON_BASE_STYLE}>"Top start"</tooltip::Trigger>
                <Portal>
                    <Content />
                </Portal>
            </tooltip::Root>
            <tooltip::Root positioning={tooltip::Positioning::Top}>
                <tooltip::Trigger class={BUTTON_BASE_STYLE}>"Top"</tooltip::Trigger>
                <Portal>
                    <Content />
                </Portal>
            </tooltip::Root>
            <tooltip::Root positioning={tooltip::Positioning::TopEnd}>
                <tooltip::Trigger class={BUTTON_BASE_STYLE}>"Top end"</tooltip::Trigger>
                <Portal>
                    <Content />
                </Portal>
            </tooltip::Root>
            <tooltip::Root positioning={tooltip::Positioning::RightStart}>
                <tooltip::Trigger class={BUTTON_BASE_STYLE}>"Right start"</tooltip::Trigger>
                <Portal>
                    <Content />
                </Portal>
            </tooltip::Root>
            <tooltip::Root positioning={tooltip::Positioning::Right}>
                <tooltip::Trigger class={BUTTON_BASE_STYLE}>"Right"</tooltip::Trigger>
                <Portal>
                    <Content />
                </Portal>
            </tooltip::Root>
            <tooltip::Root positioning={tooltip::Positioning::RightEnd}>
                <tooltip::Trigger class={BUTTON_BASE_STYLE}>"Right end"</tooltip::Trigger>
                <Portal>
                    <Content />
                </Portal>
            </tooltip::Root>
            <tooltip::Root positioning={tooltip::Positioning::BottomStart}>
                <tooltip::Trigger class={BUTTON_BASE_STYLE}>"Bottom start"</tooltip::Trigger>
                <Portal>
                    <Content />
                </Portal>
            </tooltip::Root>
            <tooltip::Root positioning={tooltip::Positioning::Bottom}>
                <tooltip::Trigger class={BUTTON_BASE_STYLE}>"Bottom"</tooltip::Trigger>
                <Portal>
                    <Content />
                </Portal>
            </tooltip::Root>
            <tooltip::Root positioning={tooltip::Positioning::BottomEnd}>
                <tooltip::Trigger class={BUTTON_BASE_STYLE}>"Bottom end"</tooltip::Trigger>
                <Portal>
                    <Content />
                </Portal>
            </tooltip::Root>
            <tooltip::Root positioning={tooltip::Positioning::LeftStart}>
                <tooltip::Trigger class={BUTTON_BASE_STYLE}>"Left start"</tooltip::Trigger>
                <Portal>
                    <Content />
                </Portal>
            </tooltip::Root>
            <tooltip::Root positioning={tooltip::Positioning::Left}>
                <tooltip::Trigger class={BUTTON_BASE_STYLE}>"Left"</tooltip::Trigger>
                <Portal>
                    <Content />
                </Portal>
            </tooltip::Root>
            <tooltip::Root positioning={tooltip::Positioning::LeftEnd}>
                <tooltip::Trigger class={BUTTON_BASE_STYLE}>"Left end"</tooltip::Trigger>
                <Portal>
                    <Content />
                </Portal>
            </tooltip::Root>
        </div>
    }
}
