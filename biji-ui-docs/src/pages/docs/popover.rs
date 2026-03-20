use leptos::{portal::Portal, prelude::*};

use crate::components::{
    api_table::{
        DataAttrRow, DataAttrsTable, KeyboardRow, KeyboardTable, PropRow, PropsTable,
        SectionHeading,
    },
    button::{Variant, btn},
    code::Code,
};

const INSTALL_CODE: &str = concat!(
    "biji-ui = { version = \"",
    env!("CARGO_PKG_VERSION"),
    "\", features = [\"popover\"] }",
);

const USAGE_CODE: &str = r#"use leptos::{portal::Portal, prelude::*};
use biji_ui::components::popover;

#[component]
pub fn MyPopover() -> impl IntoView {
    view! {
        <popover::Root positioning={popover::Positioning::Bottom}>
            <popover::Trigger class="py-1.5 px-3 text-sm rounded border">
                "Open popover"
            </popover::Trigger>
            <Portal>
                <popover::Content
                    class="z-50 p-4 text-sm rounded-lg border shadow-md transition bg-background origin-[var(--biji-transform-origin)]"
                    show_class="opacity-100 scale-100 duration-150 ease-out"
                    hide_class="opacity-0 scale-95 duration-100 ease-in"
                >
                    <popover::Arrow class="border-t border-l border-border" />
                    "Popover content goes here."
                </popover::Content>
            </Portal>
        </popover::Root>
    }
}"#;

const ROOT_WITH_CODE: &str = r#"use leptos::{portal::Portal, prelude::*};
use biji_ui::components::popover;

#[component]
pub fn MyPopover() -> impl IntoView {
    view! {
        <popover::RootWith let:p>
            <p class="mb-2 text-sm text-muted-foreground">
                {move || if p.open.get() { "Popover is open" } else { "Popover is closed" }}
            </p>
            <popover::Trigger class="py-1.5 px-3 text-sm rounded border">
                "Open popover"
            </popover::Trigger>
            <Portal>
                <popover::Content
                    class="z-50 p-4 text-sm rounded-lg border shadow-md transition bg-background origin-[var(--biji-transform-origin)]"
                    show_class="opacity-100 scale-100 duration-150 ease-out"
                    hide_class="opacity-0 scale-95 duration-100 ease-in"
                >
                    "Popover content goes here."
                </popover::Content>
            </Portal>
        </popover::RootWith>
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
        default: "Bottom",
        description: "Where to render the content relative to the trigger.",
    },
    PropRow {
        name: "hide_delay",
        prop_type: "Duration",
        default: "200ms",
        description: "How long to wait before unmounting the content after closing begins.",
    },
    PropRow {
        name: "arrow_size",
        prop_type: "i32",
        default: "8",
        description: "Width and height of the arrow element in pixels.",
    },
    PropRow {
        name: "open",
        prop_type: "bool",
        default: "false",
        description: "Initial open state.",
    },
    PropRow {
        name: "auto_focus",
        prop_type: "bool",
        default: "true",
        description: "When true, focuses the first focusable element inside Content when the popover opens.",
    },
    PropRow {
        name: "avoid_collisions",
        prop_type: "AvoidCollisions",
        default: "Flip",
        description: "How the overlay reacts when it would overflow the viewport.",
    },
    PropRow {
        name: "on_open_change",
        prop_type: "Option<Callback<bool>>",
        default: "None",
        description: "Callback fired when the open state changes.",
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
        description: "CSS class applied in both open and closed states. Use `transition` rather than `transition-all` to avoid animating position changes. Add `origin-[var(--biji-transform-origin)]` to scale animations from the trigger direction.",
    },
    PropRow {
        name: "show_class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied when the popover is open.",
    },
    PropRow {
        name: "hide_class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied while the popover is closing.",
    },
];

const POSITIONING_PROPS: &[PropRow] = &[
    PropRow {
        name: "TopStart",
        prop_type: "Positioning",
        default: "",
        description: "Above the trigger, aligned to its left edge.",
    },
    PropRow {
        name: "Top",
        prop_type: "Positioning",
        default: "",
        description: "Above the trigger, centered.",
    },
    PropRow {
        name: "TopEnd",
        prop_type: "Positioning",
        default: "",
        description: "Above the trigger, aligned to its right edge.",
    },
    PropRow {
        name: "RightStart",
        prop_type: "Positioning",
        default: "",
        description: "To the right of the trigger, aligned to its top edge.",
    },
    PropRow {
        name: "Right",
        prop_type: "Positioning",
        default: "",
        description: "To the right of the trigger, centered.",
    },
    PropRow {
        name: "RightEnd",
        prop_type: "Positioning",
        default: "",
        description: "To the right of the trigger, aligned to its bottom edge.",
    },
    PropRow {
        name: "BottomStart",
        prop_type: "Positioning",
        default: "",
        description: "Below the trigger, aligned to its left edge.",
    },
    PropRow {
        name: "Bottom",
        prop_type: "Positioning",
        default: "default",
        description: "Below the trigger, centered.",
    },
    PropRow {
        name: "BottomEnd",
        prop_type: "Positioning",
        default: "",
        description: "Below the trigger, aligned to its right edge.",
    },
    PropRow {
        name: "LeftStart",
        prop_type: "Positioning",
        default: "",
        description: "To the left of the trigger, aligned to its top edge.",
    },
    PropRow {
        name: "Left",
        prop_type: "Positioning",
        default: "",
        description: "To the left of the trigger, centered.",
    },
    PropRow {
        name: "LeftEnd",
        prop_type: "Positioning",
        default: "",
        description: "To the left of the trigger, aligned to its bottom edge.",
    },
];

const AVOID_COLLISIONS_PROPS: &[PropRow] = &[
    PropRow {
        name: "Flip",
        prop_type: "AvoidCollisions",
        default: "default",
        description: "Keeps the preferred side. Flips to the opposite side if it does not fit. If neither fits, uses whichever has more space.",
    },
    PropRow {
        name: "AutoPlace",
        prop_type: "AvoidCollisions",
        default: "",
        description: "Always places the overlay on the side with the most available space, regardless of the preferred positioning.",
    },
    PropRow {
        name: "None",
        prop_type: "AvoidCollisions",
        default: "",
        description: "No collision detection. Always uses the exact positioning specified.",
    },
];

const ARROW_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the arrow element.",
}];

const DATA_ATTRS: &[DataAttrRow] = &[DataAttrRow {
    name: "data-state",
    description: "\"open\" when the popover is visible; \"closed\" when hidden. Present on Trigger.",
}];

const KEYBOARD: &[KeyboardRow] = &[KeyboardRow {
    key: "Escape",
    description: "Closes the popover and returns focus to the trigger.",
}];

#[component]
pub fn PopoverDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    view! {
        <DocPage title="Popover">
            <p class="mt-3 mb-11 text-base text-balance">
                "An anchor-positioned floating panel that displays rich content relative to a trigger element."
            </p>
            <DocPreview>
                <PopoverExample />
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
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"PopoverState"</code>
                " inline via the "
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"let:"</code>
                " binding. The state is "
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"Copy"</code>
                " and safe to pass as a prop."
            </p>
            <DocPreview>
                <PopoverRootWithExample />
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
            <PropsTable title="Positioning" rows={POSITIONING_PROPS} />
            <PropsTable title="AvoidCollisions" rows={AVOID_COLLISIONS_PROPS} />
            <DataAttrsTable rows={DATA_ATTRS} />
            <KeyboardTable rows={KEYBOARD} />
        </DocPage>
    }
}

#[component]
pub fn PopoverRootWithExample() -> impl IntoView {
    use biji_ui::components::popover;

    view! {
        <div class="flex flex-col items-center gap-3 p-8">
            <popover::RootWith let:p>
                <p class="text-sm text-muted-foreground">
                    {move || if p.open.get() { "Popover is open" } else { "Popover is closed" }}
                </p>
                <popover::Trigger class={btn(Variant::Outline)}>
                    "Open popover"
                </popover::Trigger>
                <Portal>
                    <popover::Content
                        class="z-50 p-4 w-64 text-sm rounded-lg border shadow-md transition \
                               bg-background border-border origin-[var(--biji-transform-origin)]"
                        show_class="opacity-100 scale-100 duration-150 ease-out"
                        hide_class="opacity-0 scale-95 duration-100 ease-in"
                    >
                        <popover::Arrow class="border-t border-l border-border" />
                        "Popover content goes here."
                    </popover::Content>
                </Portal>
            </popover::RootWith>
        </div>
    }
}

#[component]
pub fn PopoverExample() -> impl IntoView {
    use biji_ui::components::popover;

    view! {
        <div class="flex flex-wrap gap-6 justify-center">
            <popover::Root positioning={popover::Positioning::Bottom}>
                <popover::Trigger class={btn(Variant::Outline)}>
                    "Open popover"
                </popover::Trigger>
                <Portal>
                    <popover::Content
                        class="p-4 w-72 text-sm rounded-lg border shadow-md transition z-50 border-border bg-background origin-[var(--biji-transform-origin)]"
                        show_class="opacity-100 scale-100 duration-150 ease-out"
                        hide_class="opacity-0 scale-95 duration-100 ease-in"
                    >
                        <popover::Arrow class="border-t border-l border-border bg-background" />
                        <p class="mb-1 font-semibold">"Dimensions"</p>
                        <p class="text-xs text-muted-foreground">
                            "Set the dimensions for the layer."
                        </p>
                        <div class="flex flex-col gap-2 mt-3">
                            <label class="flex flex-col gap-1 text-xs">
                                <span class="font-medium">"Width"</span>
                                <input
                                    type="text"
                                    placeholder="100%"
                                    class="py-1 px-3 h-8 text-sm rounded-md border focus:ring-2 focus:outline-none border-input bg-background focus:ring-ring"
                                />
                            </label>
                            <label class="flex flex-col gap-1 text-xs">
                                <span class="font-medium">"Max width"</span>
                                <input
                                    type="text"
                                    placeholder="300px"
                                    class="py-1 px-3 h-8 text-sm rounded-md border focus:ring-2 focus:outline-none border-input bg-background focus:ring-ring"
                                />
                            </label>
                        </div>
                    </popover::Content>
                </Portal>
            </popover::Root>
        </div>
    }
}
