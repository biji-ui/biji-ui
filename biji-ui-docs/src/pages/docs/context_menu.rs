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
    "\", features = [\"context_menu\"] }",
);

const USAGE_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::context_menu;

#[component]
pub fn MyContextMenu() -> impl IntoView {
    view! {
        <context_menu::Root>
            <context_menu::Trigger class="flex h-32 w-48 items-center justify-center rounded-md border border-dashed border-border text-sm text-muted-foreground">
                "Right-click here"
            </context_menu::Trigger>
            <context_menu::Content class="z-50 min-w-[8rem] overflow-hidden rounded-md border border-border bg-background py-1 shadow-md text-sm">
                <context_menu::Item
                    class="px-3 py-1.5 cursor-default select-none outline-none hover:bg-accent data-[highlighted]:bg-accent"
                    on_select={Callback::new(|_| { /* handle action */ })}
                >
                    "Copy"
                </context_menu::Item>
                <context_menu::Item
                    class="px-3 py-1.5 cursor-default select-none outline-none hover:bg-accent data-[highlighted]:bg-accent"
                >
                    "Paste"
                </context_menu::Item>
                <context_menu::Separator class="my-1 border-t border-border" />
                <context_menu::Item
                    class="px-3 py-1.5 cursor-default select-none outline-none hover:bg-accent data-[highlighted]:bg-accent text-destructive"
                    on_select={Callback::new(|_| { /* handle action */ })}
                >
                    "Delete"
                </context_menu::Item>
            </context_menu::Content>
        </context_menu::Root>
    }
}"#;

const ROOT_WITH_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::context_menu;

#[component]
pub fn MyContextMenu() -> impl IntoView {
    view! {
        <context_menu::RootWith let:m>
            <p class="text-sm text-muted-foreground">
                {move || if m.open.get() { "Menu is open" } else { "Right-click to open" }}
            </p>
            <context_menu::Trigger class="flex h-24 w-48 items-center justify-center rounded-md border border-dashed border-border text-sm text-muted-foreground">
                "Right-click here"
            </context_menu::Trigger>
            <context_menu::Content class="z-50 min-w-[8rem] overflow-hidden rounded-md border border-border bg-background py-1 shadow-md text-sm">
                <context_menu::Item class="px-3 py-1.5 cursor-default select-none outline-none hover:bg-accent">
                    "Copy"
                </context_menu::Item>
                <context_menu::Item class="px-3 py-1.5 cursor-default select-none outline-none hover:bg-accent">
                    "Paste"
                </context_menu::Item>
            </context_menu::Content>
        </context_menu::RootWith>
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
        name: "allow_loop",
        prop_type: "bool",
        default: "true",
        description: "Whether keyboard navigation wraps around at the ends.",
    },
    PropRow {
        name: "hide_delay",
        prop_type: "Duration",
        default: "200ms",
        description: "Delay before the panel is removed from the DOM after closing.",
    },
    PropRow {
        name: "on_open_change",
        prop_type: "Option<Callback<bool>>",
        default: "None",
        description: "Fired with true when the menu opens and false when it closes.",
    },
];

const TRIGGER_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the trigger div.",
}];

const CONTENT_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the floating menu panel.",
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

const ITEM_PROPS: &[PropRow] = &[
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
    PropRow {
        name: "on_select",
        prop_type: "Option<Callback<()>>",
        default: "None",
        description: "Fired when the item is selected by click or Enter/Space.",
    },
];

const DATA_ATTRS: &[DataAttrRow] = &[
    DataAttrRow {
        name: "data-state",
        description: "\"open\" or \"closed\". Present on Trigger.",
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
        description: "Moves focus to the next item.",
    },
    KeyboardRow {
        key: "ArrowUp",
        description: "Moves focus to the previous item.",
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
        description: "Activates the focused item and closes the menu.",
    },
    KeyboardRow {
        key: "Escape",
        description: "Closes the menu.",
    },
    KeyboardRow {
        key: "Tab",
        description: "Closes the menu.",
    },
];

#[component]
pub fn ContextMenuDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    view! {
        <DocPage title="Context Menu">
            <p class="mt-3 mb-11 text-base text-balance">
                "A menu that appears at the pointer position in response to a right-click."
            </p>
            <DocPreview>
                <ContextMenuExample />
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
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"ContextMenuState"</code>
                " inline via the "
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"let:"</code>
                " binding. The state exposes "
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"open"</code>
                ", "
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"pointer_x"</code>
                ", and "
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"pointer_y"</code>
                " as reactive signals."
            </p>
            <DocPreview>
                <ContextMenuRootWithExample />
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
            <PropsTable title="Item" rows={ITEM_PROPS} />
            <DataAttrsTable rows={DATA_ATTRS} />
            <KeyboardTable rows={KEYBOARD} />
        </DocPage>
    }
}

#[component]
pub fn ContextMenuRootWithExample() -> impl IntoView {
    use biji_ui::components::context_menu;

    const CONTENT_CLS: &str = "z-50 min-w-[10rem] overflow-hidden rounded-md border border-border \
        bg-background py-1 shadow-md text-sm \
        transition origin-[var(--biji-transform-origin)]";

    const ITEM_CLS: &str = "flex items-center px-3 py-1.5 cursor-default select-none outline-none \
        hover:bg-accent hover:text-accent-foreground \
        data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground";

    view! {
        <div class="flex flex-col items-center gap-3 p-8">
            <context_menu::RootWith let:m>
                <p class="text-sm text-muted-foreground">
                    {move || if m.open.get() { "Menu is open" } else { "Right-click to open" }}
                </p>
                <context_menu::Trigger class="flex h-24 w-56 items-center justify-center rounded-md border border-dashed border-border text-sm text-muted-foreground select-none cursor-default">
                    "Right-click here"
                </context_menu::Trigger>
                <context_menu::Content
                    class={CONTENT_CLS}
                    show_class="opacity-100 scale-100"
                    hide_class="opacity-0 scale-95"
                >
                    <context_menu::Item class={ITEM_CLS}>"Copy"</context_menu::Item>
                    <context_menu::Item class={ITEM_CLS}>"Paste"</context_menu::Item>
                    <context_menu::Separator class="my-1 border-t border-border" />
                    <context_menu::Item class={ITEM_CLS}>"Delete"</context_menu::Item>
                </context_menu::Content>
            </context_menu::RootWith>
        </div>
    }
}

#[component]
pub fn ContextMenuExample() -> impl IntoView {
    use biji_ui::components::context_menu;

    const CONTENT_CLS: &str = "z-50 min-w-[10rem] overflow-hidden rounded-md border border-border \
        bg-background py-1 shadow-md text-sm \
        transition origin-[var(--biji-transform-origin)]";

    const ITEM_CLS: &str = "flex items-center px-3 py-1.5 cursor-default select-none outline-none \
        hover:bg-accent hover:text-accent-foreground \
        focus:bg-accent focus:text-accent-foreground \
        data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground \
        data-[disabled]:pointer-events-none data-[disabled]:opacity-50";

    let last_action = RwSignal::new(String::from("None"));

    view! {
        <div class="flex flex-col items-center gap-3">
            <context_menu::Root>
                <context_menu::Trigger class="flex h-32 w-56 items-center justify-center rounded-md border border-dashed border-border text-sm text-muted-foreground select-none cursor-default">
                    "Right-click here"
                </context_menu::Trigger>
                <context_menu::Content
                    class={CONTENT_CLS}
                    show_class="opacity-100 scale-100"
                    hide_class="opacity-0 scale-95"
                >
                    <context_menu::Label class="px-3 py-1 text-xs font-medium text-muted-foreground">
                        "Actions"
                    </context_menu::Label>
                    <context_menu::Item
                        class={ITEM_CLS}
                        on_select={Callback::new(move |_| last_action.set("Copy".to_string()))}
                    >
                        "Copy"
                    </context_menu::Item>
                    <context_menu::Item
                        class={ITEM_CLS}
                        on_select={Callback::new(move |_| last_action.set("Cut".to_string()))}
                    >
                        "Cut"
                    </context_menu::Item>
                    <context_menu::Item
                        class={ITEM_CLS}
                        on_select={Callback::new(move |_| last_action.set("Paste".to_string()))}
                    >
                        "Paste"
                    </context_menu::Item>
                    <context_menu::Separator class="my-1 border-t border-border" />
                    <context_menu::Item
                        class={ITEM_CLS}
                        disabled=true
                    >
                        "Share (disabled)"
                    </context_menu::Item>
                    <context_menu::Separator class="my-1 border-t border-border" />
                    <context_menu::Item
                        class={format!("{ITEM_CLS} text-error")}
                        on_select={Callback::new(move |_| last_action.set("Delete".to_string()))}
                    >
                        "Delete"
                    </context_menu::Item>
                </context_menu::Content>
            </context_menu::Root>
            <p class="text-xs text-muted-foreground">
                "Last action: " <span class="font-medium text-foreground">{move || last_action.get()}</span>
            </p>
        </div>
    }
}
