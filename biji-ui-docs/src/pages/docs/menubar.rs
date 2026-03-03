use std::time::Duration;

use biji_ui::cn;
use leptos::{logging::log, portal::Portal, prelude::*};

use crate::{
    components::{
        api_table::{
            DataAttrRow, DataAttrsTable, KeyboardRow, KeyboardTable, PropRow, PropsTable,
            SectionHeading,
        },
        code::Code,
    },
    icons,
};

const USAGE_CODE: &str = r#"use std::time::Duration;
use leptos::{portal::Portal, prelude::*};
use biji_ui::components::menubar;

#[component]
pub fn MyMenubar() -> impl IntoView {
    view! {
        <menubar::Root class="flex" allow_item_loop=true>
            <menubar::Menu
                class="relative"
                positioning={menubar::Positioning::BottomStart}
                hide_delay={Duration::from_millis(200)}
            >
                <menubar::Trigger class="py-1.5 px-2 text-sm rounded-sm hover:bg-accent">
                    "File"
                </menubar::Trigger>
                <Portal>
                    <menubar::Content
                        class="flex flex-col p-1 w-56 rounded-md border shadow-md bg-background"
                        show_class="opacity-100 transition duration-150 ease-in"
                        hide_class="opacity-0 transition duration-200 ease-out"
                    >
                        <menubar::Item class="py-1.5 px-2 text-sm rounded-sm data-[highlighted]:bg-muted hover:bg-accent">
                            "New file"
                        </menubar::Item>
                        <menubar::Item
                            disabled=true
                            class="py-1.5 px-2 text-sm rounded-sm data-[disabled]:opacity-50 data-[disabled]:pointer-events-none"
                        >
                            "New window"
                        </menubar::Item>
                    </menubar::Content>
                </Portal>
            </menubar::Menu>
            <menubar::Menu
                class="relative"
                positioning={menubar::Positioning::BottomStart}
                hide_delay={Duration::from_millis(200)}
            >
                <menubar::Trigger class="py-1.5 px-2 text-sm rounded-sm hover:bg-accent">
                    "Edit"
                </menubar::Trigger>
                <Portal>
                    <menubar::Content
                        class="flex flex-col p-1 w-56 rounded-md border shadow-md bg-background"
                        show_class="opacity-100 transition duration-150 ease-in"
                        hide_class="opacity-0 transition duration-200 ease-out"
                    >
                        <menubar::Item class="py-1.5 px-2 text-sm rounded-sm data-[highlighted]:bg-muted hover:bg-accent">
                            "Undo"
                        </menubar::Item>
                        <menubar::Item class="py-1.5 px-2 text-sm rounded-sm data-[highlighted]:bg-muted hover:bg-accent">
                            "Redo"
                        </menubar::Item>
                    </menubar::Content>
                </Portal>
            </menubar::Menu>
        </menubar::Root>
    }
}"#;

const ROOT_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the menubar root element.",
    },
    PropRow {
        name: "allow_menu_loop",
        prop_type: "bool",
        default: "false",
        description: "When true, horizontal keyboard navigation wraps from the last menu back to the first and vice versa.",
    },
    PropRow {
        name: "allow_item_loop",
        prop_type: "bool",
        default: "false",
        description: "When true, vertical keyboard navigation within a menu wraps from the last item back to the first.",
    },
    PropRow {
        name: "prevent_scroll",
        prop_type: "bool",
        default: "false",
        description: "When true, prevents the page from scrolling while any menu is open.",
    },
];

const MENU_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the individual menu container.",
    },
    PropRow {
        name: "disabled",
        prop_type: "bool",
        default: "false",
        description: "Disables the menu trigger when true.",
    },
    PropRow {
        name: "positioning",
        prop_type: "Positioning",
        default: "BottomStart",
        description: "Where to position the menu content relative to its trigger.",
    },
    PropRow {
        name: "hide_delay",
        prop_type: "Duration",
        default: "200ms",
        description: "How long to wait before unmounting the content after closing begins. Should match your CSS transition duration.",
    },
];

const TRIGGER_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the menu trigger element.",
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
        description: "CSS class applied when the menu is open.",
    },
    PropRow {
        name: "hide_class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied while the menu is closing.",
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
        description: "Prevents interaction with the item and applies data-disabled.",
    },
];

const SUBMENU_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the submenu wrapper element.",
    },
    PropRow {
        name: "disabled",
        prop_type: "bool",
        default: "false",
        description: "Disables the submenu trigger and prevents it from opening.",
    },
    PropRow {
        name: "positioning",
        prop_type: "Positioning",
        default: "RightStart",
        description: "Where to position the submenu content.",
    },
    PropRow {
        name: "hide_delay",
        prop_type: "Duration",
        default: "200ms",
        description: "How long to wait before unmounting the submenu content after closing begins.",
    },
];

const SUBMENU_TRIGGER_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the submenu trigger element.",
}];

const DATA_ATTRS: &[DataAttrRow] = &[
    DataAttrRow {
        name: "data-highlighted",
        description: "Present on Trigger, Item, and SubMenuTrigger when they have keyboard focus or are hovered.",
    },
    DataAttrRow {
        name: "data-disabled",
        description: "Present on Trigger, Item, and SubMenuTrigger when disabled is true.",
    },
    DataAttrRow {
        name: "data-open",
        description: "Present on Trigger with value true when its menu is open.",
    },
];

const KEYBOARD: &[KeyboardRow] = &[
    KeyboardRow {
        key: "ArrowLeft",
        description: "Moves focus to the previous menu trigger in the bar (follows the open state).",
    },
    KeyboardRow {
        key: "ArrowRight",
        description: "Moves focus to the next menu trigger in the bar (follows the open state).",
    },
    KeyboardRow {
        key: "ArrowDown",
        description: "Opens the focused menu and moves focus to its first item.",
    },
    KeyboardRow {
        key: "ArrowUp",
        description: "Opens the focused menu and moves focus to its last item.",
    },
    KeyboardRow {
        key: "Enter",
        description: "Opens the focused menu trigger and focuses the first item.",
    },
    KeyboardRow {
        key: "Escape",
        description: "Closes all open menus and returns focus to the menubar.",
    },
];

#[component]
pub fn MenubarDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    view! {
        <DocPage title="Menubar">
            <p class="mt-3 mb-11 text-base text-balance">
                "Displays a menu to the user, which can consist of links or functions, triggered by a button."
            </p>
            <DocPreview>
                <MenubarExample />
            </DocPreview>
            <SectionHeading title="Usage" />
            <Code
                class="[&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={USAGE_CODE}
                language="rust"
            />
            <SectionHeading title="API Reference" />
            <PropsTable title="Root" rows={ROOT_PROPS} />
            <PropsTable title="Menu" rows={MENU_PROPS} />
            <PropsTable title="Trigger" rows={TRIGGER_PROPS} />
            <PropsTable title="Content" rows={CONTENT_PROPS} />
            <PropsTable title="Item" rows={ITEM_PROPS} />
            <PropsTable title="SubMenu" rows={SUBMENU_PROPS} />
            <PropsTable title="SubMenuTrigger" rows={SUBMENU_TRIGGER_PROPS} />
            <PropsTable title="SubMenuContent" rows={CONTENT_PROPS} />
            <DataAttrsTable rows={DATA_ATTRS} />
            <KeyboardTable rows={KEYBOARD} />
        </DocPage>
    }
}

#[component]
pub fn MenubarExample() -> impl IntoView {
    use biji_ui::components::menubar;

    const ITEM_STYLE: &str = "flex items-center py-1.5 px-2 text-sm rounded-sm cursor-pointer outline-none select-none focus:outline-none hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[disabled]:pointer-events-none data-[disabled]:opacity-50 data-[highlighted]:bg-muted";

    view! {
        <menubar::Root class="flex" allow_item_loop=true allow_menu_loop=true>
            <menubar::Menu
                class="relative"
                positioning={menubar::Positioning::BottomStart}
                hide_delay={Duration::from_millis(200)}
            >
                <menubar::Trigger class="flex items-center py-1.5 px-2 text-sm rounded-sm cursor-pointer outline-none select-none focus:outline-none hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[disabled]:pointer-events-none data-[disabled]:opacity-50 data-[highlighted]:bg-muted">
                    "File"
                </menubar::Trigger>
                <Portal>
                    <menubar::Content
                        class="flex flex-col p-1 w-56 rounded-md border shadow-md focus:outline-none border-border min-w-[8rem] bg-background text-foreground"
                        show_class="z-10 opacity-100 transition duration-150 ease-in"
                        hide_class="opacity-0 transition duration-200 ease-out -z-10"
                    >
                        <menubar::Item class={ITEM_STYLE}>
                            <button
                                class="flex w-full h-full"
                                on:click={|_| log!("New file clicked")}
                            >
                                "New file..."
                            </button>
                        </menubar::Item>
                        <menubar::Item class={ITEM_STYLE} disabled=true>
                            "New window"
                        </menubar::Item>
                        <hr class="h-px border-border" />
                        <menubar::SubMenu hide_delay={Duration::from_millis(200)}>
                            <menubar::SubMenuTrigger class={cn!(
                                "h-full w-full items-center justify-between", ITEM_STYLE
                            )}>
                                "Share" <icons::ChevronRight class="h-4"></icons::ChevronRight>
                            </menubar::SubMenuTrigger>
                            <Portal>
                                <menubar::SubMenuContent
                                    class="inline-block flex flex-col p-1 w-56 rounded-md border shadow-md focus:outline-none border-border min-w-[8rem] bg-background text-foreground"
                                    show_class="z-10 opacity-100 transition duration-150 ease-in"
                                    hide_class="opacity-0 transition duration-200 ease-out -z-10"
                                >
                                    <menubar::Item class={ITEM_STYLE}>"Email"</menubar::Item>
                                    <menubar::Item class={ITEM_STYLE} disabled=true>
                                        "Message"
                                    </menubar::Item>
                                    <menubar::Item class={ITEM_STYLE}>"Copy link"</menubar::Item>
                                    <hr class="h-px border-border" />
                                    <menubar::SubMenu hide_delay={Duration::from_millis(200)}>
                                        <menubar::SubMenuTrigger class={cn!(
                                            "h-full w-full items-center justify-between", ITEM_STYLE
                                        )}>
                                            "More"
                                            <icons::ChevronRight class="h-4"></icons::ChevronRight>
                                        </menubar::SubMenuTrigger>
                                        <Portal>
                                            <menubar::SubMenuContent
                                                class="flex flex-col p-1 w-56 rounded-md border shadow-md focus:outline-none border-border min-w-[8rem] bg-background text-foreground"
                                                show_class="z-10 opacity-100 transition duration-150 ease-in"
                                                hide_class="opacity-0 transition duration-200 ease-out -z-10"
                                            >
                                                <menubar::Item class={ITEM_STYLE}>"Facebook"</menubar::Item>
                                                <menubar::Item class={ITEM_STYLE}>"LinkedIn"</menubar::Item>
                                                <menubar::Item class={ITEM_STYLE}>
                                                    "Instagram"
                                                </menubar::Item>
                                            </menubar::SubMenuContent>
                                        </Portal>
                                    </menubar::SubMenu>
                                </menubar::SubMenuContent>
                            </Portal>
                        </menubar::SubMenu>
                    </menubar::Content>
                </Portal>
            </menubar::Menu>
            <menubar::Menu
                class="relative"
                positioning={menubar::Positioning::BottomStart}
                hide_delay={Duration::from_millis(200)}
            >
                <menubar::Trigger class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted">
                    "Edit"
                </menubar::Trigger>
                <Portal>
                    <menubar::Content
                        class="flex flex-col p-1 w-56 rounded-md border shadow-md focus:outline-none border-border min-w-[8rem] bg-background text-foreground"
                        show_class="z-10 opacity-100 transition duration-150 ease-in"
                        hide_class="opacity-0 transition duration-200 ease-out -z-10"
                    >
                        <menubar::Item class={ITEM_STYLE}>"Undo"</menubar::Item>
                        <menubar::Item class={ITEM_STYLE}>"Redo"</menubar::Item>
                    </menubar::Content>
                </Portal>
            </menubar::Menu>
            <menubar::Menu
                class="relative"
                positioning={menubar::Positioning::BottomStart}
                hide_delay={Duration::from_millis(200)}
            >
                <menubar::Trigger class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted">
                    "Components"
                </menubar::Trigger>
                <Portal>
                    <menubar::Content
                        class="flex flex-col p-1 w-56 rounded-md border shadow-md focus:outline-none border-border min-w-[8rem] bg-background text-foreground"
                        show_class="z-10 opacity-100 transition duration-150 ease-in"
                        hide_class="opacity-0 transition duration-200 ease-out -z-10"
                    >
                        <menubar::Item class={ITEM_STYLE}>
                            <a href="/docs/accordion">"Accordion"</a>
                        </menubar::Item>
                        <menubar::Item class={ITEM_STYLE}>
                            <a href="/docs/dropdown-menu">"Dropdown Menu"</a>
                        </menubar::Item>
                    </menubar::Content>
                </Portal>
            </menubar::Menu>
        </menubar::Root>
    }
}
