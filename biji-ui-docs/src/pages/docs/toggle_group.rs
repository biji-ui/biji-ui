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
    "\", features = [\"toggle_group\"] }",
);

const USAGE_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::toggle_group::{self, ToggleGroupType};

#[component]
pub fn MyToggleGroup() -> impl IntoView {
    view! {
        <toggle_group::Root
            group_type={ToggleGroupType::Single}
            class="inline-flex rounded-md border border-border"
        >
            <toggle_group::Item value="left" class="px-3 py-2 text-sm data-[state=on]:bg-accent">
                "Left"
            </toggle_group::Item>
            <toggle_group::Item value="center" class="px-3 py-2 text-sm data-[state=on]:bg-accent">
                "Center"
            </toggle_group::Item>
            <toggle_group::Item value="right" class="px-3 py-2 text-sm data-[state=on]:bg-accent">
                "Right"
            </toggle_group::Item>
        </toggle_group::Root>
    }
}"#;

const ROOT_WITH_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::toggle_group::{self, ToggleGroupType};

#[component]
pub fn AlignmentPicker() -> impl IntoView {
    let item_class = "px-3 py-2 text-sm font-medium transition-colors \
        first:rounded-l-md last:rounded-r-md border-r border-border last:border-r-0 \
        data-[state=on]:bg-accent data-[state=on]:text-accent-foreground";

    view! {
        <toggle_group::RootWith
            group_type={ToggleGroupType::Single}
            value="center"
            class="inline-flex rounded-md border border-border"
            let:tg
        >
            <toggle_group::Item value="left" class={item_class}>"Left"</toggle_group::Item>
            <toggle_group::Item value="center" class={item_class}>"Center"</toggle_group::Item>
            <toggle_group::Item value="right" class={item_class}>"Right"</toggle_group::Item>
            <p class="text-xs text-muted-foreground w-full text-center mt-2">
                "Selected: " {move || tg.value.with(|v| v.first().cloned().unwrap_or_default())}
            </p>
        </toggle_group::RootWith>
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
        name: "group_type",
        prop_type: "ToggleGroupType",
        default: "Single",
        description: "Whether one or multiple items can be pressed at a time.",
    },
    PropRow {
        name: "value",
        prop_type: "Option<String>",
        default: "None",
        description: "Initial pressed value for Single mode.",
    },
    PropRow {
        name: "values",
        prop_type: "Option<Vec<String>>",
        default: "None",
        description: "Initial pressed values for Multiple mode.",
    },
    PropRow {
        name: "disabled",
        prop_type: "bool",
        default: "false",
        description: "When true, all items are disabled.",
    },
];

const ITEM_PROPS: &[PropRow] = &[
    PropRow {
        name: "value",
        prop_type: "String",
        default: "",
        description: "The value associated with this item.",
    },
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the button element.",
    },
    PropRow {
        name: "disabled",
        prop_type: "bool",
        default: "false",
        description: "When true, this item cannot be toggled.",
    },
];

const TYPE_PROPS: &[PropRow] = &[
    PropRow {
        name: "Single",
        prop_type: "ToggleGroupType",
        default: "default",
        description: "At most one item can be pressed; pressing the active item deselects it.",
    },
    PropRow {
        name: "Multiple",
        prop_type: "ToggleGroupType",
        default: "",
        description: "Any number of items can be pressed simultaneously.",
    },
];

const DATA_ATTRS: &[DataAttrRow] = &[
    DataAttrRow {
        name: "data-state",
        description: "\"on\" when the item is pressed, \"off\" otherwise. Present on Item.",
    },
    DataAttrRow {
        name: "data-disabled",
        description: "Present on Item when the item or the Root is disabled.",
    },
];

const KEYBOARD: &[KeyboardRow] = &[
    KeyboardRow {
        key: "ArrowRight / ArrowDown",
        description: "Moves focus to the next item (wraps).",
    },
    KeyboardRow {
        key: "ArrowLeft / ArrowUp",
        description: "Moves focus to the previous item (wraps).",
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
        key: "Enter / Space",
        description: "Toggles the focused item.",
    },
];

#[component]
pub fn ToggleGroupDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    view! {
        <DocPage title="Toggle Group">
            <p class="mt-3 mb-11 text-base text-balance">
                "A set of toggle buttons where one or more can be pressed at a time."
            </p>
            <DocPreview>
                <ToggleGroupExample />
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
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"ToggleGroupState"</code>
                " inside the children. The "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"let:tg"</code>
                " binding exposes "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"tg.value"</code>
                " as a reactive signal for reading the current selection without callbacks."
            </p>
            <DocPreview>
                <ToggleGroupRootWithExample />
            </DocPreview>
            <Code
                class="mt-4 [&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={ROOT_WITH_CODE}
                language="rust"
            />
            <SectionHeading title="API Reference" />
            <PropsTable title="Root / RootWith" rows={ROOT_PROPS} />
            <PropsTable title="Item" rows={ITEM_PROPS} />
            <PropsTable title="ToggleGroupType" rows={TYPE_PROPS} />
            <DataAttrsTable rows={DATA_ATTRS} />
            <KeyboardTable rows={KEYBOARD} />
        </DocPage>
    }
}

#[component]
pub fn ToggleGroupRootWithExample() -> impl IntoView {
    use biji_ui::components::toggle_group::{self, ToggleGroupType};

    const ITEM_CLS: &str = "px-3 py-2 text-sm font-medium transition-colors \
        first:rounded-l-md last:rounded-r-md border-r border-border last:border-r-0 \
        hover:bg-accent hover:text-accent-foreground \
        data-[state=on]:bg-accent data-[state=on]:text-accent-foreground \
        data-[disabled]:pointer-events-none data-[disabled]:opacity-50 \
        focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring";

    view! {
        <div class="flex flex-col items-center gap-6">
            <div class="flex flex-col items-center gap-2">
                <p class="text-xs text-muted-foreground">"Alignment (single)"</p>
                <toggle_group::RootWith
                    group_type={ToggleGroupType::Single}
                    value="center"
                    class="flex flex-col items-center gap-2"
                    let:tg
                >
                    <div class="inline-flex rounded-md border border-border">
                        <toggle_group::Item value="left" class={ITEM_CLS}>
                            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <line x1="21" y1="6" x2="3" y2="6"/><line x1="15" y1="12" x2="3" y2="12"/>
                                <line x1="17" y1="18" x2="3" y2="18"/>
                            </svg>
                        </toggle_group::Item>
                        <toggle_group::Item value="center" class={ITEM_CLS}>
                            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <line x1="21" y1="6" x2="3" y2="6"/><line x1="17" y1="12" x2="7" y2="12"/>
                                <line x1="19" y1="18" x2="5" y2="18"/>
                            </svg>
                        </toggle_group::Item>
                        <toggle_group::Item value="right" class={ITEM_CLS}>
                            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <line x1="21" y1="6" x2="3" y2="6"/><line x1="21" y1="12" x2="9" y2="12"/>
                                <line x1="21" y1="18" x2="7" y2="18"/>
                            </svg>
                        </toggle_group::Item>
                    </div>
                    <p class="text-xs text-muted-foreground">
                        "Selected: " {move || tg.value.with(|v| v.first().cloned().unwrap_or_default())}
                    </p>
                </toggle_group::RootWith>
            </div>

            <div class="flex flex-col items-center gap-2">
                <p class="text-xs text-muted-foreground">"Formatting (multiple)"</p>
                <toggle_group::RootWith
                    group_type={ToggleGroupType::Multiple}
                    values={vec!["bold".to_string()]}
                    class="flex flex-col items-center gap-2"
                    let:tg
                >
                    <div class="inline-flex rounded-md border border-border">
                        <toggle_group::Item value="bold" class={ITEM_CLS}>
                            <span class="font-bold text-sm">"B"</span>
                        </toggle_group::Item>
                        <toggle_group::Item value="italic" class={ITEM_CLS}>
                            <span class="italic text-sm">"I"</span>
                        </toggle_group::Item>
                        <toggle_group::Item value="underline" class={ITEM_CLS}>
                            <span class="underline text-sm">"U"</span>
                        </toggle_group::Item>
                    </div>
                    <p class="text-xs text-muted-foreground">
                        "Active: " {move || tg.value.with(|v| if v.is_empty() { "none".to_string() } else { v.join(", ") })}
                    </p>
                </toggle_group::RootWith>
            </div>
        </div>
    }
}

#[component]
pub fn ToggleGroupExample() -> impl IntoView {
    use biji_ui::components::toggle_group::{self, ToggleGroupType};

    const ITEM_CLS: &str = "px-3 py-2 text-sm font-medium transition-colors \
        first:rounded-l-md last:rounded-r-md border-r border-border last:border-r-0 \
        hover:bg-accent hover:text-accent-foreground \
        data-[state=on]:bg-accent data-[state=on]:text-accent-foreground \
        data-[disabled]:pointer-events-none data-[disabled]:opacity-50 \
        focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring";

    view! {
        <div class="flex flex-col items-center gap-6">
            <div class="flex flex-col items-center gap-2">
                <p class="text-xs text-muted-foreground">"Alignment (single)"</p>
                <toggle_group::Root
                    group_type={ToggleGroupType::Single}
                    value="center"
                    class="inline-flex rounded-md border border-border"
                >
                    <toggle_group::Item value="left" class={ITEM_CLS}>
                        <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <line x1="21" y1="6" x2="3" y2="6"/><line x1="15" y1="12" x2="3" y2="12"/>
                            <line x1="17" y1="18" x2="3" y2="18"/>
                        </svg>
                    </toggle_group::Item>
                    <toggle_group::Item value="center" class={ITEM_CLS}>
                        <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <line x1="21" y1="6" x2="3" y2="6"/><line x1="17" y1="12" x2="7" y2="12"/>
                            <line x1="19" y1="18" x2="5" y2="18"/>
                        </svg>
                    </toggle_group::Item>
                    <toggle_group::Item value="right" class={ITEM_CLS}>
                        <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <line x1="21" y1="6" x2="3" y2="6"/><line x1="21" y1="12" x2="9" y2="12"/>
                            <line x1="21" y1="18" x2="7" y2="18"/>
                        </svg>
                    </toggle_group::Item>
                </toggle_group::Root>
            </div>

            <div class="flex flex-col items-center gap-2">
                <p class="text-xs text-muted-foreground">"Formatting (multiple)"</p>
                <toggle_group::Root
                    group_type={ToggleGroupType::Multiple}
                    values={vec!["bold".to_string()]}
                    class="inline-flex rounded-md border border-border"
                >
                    <toggle_group::Item value="bold" class={ITEM_CLS}>
                        <span class="font-bold text-sm">"B"</span>
                    </toggle_group::Item>
                    <toggle_group::Item value="italic" class={ITEM_CLS}>
                        <span class="italic text-sm">"I"</span>
                    </toggle_group::Item>
                    <toggle_group::Item value="underline" class={ITEM_CLS}>
                        <span class="underline text-sm">"U"</span>
                    </toggle_group::Item>
                </toggle_group::Root>
            </div>
        </div>
    }
}
