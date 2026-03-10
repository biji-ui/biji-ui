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
    "\", features = [\"combobox\"] }",
);

const USAGE_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::combobox;

#[component]
pub fn MyCombobox() -> impl IntoView {
    view! {
        <combobox::Root inline=true>
            <combobox::InputTrigger
                class="w-48 px-3 py-2 text-sm rounded-md border border-border bg-background outline-none"
                placeholder="Search a fruit..."
            />
            <combobox::Content class="z-50 w-48 overflow-hidden rounded-md border border-border bg-background py-1 shadow-md text-sm">
                <combobox::Empty>
                    <div class="px-3 py-4 text-center text-muted-foreground">"No results."</div>
                </combobox::Empty>
                <combobox::Item
                    value="apple"
                    label="Apple"
                    class="px-3 py-1.5 cursor-default select-none outline-none data-[highlighted]:bg-accent"
                >
                    "Apple"
                </combobox::Item>
                <combobox::Item
                    value="banana"
                    label="Banana"
                    class="px-3 py-1.5 cursor-default select-none outline-none data-[highlighted]:bg-accent"
                >
                    "Banana"
                </combobox::Item>
            </combobox::Content>
        </combobox::Root>
    }
}"#;

const USAGE_CODE_BUTTON: &str = r#"// Alternative: button trigger with search inside the dropdown
<combobox::Root>
    <combobox::Trigger class="...">
        <combobox::Value placeholder="Select a fruit..." />
        "▾"
    </combobox::Trigger>
    <combobox::Content class="...">
        <combobox::Input placeholder="Search..." class="..." />
        <combobox::Item value="apple" label="Apple" class="...">
            "Apple"
        </combobox::Item>
    </combobox::Content>
</combobox::Root>"#;

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
        description: "Where the dropdown panel appears relative to the trigger.",
    },
    PropRow {
        name: "hide_delay",
        prop_type: "Duration",
        default: "200ms",
        description: "Delay before the panel is removed from the DOM after closing.",
    },
    PropRow {
        name: "avoid_collisions",
        prop_type: "AvoidCollisions",
        default: "Flip",
        description: "Strategy used to keep the panel within the viewport.",
    },
    PropRow {
        name: "on_value_change",
        prop_type: "Option<Callback<String>>",
        default: "None",
        description: "Fired with the selected value when selection changes.",
    },
    PropRow {
        name: "inline",
        prop_type: "bool",
        default: "false",
        description: "Set to true when using InputTrigger (the input sits above the dropdown).",
    },
];

const TRIGGER_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the trigger button.",
}];

const INPUT_TRIGGER_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the inline input trigger.",
    },
    PropRow {
        name: "placeholder",
        prop_type: "String",
        default: "\"\"",
        description: "Placeholder text shown when nothing is selected.",
    },
];

const VALUE_PROPS: &[PropRow] = &[PropRow {
    name: "placeholder",
    prop_type: "String",
    default: "\"\"",
    description: "Text shown when no item is selected.",
}];

const CONTENT_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the floating dropdown panel.",
    },
    PropRow {
        name: "show_class",
        prop_type: "String",
        default: "\"\"",
        description: "Extra class added while the panel is visible.",
    },
    PropRow {
        name: "hide_class",
        prop_type: "String",
        default: "\"\"",
        description: "Extra class added while the panel is animating out.",
    },
];

const INPUT_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the search input.",
    },
    PropRow {
        name: "placeholder",
        prop_type: "String",
        default: "\"\"",
        description: "Placeholder text for the search input.",
    },
];

const ITEM_PROPS: &[PropRow] = &[
    PropRow {
        name: "value",
        prop_type: "String",
        default: "—",
        description: "The value submitted when this item is selected.",
    },
    PropRow {
        name: "label",
        prop_type: "Option<String>",
        default: "value",
        description: "Display label used for filtering and shown in the trigger. Defaults to value.",
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
        description: "When true, the item cannot be selected.",
    },
];

const DATA_ATTRS: &[DataAttrRow] = &[
    DataAttrRow {
        name: "data-state",
        description: "\"open\" or \"closed\". Present on Trigger.",
    },
    DataAttrRow {
        name: "data-state",
        description: "\"checked\" or \"unchecked\". Present on Item.",
    },
    DataAttrRow {
        name: "data-highlighted",
        description: "Present on Item when it has keyboard focus or the mouse is over it.",
    },
    DataAttrRow {
        name: "data-disabled",
        description: "Present on Item when it is disabled.",
    },
];

const KEYBOARD: &[KeyboardRow] = &[
    KeyboardRow {
        key: "ArrowDown",
        description: "Moves focus to the next visible item.",
    },
    KeyboardRow {
        key: "ArrowUp",
        description: "Moves focus to the previous visible item.",
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
        description: "Selects the focused item and closes the dropdown.",
    },
    KeyboardRow {
        key: "Escape",
        description: "Closes the dropdown and returns focus to the trigger.",
    },
    KeyboardRow {
        key: "Tab",
        description: "Closes the dropdown.",
    },
];

#[component]
pub fn ComboboxDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    view! {
        <DocPage title="Combobox">
            <p class="mt-3 mb-11 text-base text-balance">
                "A searchable input that filters a list of options. Type to narrow results, navigate with arrow keys, and select with Enter or click."
            </p>
            <DocPreview>
                <ComboboxInlineExample />
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
            <SectionHeading title="Button Trigger Variant" />
            <p class="mb-4 text-sm text-muted-foreground">
                "For a select-style trigger with a hidden search field that appears inside the dropdown, omit "
                <code class="font-mono text-foreground">{"inline"}</code>
                " and use " <code class="font-mono text-foreground">{"Trigger"}</code>
                " + " <code class="font-mono text-foreground">{"Value"}</code>
                " + " <code class="font-mono text-foreground">{"Input"}</code>
                " (inside Content) instead."
            </p>
            <Code
                class="[&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={USAGE_CODE_BUTTON}
                language="rust"
            />
            <DocPreview>
                <ComboboxExample />
            </DocPreview>
            <SectionHeading title="API Reference" />
            <PropsTable title="Root" rows={ROOT_PROPS} />
            <PropsTable title="InputTrigger" rows={INPUT_TRIGGER_PROPS} />
            <PropsTable title="Trigger" rows={TRIGGER_PROPS} />
            <PropsTable title="Value" rows={VALUE_PROPS} />
            <PropsTable title="Content" rows={CONTENT_PROPS} />
            <PropsTable title="Input" rows={INPUT_PROPS} />
            <PropsTable title="Item" rows={ITEM_PROPS} />
            <DataAttrsTable rows={DATA_ATTRS} />
            <KeyboardTable rows={KEYBOARD} />
        </DocPage>
    }
}

#[component]
pub fn ComboboxExample() -> impl IntoView {
    use biji_ui::components::combobox;

    const TRIGGER_CLS: &str = "flex items-center justify-between w-56 px-3 py-2 text-sm \
        rounded-md border border-border bg-background text-foreground \
        hover:bg-accent hover:text-accent-foreground \
        data-[state=open]:bg-accent data-[state=open]:text-accent-foreground \
        cursor-default select-none";

    const CONTENT_CLS: &str = "z-50 w-56 overflow-hidden rounded-md border border-border \
        bg-background shadow-md text-sm \
        transition origin-[var(--biji-transform-origin)]";

    const INPUT_CLS: &str = "w-full px-3 py-2 text-sm outline-none bg-transparent \
        border-b border-border placeholder:text-muted-foreground";

    const ITEM_CLS: &str = "flex items-center px-3 py-1.5 cursor-default select-none outline-none \
        hover:bg-accent hover:text-accent-foreground \
        data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground \
        data-[disabled]:pointer-events-none data-[disabled]:opacity-50";

    let last_value = RwSignal::new(String::from("None"));

    let fruits = [
        ("apple", "Apple"),
        ("apricot", "Apricot"),
        ("avocado", "Avocado"),
        ("banana", "Banana"),
        ("blueberry", "Blueberry"),
        ("cherry", "Cherry"),
        ("grape", "Grape"),
        ("kiwi", "Kiwi"),
        ("lemon", "Lemon"),
        ("mango", "Mango"),
        ("orange", "Orange"),
        ("peach", "Peach"),
        ("pear", "Pear"),
        ("pineapple", "Pineapple"),
        ("strawberry", "Strawberry"),
    ];

    view! {
        <div class="flex flex-col items-center gap-3">
            <combobox::Root
                on_value_change={Callback::new(move |v: String| last_value.set(v))}
            >
                <combobox::Trigger class={TRIGGER_CLS}>
                    <combobox::Value placeholder="Select a fruit..." />
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="14"
                        height="14"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="text-muted-foreground"
                    >
                        <path d="m7 15 5 5 5-5"/>
                        <path d="m7 9 5-5 5 5"/>
                    </svg>
                </combobox::Trigger>
                <combobox::Content
                    class={CONTENT_CLS}
                    show_class="opacity-100 scale-100"
                    hide_class="opacity-0 scale-95"
                >
                    <combobox::Input
                        class={INPUT_CLS}
                        placeholder="Search fruit..."
                    />
                    <div class="max-h-60 overflow-y-auto py-1">
                        <combobox::Empty>
                            <div class="px-3 py-6 text-center text-sm text-muted-foreground">
                                "No fruit found."
                            </div>
                        </combobox::Empty>
                        {fruits
                            .into_iter()
                            .map(|(value, label)| {
                                view! {
                                    <combobox::Item
                                        value={value}
                                        label={label}
                                        class={ITEM_CLS}
                                    >
                                        <span class="flex-1">{label}</span>
                                        <combobox::ItemIndicator>
                                            <svg
                                                xmlns="http://www.w3.org/2000/svg"
                                                width="14"
                                                height="14"
                                                viewBox="0 0 24 24"
                                                fill="none"
                                                stroke="currentColor"
                                                stroke-width="2.5"
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                            >
                                                <path d="M20 6 9 17l-5-5"/>
                                            </svg>
                                        </combobox::ItemIndicator>
                                    </combobox::Item>
                                }
                            })
                            .collect_view()}
                    </div>
                </combobox::Content>
            </combobox::Root>
            <p class="text-xs text-muted-foreground">
                "Selected: " <span class="font-medium text-foreground">{move || last_value.get()}</span>
            </p>
        </div>
    }
}

#[component]
pub fn ComboboxInlineExample() -> impl IntoView {
    use biji_ui::components::combobox;

    const INPUT_TRIGGER_CLS: &str = "w-56 px-3 py-2 text-sm rounded-md border border-border \
        bg-background text-foreground outline-none \
        focus:ring-2 focus:ring-ring focus:ring-offset-0 \
        placeholder:text-muted-foreground";

    const CONTENT_CLS: &str = "z-50 w-56 overflow-hidden rounded-md border border-border \
        bg-background shadow-md text-sm \
        transition origin-[var(--biji-transform-origin)]";

    const ITEM_CLS: &str = "flex items-center px-3 py-1.5 cursor-default select-none outline-none \
        hover:bg-accent hover:text-accent-foreground \
        data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground \
        data-[disabled]:pointer-events-none data-[disabled]:opacity-50";

    let last_value = RwSignal::new(String::from("None"));

    let fruits = [
        ("apple", "Apple"),
        ("apricot", "Apricot"),
        ("avocado", "Avocado"),
        ("banana", "Banana"),
        ("blueberry", "Blueberry"),
        ("cherry", "Cherry"),
        ("grape", "Grape"),
        ("kiwi", "Kiwi"),
        ("lemon", "Lemon"),
        ("mango", "Mango"),
        ("orange", "Orange"),
        ("peach", "Peach"),
        ("pear", "Pear"),
        ("pineapple", "Pineapple"),
        ("strawberry", "Strawberry"),
    ];

    view! {
        <div class="flex flex-col items-center gap-3">
            <combobox::Root
                inline=true
                on_value_change={Callback::new(move |v: String| last_value.set(v))}
            >
                <combobox::InputTrigger
                    class={INPUT_TRIGGER_CLS}
                    placeholder="Search a fruit..."
                />
                <combobox::Content
                    class={CONTENT_CLS}
                    show_class="opacity-100 scale-100"
                    hide_class="opacity-0 scale-95"
                >
                    <div class="max-h-60 overflow-y-auto py-1">
                        <combobox::Empty>
                            <div class="px-3 py-6 text-center text-sm text-muted-foreground">
                                "No fruit found."
                            </div>
                        </combobox::Empty>
                        {fruits
                            .into_iter()
                            .map(|(value, label)| {
                                view! {
                                    <combobox::Item
                                        value={value}
                                        label={label}
                                        class={ITEM_CLS}
                                    >
                                        <span class="flex-1">{label}</span>
                                        <combobox::ItemIndicator>
                                            <svg
                                                xmlns="http://www.w3.org/2000/svg"
                                                width="14"
                                                height="14"
                                                viewBox="0 0 24 24"
                                                fill="none"
                                                stroke="currentColor"
                                                stroke-width="2.5"
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                            >
                                                <path d="M20 6 9 17l-5-5"/>
                                            </svg>
                                        </combobox::ItemIndicator>
                                    </combobox::Item>
                                }
                            })
                            .collect_view()}
                    </div>
                </combobox::Content>
            </combobox::Root>
            <p class="text-xs text-muted-foreground">
                "Selected: " <span class="font-medium text-foreground">{move || last_value.get()}</span>
            </p>
        </div>
    }
}
