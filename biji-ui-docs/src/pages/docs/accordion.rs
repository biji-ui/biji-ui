use std::time::Duration;

use leptos::prelude::*;

use crate::{
    components::{
        api_table::{DataAttrRow, DataAttrsTable, KeyboardRow, KeyboardTable, PropRow, PropsTable, SectionHeading},
        code::Code,
    },
    icons,
};

const USAGE_CODE: &str = r#"use std::time::Duration;
use leptos::prelude::*;
use biji_ui::components::accordion;

#[component]
pub fn MyAccordion() -> impl IntoView {
    let items = [
        ("What is biji-ui?", "A headless UI component library for Leptos."),
        ("Is it accessible?", "Yes, it follows WAI-ARIA patterns."),
    ];

    view! {
        <accordion::Root class="w-full" allow_loop=true>
            {items.into_iter().map(|(title, content)| view! {
                <accordion::Item class="border-b border-border">
                    <accordion::Toggle class="flex w-full items-center justify-between py-5 text-sm font-medium outline-none">
                        {title}
                    </accordion::Toggle>
                    <accordion::Content
                        class="pb-4 text-sm"
                        show_class="opacity-100 transition duration-150 ease-in"
                        hide_class="opacity-0 transition duration-200 ease-out"
                        hide_delay={Duration::from_millis(200)}
                    >
                        {content}
                    </accordion::Content>
                </accordion::Item>
            }).collect::<Vec<_>>()}
        </accordion::Root>
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
        name: "allow_loop",
        prop_type: "bool",
        default: "false",
        description: "When true, keyboard navigation wraps from the last item back to the first and vice versa.",
    },
];

const ITEM_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the item wrapper element.",
    },
    PropRow {
        name: "disabled",
        prop_type: "bool",
        default: "false",
        description: "When true, prevents the item from being opened or receiving focus.",
    },
];

const TOGGLE_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the toggle button.",
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
        description: "\"open\" when the item is expanded; \"closed\" when collapsed. Present on Item and Toggle.",
    },
    DataAttrRow {
        name: "data-highlighted",
        description: "Present on Item and Toggle when the item has keyboard focus.",
    },
    DataAttrRow {
        name: "data-disabled",
        description: "Present on Item and Toggle when the item is disabled.",
    },
];

const KEYBOARD: &[KeyboardRow] = &[
    KeyboardRow {
        key: "ArrowDown",
        description: "Moves focus to the next item's toggle.",
    },
    KeyboardRow {
        key: "ArrowUp",
        description: "Moves focus to the previous item's toggle.",
    },
    KeyboardRow {
        key: "Home",
        description: "Moves focus to the first item's toggle.",
    },
    KeyboardRow {
        key: "End",
        description: "Moves focus to the last item's toggle.",
    },
    KeyboardRow {
        key: "Enter / Space",
        description: "Toggles the focused item open or closed.",
    },
];

#[component]
pub fn AccordionDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    view! {
        <DocPage title="Accordion">
            <p class="mt-3 mb-11 text-base text-balance">
                "An accordion is a vertically stacked list of items, such as labels or thumbnails, that can be toggled to reveal or hide additional content."
            </p>
            <DocPreview>
                <AccordionExample />
            </DocPreview>
            <SectionHeading title="Usage" />
            <Code
                class="[&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={USAGE_CODE}
                language="rust"
            />
            <SectionHeading title="API Reference" />
            <PropsTable title="Root" rows={ROOT_PROPS} />
            <PropsTable title="Item" rows={ITEM_PROPS} />
            <PropsTable title="Toggle" rows={TOGGLE_PROPS} />
            <PropsTable title="Content" rows={CONTENT_PROPS} />
            <DataAttrsTable rows={DATA_ATTRS} />
            <KeyboardTable rows={KEYBOARD} />
        </DocPage>
    }
}

#[component]
pub fn AccordionExample() -> impl IntoView {
    use biji_ui::components::accordion;

    let items = [
        ("What is the meaning of life?", "To become a better person, to help others, and to leave the world a better place than you found it."),
        ("How do I become a better person?", "Read books, listen to podcasts, and surround yourself with people who inspire you."),
        ("What is the best way to help others?", "Give them your time, attention, and love."),
    ];

    view! {
        <accordion::Root class="w-full sm:max-w-[70%]">
            {items
                .into_iter()
                .map(|(title, content)| {
                    view! {
                        <accordion::Item class="border-b border-border group">
                            <accordion::Toggle class="flex w-full px-1.5 flex-1 items-center justify-between py-5 text-[15px] font-medium outline-none transition-all focus:rounded-xl focus:outline-none [&[data-state=open]>span>svg]:rotate-180 data-[highlighted]:bg-muted !ring-0 !ring-transparent">
                                {title}
                                <span class="inline-flex size-8 items-center justify-center rounded-[7px] bg-transparent transition-all hover:bg-dark-10">
                                    <icons::Caret class="size-[18px] transition-all duration-200"></icons::Caret>
                                </span>
                            </accordion::Toggle>
                            <accordion::Content
                                class="px-1.5 pt-1.5 pb-[25px] text-sm tracking-[-0.01em]"
                                show_class="opacity-100 transition duration-150 ease-in"
                                hide_class="opacity-0 transition duration-200 ease-out"
                                hide_delay={Duration::from_millis(200)}
                            >
                                {content}
                            </accordion::Content>
                        </accordion::Item>
                    }
                })
                .collect::<Vec<_>>()}
        </accordion::Root>
    }
}
