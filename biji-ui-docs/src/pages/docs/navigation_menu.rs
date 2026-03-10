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
    "\", features = [\"navigation_menu\"] }",
);

const USAGE_CODE: &str = r#"use leptos::{portal::Portal, prelude::*};
use biji_ui::components::navigation_menu;

#[component]
pub fn MyNav() -> impl IntoView {
    view! {
        <navigation_menu::Root>
            <navigation_menu::List class="flex items-center gap-1">
                <navigation_menu::Item value="docs">
                    <navigation_menu::Trigger class="px-3 py-1.5 text-sm rounded-md transition hover:bg-accent">
                        "Docs"
                    </navigation_menu::Trigger>
                    <Portal>
                        <navigation_menu::Content
                            class="z-50 min-w-48 rounded-lg border shadow-md transition bg-background border-border origin-[var(--biji-transform-origin)]"
                            show_class="opacity-100 scale-100 duration-150 ease-out"
                            hide_class="opacity-0 scale-95 duration-100 ease-in"
                        >
                            <div class="grid gap-1 p-2">
                                <navigation_menu::Link
                                    href="/docs/getting-started"
                                    class="block px-3 py-2 text-sm rounded-md hover:bg-accent"
                                >
                                    "Getting Started"
                                </navigation_menu::Link>
                                <navigation_menu::Link
                                    href="/docs/accordion"
                                    class="block px-3 py-2 text-sm rounded-md hover:bg-accent"
                                >
                                    "Accordion"
                                </navigation_menu::Link>
                            </div>
                        </navigation_menu::Content>
                    </Portal>
                </navigation_menu::Item>
                <navigation_menu::Item value="github">
                    <navigation_menu::Link
                        href="https://github.com/biji-ui/biji-ui"
                        class="block px-3 py-1.5 text-sm rounded-md transition hover:bg-accent"
                    >
                        "GitHub"
                    </navigation_menu::Link>
                </navigation_menu::Item>
            </navigation_menu::List>
        </navigation_menu::Root>
    }
}"#;

const ROOT_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the `<nav>` element.",
    },
    PropRow {
        name: "aria_label",
        prop_type: "Option<String>",
        default: "None",
        description: "Value of the `aria-label` attribute on the `<nav>` element.",
    },
    PropRow {
        name: "positioning",
        prop_type: "Positioning",
        default: "BottomStart",
        description: "Where to render each Content panel relative to its Trigger.",
    },
    PropRow {
        name: "avoid_collisions",
        prop_type: "AvoidCollisions",
        default: "Flip",
        description: "How content panels react when they would overflow the viewport.",
    },
    PropRow {
        name: "close_delay",
        prop_type: "Duration",
        default: "200ms",
        description: "How long after the pointer leaves a Trigger or Content before the panel closes.",
    },
    PropRow {
        name: "hide_delay",
        prop_type: "Duration",
        default: "200ms",
        description: "How long to wait before unmounting the content after closing begins (should match your CSS transition duration).",
    },
];

const ITEM_PROPS: &[PropRow] = &[
    PropRow {
        name: "value",
        prop_type: "String",
        default: "—",
        description: "Unique string that identifies this item. Must match the corresponding Content panel.",
    },
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the `<li>` element.",
    },
    PropRow {
        name: "disabled",
        prop_type: "bool",
        default: "false",
        description: "When true, the item's Trigger cannot be interacted with.",
    },
];

const TRIGGER_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the `<button>` element.",
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
        description: "CSS class applied when the panel is open.",
    },
    PropRow {
        name: "hide_class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied while the panel is closing.",
    },
];

const LINK_PROPS: &[PropRow] = &[
    PropRow {
        name: "href",
        prop_type: "String",
        default: "—",
        description: "The URL the link navigates to.",
    },
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the `<a>` element.",
    },
    PropRow {
        name: "disabled",
        prop_type: "bool",
        default: "false",
        description: "When true, renders `aria-disabled` and `data-disabled` on the anchor.",
    },
    PropRow {
        name: "close_on_click",
        prop_type: "bool",
        default: "true",
        description: "When true, clicking the link closes any open content panel.",
    },
];

const DATA_ATTRS: &[DataAttrRow] = &[
    DataAttrRow {
        name: "data-state",
        description: "\"open\" when the item's panel is visible; \"closed\" otherwise. Present on Trigger.",
    },
    DataAttrRow {
        name: "data-disabled",
        description: "Present on Item, Trigger, and Link when the item is disabled.",
    },
    DataAttrRow {
        name: "data-highlighted",
        description: "Present on Trigger when it holds roving-tabindex focus.",
    },
];

const KEYBOARD: &[KeyboardRow] = &[
    KeyboardRow {
        key: "Tab",
        description: "Moves focus into and out of the navigation menu. When a panel is open, Tab moves into the panel.",
    },
    KeyboardRow {
        key: "ArrowRight",
        description: "Moves focus to the next Trigger or Link. If a panel is open, opens the next item's panel.",
    },
    KeyboardRow {
        key: "ArrowLeft / ArrowUp",
        description: "Moves focus to the previous Trigger or Link. If a panel is open, opens the previous item's panel.",
    },
    KeyboardRow {
        key: "ArrowDown",
        description: "Opens the current item's Content panel and focuses the first focusable element inside it.",
    },
    KeyboardRow {
        key: "Home",
        description: "Moves focus to the first Trigger or Link.",
    },
    KeyboardRow {
        key: "End",
        description: "Moves focus to the last Trigger or Link.",
    },
    KeyboardRow {
        key: "Enter / Space",
        description: "Toggles the current item's Content panel.",
    },
    KeyboardRow {
        key: "Escape",
        description: "Closes the open panel and returns focus to its Trigger.",
    },
];

#[component]
pub fn NavigationMenuDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    view! {
        <DocPage title="Navigation Menu">
            <p class="mt-3 mb-11 text-base text-balance">
                "A horizontal navigation bar where hovering or clicking a trigger reveals a positioned content panel."
            </p>
            <DocPreview>
                <NavigationMenuExample />
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
            <SectionHeading title="API Reference" />
            <PropsTable title="Root" rows={ROOT_PROPS} />
            <PropsTable title="List" rows={&[PropRow { name: "class", prop_type: "String", default: "\"\"", description: "CSS class applied to the `<ul>` element." }]} />
            <PropsTable title="Item" rows={ITEM_PROPS} />
            <PropsTable title="Trigger" rows={TRIGGER_PROPS} />
            <PropsTable title="Content" rows={CONTENT_PROPS} />
            <PropsTable title="Link" rows={LINK_PROPS} />
            <DataAttrsTable rows={DATA_ATTRS} />
            <KeyboardTable rows={KEYBOARD} />
        </DocPage>
    }
}

#[component]
pub fn NavigationMenuExample() -> impl IntoView {
    use biji_ui::components::navigation_menu;

    const TRIGGER_CLS: &str =
        "flex items-center gap-1 px-3 py-1.5 text-sm font-medium rounded-md \
         transition-colors cursor-pointer select-none \
         hover:bg-accent data-[state=open]:bg-accent \
         data-[disabled]:opacity-50 data-[disabled]:pointer-events-none";

    const LINK_CLS: &str =
        "block px-3 py-2 text-sm rounded-md transition-colors \
         hover:bg-accent hover:text-accent-foreground";

    view! {
        <div class="flex justify-center p-8">
            <navigation_menu::Root>
                <navigation_menu::List class="flex items-center gap-1 p-1 rounded-lg border bg-background border-border">
                    <navigation_menu::Item value="components">
                        <navigation_menu::Trigger class={TRIGGER_CLS}>
                            "Components"
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
                                stroke="currentColor" stroke-width="2" stroke-linecap="round"
                                stroke-linejoin="round" class="w-3.5 h-3.5 opacity-60">
                                <path d="m6 9 6 6 6-6"/>
                            </svg>
                        </navigation_menu::Trigger>
                        <Portal>
                            <navigation_menu::Content
                                class="z-50 min-w-56 rounded-lg border shadow-lg transition \
                                       bg-background border-border \
                                       origin-[var(--biji-transform-origin)]"
                                show_class="opacity-100 scale-100 duration-150 ease-out"
                                hide_class="opacity-0 scale-95 duration-100 ease-in"
                            >
                                <div class="grid gap-0.5 p-2">
                                    <p class="px-3 py-1.5 text-xs font-semibold text-muted-foreground">
                                        "Form Controls"
                                    </p>
                                    <navigation_menu::Link href="/docs/checkbox" class={LINK_CLS}>
                                        "Checkbox"
                                    </navigation_menu::Link>
                                    <navigation_menu::Link href="/docs/radio-group" class={LINK_CLS}>
                                        "Radio Group"
                                    </navigation_menu::Link>
                                    <navigation_menu::Link href="/docs/select" class={LINK_CLS}>
                                        "Select"
                                    </navigation_menu::Link>
                                    <navigation_menu::Link href="/docs/slider" class={LINK_CLS}>
                                        "Slider"
                                    </navigation_menu::Link>
                                    <div class="my-1 border-t border-border" />
                                    <p class="px-3 py-1.5 text-xs font-semibold text-muted-foreground">
                                        "Overlay"
                                    </p>
                                    <navigation_menu::Link href="/docs/dialog" class={LINK_CLS}>
                                        "Dialog"
                                    </navigation_menu::Link>
                                    <navigation_menu::Link href="/docs/popover" class={LINK_CLS}>
                                        "Popover"
                                    </navigation_menu::Link>
                                    <navigation_menu::Link href="/docs/tooltip" class={LINK_CLS}>
                                        "Tooltip"
                                    </navigation_menu::Link>
                                </div>
                            </navigation_menu::Content>
                        </Portal>
                    </navigation_menu::Item>

                    <navigation_menu::Item value="docs">
                        <navigation_menu::Trigger class={TRIGGER_CLS}>
                            "Docs"
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
                                stroke="currentColor" stroke-width="2" stroke-linecap="round"
                                stroke-linejoin="round" class="w-3.5 h-3.5 opacity-60">
                                <path d="m6 9 6 6 6-6"/>
                            </svg>
                        </navigation_menu::Trigger>
                        <Portal>
                            <navigation_menu::Content
                                class="z-50 min-w-48 rounded-lg border shadow-lg transition \
                                       bg-background border-border \
                                       origin-[var(--biji-transform-origin)]"
                                show_class="opacity-100 scale-100 duration-150 ease-out"
                                hide_class="opacity-0 scale-95 duration-100 ease-in"
                            >
                                <div class="grid gap-0.5 p-2">
                                    <navigation_menu::Link href="/docs/getting-started" class={LINK_CLS}>
                                        "Getting Started"
                                    </navigation_menu::Link>
                                    <navigation_menu::Link href="/docs/accordion" class={LINK_CLS}>
                                        "Accordion"
                                    </navigation_menu::Link>
                                    <navigation_menu::Link href="/docs/tabs" class={LINK_CLS}>
                                        "Tabs"
                                    </navigation_menu::Link>
                                </div>
                            </navigation_menu::Content>
                        </Portal>
                    </navigation_menu::Item>

                    <navigation_menu::Item value="github">
                        <navigation_menu::Link
                            href="https://github.com/biji-ui/biji-ui"
                            class={TRIGGER_CLS}
                        >
                            "GitHub"
                        </navigation_menu::Link>
                    </navigation_menu::Item>
                </navigation_menu::List>
            </navigation_menu::Root>
        </div>
    }
}
