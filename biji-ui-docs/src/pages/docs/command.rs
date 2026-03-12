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
    "\", features = [\"command\"] }",
);

const USAGE_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::command;

#[component]
pub fn MyCommand() -> impl IntoView {
    view! {
        <command::Root class="overflow-hidden w-full max-w-sm rounded-lg border shadow-md border-border">
            <command::Input
                placeholder="Search..."
                class="py-2 px-3 w-full text-sm border-b outline-none border-border bg-background"
            />
            <command::List class="overflow-y-auto p-1 max-h-64">
                <command::Empty>
                    <div class="py-6 text-sm text-center text-muted-foreground">
                        "No results found."
                    </div>
                </command::Empty>
                <command::Group label="Actions" label_class="px-2 py-1 text-xs font-semibold text-muted-foreground">
                    <command::Item
                        value="new-file"
                        class="flex items-center py-1.5 px-2 text-sm rounded-sm cursor-pointer outline-none data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground data-[disabled]:opacity-50 data-[disabled]:pointer-events-none"
                        on_select={Callback::new(|val| leptos::logging::log!("selected: {val}"))}
                    >
                        "New File"
                    </command::Item>
                    <command::Item
                        value="new-folder"
                        class="flex items-center py-1.5 px-2 text-sm rounded-sm cursor-pointer outline-none data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground"
                    >
                        "New Folder"
                    </command::Item>
                </command::Group>
            </command::List>
        </command::Root>
    }
}"#;

const ROOT_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the root wrapper element.",
}];

const INPUT_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the search input element.",
    },
    PropRow {
        name: "placeholder",
        prop_type: "String",
        default: "\"\"",
        description: "Placeholder text shown when the input is empty.",
    },
];

const LIST_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the list container element.",
}];

const GROUP_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the group container element.",
    },
    PropRow {
        name: "label",
        prop_type: "Option<String>",
        default: "None",
        description: "Optional heading text rendered above the group items.",
    },
    PropRow {
        name: "label_class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the group label element.",
    },
];

const ITEM_PROPS: &[PropRow] = &[
    PropRow {
        name: "value",
        prop_type: "String",
        default: "",
        description: "Unique string value identifying this item. Also used as the default label for filtering.",
    },
    PropRow {
        name: "label",
        prop_type: "Option<String>",
        default: "None",
        description: "Override the text used for filtering. Defaults to value when not provided.",
    },
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the item element.",
    },
    PropRow {
        name: "disabled",
        prop_type: "bool",
        default: "false",
        description: "When true, the item is excluded from navigation and cannot be selected.",
    },
    PropRow {
        name: "on_select",
        prop_type: "Option<Callback<String>>",
        default: "None",
        description: "Callback fired with the item's value when it is selected via click or Enter.",
    },
];

const HIGHLIGHTED_TEXT_PROPS: &[PropRow] = &[
    PropRow {
        name: "label",
        prop_type: "String",
        default: "",
        description: "The full text string to display. The matched portion is wrapped in a highlighted span.",
    },
    PropRow {
        name: "highlight_class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the span wrapping the matched substring.",
    },
];

const HIGHLIGHTED_USAGE_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::command;

#[component]
pub fn MyHighlightedCommand() -> impl IntoView {
    view! {
        <command::Root class="overflow-hidden w-full max-w-sm rounded-lg border shadow-md border-border">
            <command::Input
                placeholder="Search..."
                class="py-2 px-3 w-full text-sm border-b outline-none border-border bg-background"
            />
            <command::List class="overflow-y-auto p-1 max-h-64">
                <command::Empty>
                    <div class="py-6 text-sm text-center text-muted-foreground">
                        "No results found."
                    </div>
                </command::Empty>
                <command::Item
                    value="accordion"
                    class="flex items-center py-1.5 px-2 text-sm rounded-sm cursor-pointer outline-none data-[highlighted]:bg-accent"
                >
                    <command::HighlightedText
                        label="Accordion"
                        highlight_class="bg-yellow-200/80 dark:bg-yellow-500/30 rounded"
                    />
                </command::Item>
                <command::Item
                    value="dialog"
                    class="flex items-center py-1.5 px-2 text-sm rounded-sm cursor-pointer outline-none data-[highlighted]:bg-accent"
                >
                    <command::HighlightedText
                        label="Dialog"
                        highlight_class="bg-yellow-200/80 dark:bg-yellow-500/30 rounded"
                    />
                </command::Item>
            </command::List>
        </command::Root>
    }
}"#;

const DATA_ATTRS: &[DataAttrRow] = &[
    DataAttrRow {
        name: "data-highlighted",
        description: "Present on Item when it is the currently focused/highlighted item.",
    },
    DataAttrRow {
        name: "data-disabled",
        description: "Present on Item when the item is disabled.",
    },
];

const KEYBOARD: &[KeyboardRow] = &[
    KeyboardRow {
        key: "ArrowDown",
        description: "Moves focus to the next visible item, wrapping to the first.",
    },
    KeyboardRow {
        key: "ArrowUp",
        description: "Moves focus to the previous visible item, wrapping to the last.",
    },
    KeyboardRow {
        key: "Home",
        description: "Moves focus to the first visible item.",
    },
    KeyboardRow {
        key: "End",
        description: "Moves focus to the last visible item.",
    },
    KeyboardRow {
        key: "Enter",
        description: "Selects the focused item and fires on_select.",
    },
];

#[component]
pub fn CommandDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    view! {
        <DocPage title="Command">
            <p class="mt-3 mb-11 text-base text-balance">
                "A searchable command palette for navigating and executing actions via keyboard or mouse."
            </p>
            <DocPreview>
                <CommandExample />
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
            <SectionHeading title="Text Highlighting" />
            <p class="mb-4 text-sm text-muted-foreground">
                "Use " <code class="font-mono text-foreground">"command::HighlightedText"</code>
                " inside an item to automatically highlight the portion of the label that matches the current search query."
            </p>
            <DocPreview>
                <HighlightedCommandExample />
            </DocPreview>
            <Code
                class="[&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={HIGHLIGHTED_USAGE_CODE}
                language="rust"
            />
            <SectionHeading title="API Reference" />
            <PropsTable title="Root" rows={ROOT_PROPS} />
            <PropsTable title="Input" rows={INPUT_PROPS} />
            <PropsTable title="List" rows={LIST_PROPS} />
            <PropsTable title="Group" rows={GROUP_PROPS} />
            <PropsTable title="Item" rows={ITEM_PROPS} />
            <PropsTable title="HighlightedText" rows={HIGHLIGHTED_TEXT_PROPS} />
            <DataAttrsTable rows={DATA_ATTRS} />
            <KeyboardTable rows={KEYBOARD} />
        </DocPage>
    }
}

#[component]
pub fn CommandExample() -> impl IntoView {
    use biji_ui::components::command;

    const ROOT_CLS: &str =
        "w-full max-w-sm rounded-lg border border-border shadow-md overflow-hidden bg-background";
    const INPUT_CLS: &str = "w-full px-3 py-2 text-sm border-b border-border outline-none bg-background placeholder:text-muted-foreground";
    const ITEM_CLS: &str = "flex items-center gap-2 px-2 py-1.5 text-sm rounded-sm cursor-pointer outline-none \
        data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground \
        data-[disabled]:opacity-50 data-[disabled]:pointer-events-none";
    const GROUP_LABEL_CLS: &str =
        "px-2 py-1 text-xs font-semibold text-muted-foreground uppercase tracking-wider";

    let selected = RwSignal::new(String::new());

    view! {
        <div class="flex flex-col gap-4 w-full max-w-sm">
            <command::Root class={ROOT_CLS}>
                <command::Input placeholder="Type to search..." class={INPUT_CLS} />
                <command::List class="overflow-y-auto p-1 max-h-64">
                    <command::Empty>
                        <div class="py-6 text-sm text-center text-muted-foreground">
                            "No results found."
                        </div>
                    </command::Empty>
                    <command::Group label="Files" label_class={GROUP_LABEL_CLS}>
                        <command::Item
                            value="new-file"
                            class={ITEM_CLS}
                            on_select={Callback::new(move |v| selected.set(v))}
                        >
                            "New File"
                        </command::Item>
                        <command::Item
                            value="open-file"
                            class={ITEM_CLS}
                            on_select={Callback::new(move |v| selected.set(v))}
                        >
                            "Open File"
                        </command::Item>
                        <command::Item
                            value="save-file"
                            class={ITEM_CLS}
                            on_select={Callback::new(move |v| selected.set(v))}
                        >
                            "Save File"
                        </command::Item>
                    </command::Group>
                    <command::Group label="Edit" label_class={GROUP_LABEL_CLS}>
                        <command::Item
                            value="cut"
                            class={ITEM_CLS}
                            on_select={Callback::new(move |v| selected.set(v))}
                        >
                            "Cut"
                        </command::Item>
                        <command::Item
                            value="copy"
                            class={ITEM_CLS}
                            on_select={Callback::new(move |v| selected.set(v))}
                        >
                            "Copy"
                        </command::Item>
                        <command::Item
                            value="paste"
                            disabled=true
                            class={ITEM_CLS}
                            on_select={Callback::new(move |v| selected.set(v))}
                        >
                            "Paste (disabled)"
                        </command::Item>
                    </command::Group>
                </command::List>
            </command::Root>
            <Show when={move || !selected.get().is_empty()}>
                <p class="text-sm text-muted-foreground">
                    "Selected: "
                    <span class="font-medium text-foreground">{move || selected.get()}</span>
                </p>
            </Show>
        </div>
    }
}

#[component]
pub fn HighlightedCommandExample() -> impl IntoView {
    use biji_ui::components::command;

    const ROOT_CLS: &str =
        "w-full max-w-sm rounded-lg border border-border shadow-md overflow-hidden bg-background";
    const INPUT_CLS: &str = "w-full px-3 py-2 text-sm border-b border-border outline-none bg-background placeholder:text-muted-foreground";
    const ITEM_CLS: &str = "flex items-center gap-2 px-2 py-1.5 text-sm rounded-sm cursor-pointer outline-none \
        data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground";
    const HIGHLIGHT_CLS: &str = "bg-yellow-200/80 dark:bg-yellow-500/30 rounded";
    const GROUP_LABEL_CLS: &str =
        "px-2 py-1 text-xs font-semibold text-muted-foreground uppercase tracking-wider";

    let items = [
        ("accordion", "Accordion"),
        ("alert-dialog", "Alert Dialog"),
        ("calendar", "Calendar"),
        ("checkbox", "Checkbox"),
        ("collapsible", "Collapsible"),
        ("dialog", "Dialog"),
        ("popover", "Popover"),
        ("select", "Select"),
        ("tabs", "Tabs"),
        ("tooltip", "Tooltip"),
    ];

    view! {
        <command::Root class={ROOT_CLS}>
            <command::Input placeholder="Search docs..." class={INPUT_CLS} />
            <command::List class="overflow-y-auto p-1 max-h-64">
                <command::Empty>
                    <div class="py-6 text-sm text-center text-muted-foreground">
                        "No results found."
                    </div>
                </command::Empty>
                <command::Group label="Components" label_class={GROUP_LABEL_CLS}>
                    {items
                        .into_iter()
                        .map(|(value, label)| {
                            view! {
                                <command::Item value={value} class={ITEM_CLS}>
                                    <command::HighlightedText
                                        label={label.to_string()}
                                        highlight_class={HIGHLIGHT_CLS}
                                    />
                                </command::Item>
                            }
                        })
                        .collect_view()}
                </command::Group>
            </command::List>
        </command::Root>
    }
}
