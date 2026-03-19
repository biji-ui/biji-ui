use leptos::{portal::Portal, prelude::*};

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
    "\", features = [\"select\"] }",
);

const USAGE_CODE: &str = r#"use leptos::{portal::Portal, prelude::*};
use biji_ui::components::select;

// Reusable class constants
const TRIGGER_CLS: &str =
    "flex h-10 w-48 items-center justify-between rounded-md border border-input \
     bg-background px-3 py-2 text-sm ring-offset-background \
     focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring \
     focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 \
     data-[state=open]:ring-2 data-[state=open]:ring-ring";

const CONTENT_CLS: &str =
    "z-50 min-w-[8rem] overflow-hidden rounded-md border border-border \
     bg-background text-foreground shadow-md text-sm py-1 \
     transition origin-[var(--biji-transform-origin)]";

const ITEM_CLS: &str =
    "relative flex w-full cursor-default select-none items-center justify-between \
     rounded-sm px-3 py-1.5 text-sm outline-none \
     hover:bg-accent hover:text-accent-foreground \
     focus:bg-accent focus:text-accent-foreground \
     data-[disabled]:pointer-events-none data-[disabled]:opacity-50";

#[component]
pub fn MySelect() -> impl IntoView {
    view! {
        <select::Root>
            <select::Trigger class=TRIGGER_CLS>
                <select::Value placeholder="Select a fruit..." />
                <svg class="h-4 w-4 opacity-50 shrink-0" viewBox="0 0 24 24" fill="none"
                    stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7"/>
                </svg>
            </select::Trigger>
            <Portal>
                <select::Content
                    class=CONTENT_CLS
                    show_class="opacity-100 scale-100 duration-150 ease-out"
                    hide_class="opacity-0 scale-95 duration-100 ease-in"
                >
                    <select::Item value="apple" class=ITEM_CLS>
                        <select::ItemText>"Apple"</select::ItemText>
                        <select::ItemIndicator>
                            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none"
                                stroke="currentColor" stroke-width="2.5">
                                <path stroke-linecap="round" stroke-linejoin="round"
                                    d="M5 13l4 4L19 7"/>
                            </svg>
                        </select::ItemIndicator>
                    </select::Item>
                    <select::Item value="banana" class=ITEM_CLS>
                        <select::ItemText>"Banana"</select::ItemText>
                        <select::ItemIndicator>
                            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none"
                                stroke="currentColor" stroke-width="2.5">
                                <path stroke-linecap="round" stroke-linejoin="round"
                                    d="M5 13l4 4L19 7"/>
                            </svg>
                        </select::ItemIndicator>
                    </select::Item>
                    <select::Item value="cherry" class=ITEM_CLS>
                        <select::ItemText>"Cherry"</select::ItemText>
                        <select::ItemIndicator>
                            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none"
                                stroke="currentColor" stroke-width="2.5">
                                <path stroke-linecap="round" stroke-linejoin="round"
                                    d="M5 13l4 4L19 7"/>
                            </svg>
                        </select::ItemIndicator>
                    </select::Item>
                </select::Content>
            </Portal>
        </select::Root>
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
        name: "value",
        prop_type: "Option<String>",
        default: "None",
        description: "The initially selected value.",
    },
    PropRow {
        name: "positioning",
        prop_type: "Positioning",
        default: "BottomStart",
        description: "Where to render the content relative to the trigger.",
    },
    PropRow {
        name: "hide_delay",
        prop_type: "Duration",
        default: "200ms",
        description: "How long to wait before unmounting the content after closing begins.",
    },
    PropRow {
        name: "avoid_collisions",
        prop_type: "AvoidCollisions",
        default: "Flip",
        description: "How the overlay reacts when it would overflow the viewport.",
    },
    PropRow {
        name: "on_value_change",
        prop_type: "Option<Callback<String>>",
        default: "None",
        description: "Callback fired when the selected value changes.",
    },
];

const TRIGGER_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the trigger button. Use data-[state=open]:... to style the open state.",
}];

const VALUE_PROPS: &[PropRow] = &[PropRow {
    name: "placeholder",
    prop_type: "String",
    default: "\"\"",
    description: "Text shown when no value is selected.",
}];

const CONTENT_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied in both open and closed states. Add origin-[var(--biji-transform-origin)] to scale animations from the trigger direction.",
    },
    PropRow {
        name: "show_class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied when the select is open.",
    },
    PropRow {
        name: "hide_class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied while the select is closing.",
    },
];

const ITEM_PROPS: &[PropRow] = &[
    PropRow {
        name: "value",
        prop_type: "String",
        default: "",
        description: "The value this item represents (stored in context on selection).",
    },
    PropRow {
        name: "label",
        prop_type: "Option<String>",
        default: "value",
        description: "Display text shown in the trigger when this item is selected. Defaults to value if not provided.",
    },
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the item element. Use hover: and focus: to style the highlighted state.",
    },
    PropRow {
        name: "disabled",
        prop_type: "bool",
        default: "false",
        description: "When true, the item cannot be selected.",
    },
];

const DATA_ATTRS: &[DataAttrRow] = &[
    DataAttrRow {
        name: "data-state",
        description: "\"open\" or \"closed\" on Trigger. \"checked\" or \"unchecked\" on Item.",
    },
    DataAttrRow {
        name: "data-highlighted",
        description: "Present on Item when it has keyboard focus or is hovered. Style the focused state with hover: and focus: Tailwind classes on the item directly.",
    },
    DataAttrRow {
        name: "data-disabled",
        description: "Present on Item when the item is disabled.",
    },
];

const KEYBOARD: &[KeyboardRow] = &[
    KeyboardRow {
        key: "ArrowDown",
        description: "Opens the select and focuses the first item; navigates to the next item when open.",
    },
    KeyboardRow {
        key: "ArrowUp",
        description: "Opens the select and focuses the last item; navigates to the previous item when open.",
    },
    KeyboardRow {
        key: "Enter / Space",
        description: "Selects the focused item and closes the dropdown.",
    },
    KeyboardRow {
        key: "Home",
        description: "Moves focus to the first item.",
    },
    KeyboardRow {
        key: "End",
        description: "Moves focus to the last item.",
    },
    KeyboardRow {
        key: "Escape",
        description: "Closes the select without selecting and returns focus to the trigger.",
    },
    KeyboardRow {
        key: "Tab",
        description: "Closes the select and moves focus to the next focusable element.",
    },
];

const ROOT_WITH_CODE: &str = r#"use leptos::{portal::Portal, prelude::*};
use biji_ui::components::select;

#[component]
pub fn MySelect() -> impl IntoView {
    view! {
        <select::RootWith let:s>
            <p class="text-sm text-muted-foreground">
                {move || {
                    if s.open.get() {
                        "Dropdown is open".to_string()
                    } else {
                        s.value.get()
                            .map(|v| format!("Selected: {v}"))
                            .unwrap_or_else(|| "Nothing selected".to_string())
                    }
                }}
            </p>
            <select::Trigger class="...">
                <select::Value placeholder="Select a fruit..." />
            </select::Trigger>
            <Portal>
                <select::Content class="..." show_class="..." hide_class="...">
                    <select::Item value="apple" label="Apple" class="...">
                        <select::ItemText>"Apple"</select::ItemText>
                    </select::Item>
                </select::Content>
            </Portal>
        </select::RootWith>
    }
}"#;

// Shared item class used in the preview example.
const ITEM_CLS: &str = "relative flex w-full cursor-default select-none items-center \
     justify-between rounded-sm px-3 py-1.5 text-sm outline-none \
     hover:bg-accent hover:text-accent-foreground \
     focus:bg-accent focus:text-accent-foreground \
     data-[disabled]:pointer-events-none data-[disabled]:opacity-50";

const TRIGGER_CLS: &str =
    "flex h-10 w-52 items-center justify-between gap-2 rounded-md border border-input \
     bg-background px-3 py-2 text-sm ring-offset-background \
     focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring \
     focus-visible:ring-offset-2 data-[state=open]:ring-2 data-[state=open]:ring-ring";

const CONTENT_CLS: &str =
    "z-50 min-w-[8rem] overflow-hidden rounded-md border border-border \
     bg-background text-foreground shadow-md text-sm py-1 \
     transition origin-[var(--biji-transform-origin)]";

#[component]
pub fn SelectDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    view! {
        <DocPage title="Select">
            <p class="mt-3 mb-11 text-base text-balance">
                "An accessible custom select that displays a list of options in an anchor-positioned overlay."
            </p>
            <DocPreview>
                <SelectExample />
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
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"SelectState"</code>
                " inline via the "
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"let:"</code>
                " binding. The state is "
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"Copy"</code>
                " and safe to pass as a prop."
            </p>
            <DocPreview>
                <SelectRootWithExample />
            </DocPreview>
            <Code
                class="[&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={ROOT_WITH_CODE}
                language="rust"
            />
            <SectionHeading title="API Reference" />
            <PropsTable title="Root / RootWith" rows={ROOT_PROPS} />
            <PropsTable title="Trigger" rows={TRIGGER_PROPS} />
            <PropsTable title="Value" rows={VALUE_PROPS} />
            <PropsTable title="Content" rows={CONTENT_PROPS} />
            <PropsTable title="Item" rows={ITEM_PROPS} />
            <DataAttrsTable rows={DATA_ATTRS} />
            <KeyboardTable rows={KEYBOARD} />
        </DocPage>
    }
}

#[component]
fn ChevronDown() -> impl IntoView {
    view! {
        <svg
            class="h-4 w-4 opacity-50 shrink-0"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
        >
            <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7" />
        </svg>
    }
}

#[component]
fn CheckIcon() -> impl IntoView {
    view! {
        <svg
            class="h-4 w-4"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2.5"
        >
            <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
        </svg>
    }
}

#[component]
pub fn SelectRootWithExample() -> impl IntoView {
    use biji_ui::components::select;

    view! {
        <div class="flex flex-col items-center gap-4 p-8">
            <select::RootWith let:s>
                <p class="text-sm text-muted-foreground">
                    {move || {
                        if s.open.get() {
                            "Dropdown is open".to_string()
                        } else {
                            s.value
                                .get()
                                .map(|v| format!("Selected: {v}"))
                                .unwrap_or_else(|| "Nothing selected".to_string())
                        }
                    }}
                </p>
                <select::Trigger class={TRIGGER_CLS}>
                    <select::Value placeholder="Select a fruit..." />
                    <ChevronDown />
                </select::Trigger>
                <Portal>
                    <select::Content
                        class={CONTENT_CLS}
                        show_class="opacity-100 scale-100 duration-150 ease-out"
                        hide_class="opacity-0 scale-95 duration-100 ease-in"
                    >
                        <select::Item value="apple" label="Apple" class={ITEM_CLS}>
                            <select::ItemText>"Apple"</select::ItemText>
                            <select::ItemIndicator><CheckIcon /></select::ItemIndicator>
                        </select::Item>
                        <select::Item value="banana" label="Banana" class={ITEM_CLS}>
                            <select::ItemText>"Banana"</select::ItemText>
                            <select::ItemIndicator><CheckIcon /></select::ItemIndicator>
                        </select::Item>
                        <select::Item value="cherry" label="Cherry" class={ITEM_CLS}>
                            <select::ItemText>"Cherry"</select::ItemText>
                            <select::ItemIndicator><CheckIcon /></select::ItemIndicator>
                        </select::Item>
                        <select::Item value="mango" label="Mango" class={ITEM_CLS}>
                            <select::ItemText>"Mango"</select::ItemText>
                            <select::ItemIndicator><CheckIcon /></select::ItemIndicator>
                        </select::Item>
                    </select::Content>
                </Portal>
            </select::RootWith>
        </div>
    }
}

#[component]
pub fn SelectExample() -> impl IntoView {
    use biji_ui::components::select;

    view! {
        <div class="flex flex-col gap-4 items-center">
            <select::RootWith let:s>
                <select::Trigger class={TRIGGER_CLS}>
                    <select::Value placeholder="Select a fruit..." />
                    <ChevronDown />
                </select::Trigger>
                <Portal>
                    <select::Content
                        class={CONTENT_CLS}
                        show_class="opacity-100 scale-100 duration-150 ease-out"
                        hide_class="opacity-0 scale-95 duration-100 ease-in"
                    >
                        <select::Item value="apple" label="Apple" class={ITEM_CLS}>
                            <select::ItemText>"Apple"</select::ItemText>
                            <select::ItemIndicator><CheckIcon /></select::ItemIndicator>
                        </select::Item>
                        <select::Item value="banana" label="Banana" class={ITEM_CLS}>
                            <select::ItemText>"Banana"</select::ItemText>
                            <select::ItemIndicator><CheckIcon /></select::ItemIndicator>
                        </select::Item>
                        <select::Item value="cherry" label="Cherry" class={ITEM_CLS}>
                            <select::ItemText>"Cherry"</select::ItemText>
                            <select::ItemIndicator><CheckIcon /></select::ItemIndicator>
                        </select::Item>
                        <select::Item value="mango" label="Mango" class={ITEM_CLS}>
                            <select::ItemText>"Mango"</select::ItemText>
                            <select::ItemIndicator><CheckIcon /></select::ItemIndicator>
                        </select::Item>
                        <select::Item value="pineapple" label="Pineapple (disabled)" disabled=true class={ITEM_CLS}>
                            <select::ItemText>"Pineapple (disabled)"</select::ItemText>
                        </select::Item>
                    </select::Content>
                </Portal>
                <p class="text-sm text-muted-foreground">
                    {move || {
                        s.value
                            .get()
                            .map(|v| format!("Selected: {v}"))
                            .unwrap_or_else(|| "Nothing selected yet".to_string())
                    }}
                </p>
            </select::RootWith>
        </div>
    }
}
