use leptos::{logging::log, prelude::*};

use crate::{
    components::{
        api_table::{DataAttrRow, DataAttrsTable, KeyboardRow, KeyboardTable, PropRow, PropsTable, SectionHeading},
        code::Code,
    },
    icons,
};

const USAGE_CODE: &str = r#"use std::time::Duration;
use leptos::{portal::Portal, prelude::*};
use biji_ui::components::menu;

#[component]
pub fn MyDropdownMenu() -> impl IntoView {
    view! {
        <menu::Menu
            positioning={menu::Positioning::Bottom}
            hide_delay={Duration::from_millis(200)}
        >
            <menu::Trigger class="rounded border px-3 py-1.5 text-sm">
                "Open menu"
            </menu::Trigger>
            <Portal>
                <menu::Content
                    class="flex flex-col p-1 w-48 rounded-md border shadow-md bg-background"
                    show_class="opacity-100 transition duration-150 ease-in"
                    hide_class="opacity-0 transition duration-200 ease-out"
                >
                    <menu::Item class="px-2 py-1.5 text-sm rounded-sm hover:bg-accent data-[highlighted]:bg-muted">
                        "Profile"
                    </menu::Item>
                    <menu::Item class="px-2 py-1.5 text-sm rounded-sm hover:bg-accent data-[highlighted]:bg-muted">
                        "Settings"
                    </menu::Item>
                    <menu::Item
                        disabled=true
                        class="px-2 py-1.5 text-sm rounded-sm data-[disabled]:opacity-50 data-[disabled]:pointer-events-none"
                    >
                        "Billing"
                    </menu::Item>
                </menu::Content>
            </Portal>
        </menu::Menu>
    }
}"#;

const MENU_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the menu root element.",
    },
    PropRow {
        name: "disabled",
        prop_type: "bool",
        default: "false",
        description: "Disables the entire menu and its trigger when true.",
    },
    PropRow {
        name: "allow_loop",
        prop_type: "bool",
        default: "false",
        description: "When true, keyboard navigation wraps from the last item back to the first and vice versa.",
    },
    PropRow {
        name: "positioning",
        prop_type: "Positioning",
        default: "BottomStart",
        description: "Where to position the content relative to the trigger.",
    },
    PropRow {
        name: "hide_delay",
        prop_type: "Duration",
        default: "200ms",
        description: "How long to wait before unmounting the content after closing begins. Should match your CSS transition duration.",
    },
    PropRow {
        name: "prevent_scroll",
        prop_type: "bool",
        default: "false",
        description: "When true, prevents the page from scrolling while the menu is open.",
    },
];

const TRIGGER_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the trigger element.",
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
        name: "positioning",
        prop_type: "Positioning",
        default: "BottomStart",
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
        description: "Present on Item and SubMenuTrigger when they have keyboard focus or are hovered.",
    },
    DataAttrRow {
        name: "data-disabled",
        description: "Present on Item and SubMenuTrigger when disabled is true.",
    },
];

const KEYBOARD: &[KeyboardRow] = &[
    KeyboardRow {
        key: "ArrowDown",
        description: "Moves focus to the next item in the menu.",
    },
    KeyboardRow {
        key: "ArrowUp",
        description: "Moves focus to the previous item in the menu.",
    },
    KeyboardRow {
        key: "ArrowRight",
        description: "Opens the focused submenu and moves focus to its first item.",
    },
    KeyboardRow {
        key: "ArrowLeft",
        description: "Closes the current submenu and returns focus to its trigger.",
    },
    KeyboardRow {
        key: "Enter",
        description: "Activates the focused item (clicks the inner button or link).",
    },
    KeyboardRow {
        key: "Escape",
        description: "Closes the menu.",
    },
];

#[component]
pub fn DropdownMenuDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    view! {
        <DocPage title="Dropdown Menu">
            <p class="mt-3 mb-11 text-base text-balance">
                "Displays a menu of items that users can select from when triggered."
            </p>
            <DocPreview>
                <DropdownMenuExample />
            </DocPreview>
            <SectionHeading title="Usage" />
            <Code
                class="[&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={USAGE_CODE}
                language="rust"
            />
            <SectionHeading title="API Reference" />
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
pub fn DropdownMenuExample() -> impl IntoView {
    use std::time::Duration;

    use biji_ui::components::menu;

    view! {
        <menu::Menu
            class="w-fit"
            allow_loop=true
            positioning={menu::Positioning::Bottom}
            hide_delay={Duration::from_millis(200)}
        >
            <menu::Trigger class="inline-flex justify-center items-center w-10 h-10 text-sm font-medium rounded-full border focus-visible:ring-2 focus-visible:ring-offset-2 border-border text-foreground border-border-input bg-background shadow-btn hover:bg-muted focus-visible:ring-offset-background focus-visible focus-visible:ring-foreground active:scale-98">
                <icons::Ellipsis class="w-6 h-6 text-foreground"></icons::Ellipsis>
            </menu::Trigger>
            <menu::Content
                class="flex-col p-1 w-56 rounded-md border shadow-md focus:outline-none border-border min-w-[8rem] bg-background text-foreground"
                show_class="z-10 opacity-100 transition duration-150 ease-in"
                hide_class="-z-10 opacity-0 transition duration-200 ease-out"
            >
                <menu::Item class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none  hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[disabled]:pointer-events-none data-[disabled]:opacity-50 data-[highlighted]:bg-muted">
                    <button class="flex w-full h-full" on:click={|_| log!("Profile clicked")}>
                        "Profile"
                    </button>
                </menu::Item>
                <menu::Item
                    disabled=true
                    class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted"
                >
                    "Billing"
                </menu::Item>
                <menu::Item class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted">
                    "Settings"
                </menu::Item>
                <menu::Item class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted">
                    <a href="/docs/accordion" class="flex w-full h-full">
                        "Accordion"
                    </a>
                </menu::Item>
                <menu::SubMenu class="relative" hide_delay={Duration::from_millis(200)}>
                    <menu::SubMenuTrigger class="flex justify-between items-center w-full cursor-pointer select-none rounded-sm px-2 py-1.5 text-sm outline-none hover:bg-accent hover:text-accent-foreground focus:outline-none  !ring-0 !ring-transparent data-[disabled]:pointer-events-none data-[disabled]:opacity-50 data-[highlighted]:bg-muted">
                        "Sub menu" <icons::ChevronRight class="w-4"></icons::ChevronRight>
                    </menu::SubMenuTrigger>
                    <menu::Content
                        class="flex absolute flex-col p-1 w-56 rounded-md border shadow-md focus:outline-none border-border min-w-[8rem] bg-background text-foreground"
                        show_class="z-10 top-0 left-[105%] opacity-100 transition duration-150 ease-in"
                        hide_class="-z-10 top-0 left-[105%] opacity-0 transition duration-200 ease-out"
                    >
                        <menu::Item class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted">
                            <button
                                class="flex w-full h-full"
                                on:click={|_| log!("Profile clicked")}
                            >
                                "Profile"
                            </button>
                        </menu::Item>
                        <menu::Item
                            disabled=true
                            class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted"
                        >
                            "Billing"
                        </menu::Item>
                        <menu::Item class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted">
                            "Settings"
                        </menu::Item>
                    </menu::Content>
                </menu::SubMenu>
            </menu::Content>
        </menu::Menu>
    }
}
