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
    "\", features = [\"drawer\"] }",
);

const USAGE_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::drawer;

#[component]
pub fn MyDrawer() -> impl IntoView {
    view! {
        <drawer::Root>
            <drawer::Trigger class="px-4 py-2 rounded-md bg-primary text-primary-foreground">
                "Open Drawer"
            </drawer::Trigger>
            // Full-screen overlay
            <drawer::Overlay
                class="fixed inset-0 z-[80] bg-black/50 transition"
                show_class="opacity-100 duration-300 ease-out"
                hide_class="opacity-0 duration-200 ease-in"
            />
            // Sliding panel
            <drawer::Content
                class="fixed inset-y-0 right-0 z-[90] w-80 bg-background border-l \
                       border-border shadow-xl transition-transform"
                show_class="translate-x-0 duration-300 ease-out"
                hide_class="translate-x-full duration-200 ease-in"
            >
                <div class="flex flex-col h-full p-6">
                    <drawer::Title class="text-lg font-semibold">
                        "Panel Title"
                    </drawer::Title>
                    <drawer::Description class="mt-1 text-sm text-muted-foreground">
                        "Supplementary description text."
                    </drawer::Description>
                    <div class="mt-auto">
                        <drawer::Close class="w-full px-4 py-2 rounded-md border">
                            "Close"
                        </drawer::Close>
                    </div>
                </div>
            </drawer::Content>
        </drawer::Root>
    }
}"#;

const ROOT_WITH_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::drawer;

#[component]
pub fn MyDrawer() -> impl IntoView {
    view! {
        <drawer::RootWith let:d>
            <p class="mb-2 text-sm text-muted-foreground">
                {move || if d.open.get() { "Drawer is open" } else { "Drawer is closed" }}
            </p>
            <drawer::Trigger class="px-4 py-2 rounded-md bg-primary text-primary-foreground">
                "Open Drawer"
            </drawer::Trigger>
            <drawer::Overlay
                class="fixed inset-0 z-[80] bg-black/50 transition"
                show_class="opacity-100 duration-300 ease-out"
                hide_class="opacity-0 duration-200 ease-in"
            />
            <drawer::Content
                class="fixed inset-y-0 right-0 z-[90] w-80 bg-background border-l border-border shadow-xl transition-transform"
                show_class="translate-x-0 duration-300 ease-out"
                hide_class="translate-x-full duration-200 ease-in"
            >
                <div class="flex flex-col h-full p-6">
                    <drawer::Title class="text-lg font-semibold">"Settings"</drawer::Title>
                    <drawer::Close class="mt-auto w-full px-4 py-2 rounded-md border">
                        "Close"
                    </drawer::Close>
                </div>
            </drawer::Content>
        </drawer::RootWith>
    }
}"#;

const ROOT_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the wrapper `<div>`.",
    },
    PropRow {
        name: "side",
        prop_type: "DrawerSide",
        default: "Right",
        description: "Which edge the panel slides in from. Sets `data-side` on Content.",
    },
    PropRow {
        name: "prevent_scroll",
        prop_type: "bool",
        default: "true",
        description: "When true, prevents the page from scrolling while the drawer is open.",
    },
    PropRow {
        name: "hide_delay",
        prop_type: "Duration",
        default: "300ms",
        description: "How long to wait before unmounting Content after closing (match your CSS transition).",
    },
    PropRow {
        name: "open",
        prop_type: "bool",
        default: "false",
        description: "Initial open state.",
    },
    PropRow {
        name: "on_open_change",
        prop_type: "Option<Callback<bool>>",
        default: "None",
        description: "Called with `true` when opening and `false` when closing.",
    },
];

const TRIGGER_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the `<button>` element.",
}];

const OVERLAY_PROPS: &[PropRow] = &[
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
        description: "CSS class applied when the overlay is visible.",
    },
    PropRow {
        name: "hide_class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied while the overlay is hiding.",
    },
];

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
        description: "CSS class applied when the panel is open (e.g. `translate-x-0`).",
    },
    PropRow {
        name: "hide_class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied while the panel is closing (e.g. `translate-x-full`).",
    },
];

const CLOSE_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the `<button>` element.",
}];

const DATA_ATTRS: &[DataAttrRow] = &[
    DataAttrRow {
        name: "data-state",
        description: "\"open\" when the drawer is visible; \"closed\" otherwise. Present on Trigger and Content.",
    },
    DataAttrRow {
        name: "data-side",
        description: "\"top\" | \"right\" | \"bottom\" | \"left\". Present on Content.",
    },
];

const KEYBOARD: &[KeyboardRow] = &[
    KeyboardRow {
        key: "Tab",
        description: "Moves focus to the next focusable element inside the drawer panel.",
    },
    KeyboardRow {
        key: "Shift + Tab",
        description: "Moves focus to the previous focusable element inside the drawer panel.",
    },
    KeyboardRow {
        key: "Escape",
        description: "Closes the drawer and returns focus to the Trigger.",
    },
];

#[component]
pub fn DrawerDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    view! {
        <DocPage title="Drawer">
            <p class="mt-3 mb-11 text-base text-balance">
                "A panel that slides in from the edge of the screen, with focus trapping and scroll lock."
            </p>
            <DocPreview>
                <DrawerExample />
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
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"DrawerState"</code>
                " inline via the "
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"let:"</code>
                " binding. The state is "
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"Copy"</code>
                " and safe to pass as a prop."
            </p>
            <DocPreview>
                <DrawerRootWithExample />
            </DocPreview>
            <Code
                class="[&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={ROOT_WITH_CODE}
                language="rust"
            />
            <SectionHeading title="Examples" />
            <h3 class="mt-8 mb-2 text-base font-semibold">"Side"</h3>
            <p class="mb-5 text-sm text-muted-foreground">
                "Use the "
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"side"</code>
                " prop on "
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"Root"</code>
                " to control which edge the panel slides in from."
            </p>
            <DocPreview>
                <DrawerSidesExample />
            </DocPreview>
            <SectionHeading title="API Reference" />
            <PropsTable title="Root / RootWith" rows={ROOT_PROPS} />
            <PropsTable title="Trigger" rows={TRIGGER_PROPS} />
            <PropsTable title="Overlay" rows={OVERLAY_PROPS} />
            <PropsTable title="Content" rows={CONTENT_PROPS} />
            <PropsTable title="Close" rows={CLOSE_PROPS} />
            <PropsTable
                title="Title"
                rows={&[PropRow {
                    name: "class",
                    prop_type: "String",
                    default: "\"\"",
                    description: "CSS class applied to the `<h2>` element. Its `id` is auto-wired to `aria-labelledby` on Content.",
                }]}
            />
            <PropsTable
                title="Description"
                rows={&[PropRow {
                    name: "class",
                    prop_type: "String",
                    default: "\"\"",
                    description: "CSS class applied to the `<p>` element. Its `id` is auto-wired to `aria-describedby` on Content.",
                }]}
            />
            <DataAttrsTable rows={DATA_ATTRS} />
            <KeyboardTable rows={KEYBOARD} />
        </DocPage>
    }
}

#[component]
pub fn DrawerRootWithExample() -> impl IntoView {
    use biji_ui::components::drawer;

    view! {
        <div class="flex flex-col items-center gap-3 p-8">
            <drawer::RootWith let:d>
                <p class="text-sm text-muted-foreground">
                    {move || if d.open.get() { "Drawer is open" } else { "Drawer is closed" }}
                </p>
                <drawer::Trigger class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium \
                                        rounded-md border border-border bg-background \
                                        hover:bg-accent transition-colors">
                    "Open Drawer"
                </drawer::Trigger>
                <drawer::Overlay
                    class="fixed inset-0 z-[80] bg-black/50 transition"
                    show_class="opacity-100 duration-300 ease-out"
                    hide_class="opacity-0 duration-200 ease-in"
                />
                <drawer::Content
                    class="fixed inset-y-0 right-0 z-[90] w-80 bg-background border-l \
                           border-border shadow-xl transition-transform"
                    show_class="translate-x-0 duration-300 ease-out"
                    hide_class="translate-x-full duration-200 ease-in"
                >
                    <div class="flex flex-col h-full p-6">
                        <drawer::Title class="text-lg font-semibold">"Settings"</drawer::Title>
                        <drawer::Description class="mt-1 text-sm text-muted-foreground">
                            "Manage your preferences."
                        </drawer::Description>
                        <div class="mt-auto">
                            <drawer::Close class="w-full px-4 py-2 text-sm font-medium rounded-md \
                                                  bg-primary text-primary-foreground \
                                                  hover:bg-primary/90 transition-colors">
                                "Save Changes"
                            </drawer::Close>
                        </div>
                    </div>
                </drawer::Content>
            </drawer::RootWith>
        </div>
    }
}

#[component]
pub fn DrawerExample() -> impl IntoView {
    use biji_ui::components::drawer;

    view! {
        <div class="flex justify-center p-8">
            <drawer::Root>
                <drawer::Trigger class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium \
                                        rounded-md border border-border bg-background \
                                        hover:bg-accent transition-colors">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
                        stroke="currentColor" stroke-width="2" stroke-linecap="round"
                        stroke-linejoin="round" class="w-4 h-4">
                        <path d="M9 18 15 12 9 6"/>
                    </svg>
                    "Open Drawer"
                </drawer::Trigger>
                <drawer::Overlay
                    class="fixed inset-0 z-[80] bg-black/50 transition"
                    show_class="opacity-100 duration-300 ease-out"
                    hide_class="opacity-0 duration-200 ease-in"
                />
                <drawer::Content
                    class="fixed inset-y-0 right-0 z-[90] w-80 bg-background border-l \
                           border-border shadow-xl transition-transform"
                    show_class="translate-x-0 duration-300 ease-out"
                    hide_class="translate-x-full duration-200 ease-in"
                >
                    <div class="flex flex-col h-full p-6">
                        <div class="flex items-center justify-between mb-4">
                            <drawer::Title class="text-lg font-semibold">
                                "Settings"
                            </drawer::Title>
                            <drawer::Close class="flex items-center justify-center w-8 h-8 \
                                                  rounded-md hover:bg-accent transition-colors">
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
                                    stroke="currentColor" stroke-width="2" stroke-linecap="round"
                                    stroke-linejoin="round" class="w-4 h-4">
                                    <path d="M18 6 6 18"/><path d="m6 6 12 12"/>
                                </svg>
                            </drawer::Close>
                        </div>
                        <drawer::Description class="text-sm text-muted-foreground mb-6">
                            "Manage your account settings and preferences."
                        </drawer::Description>
                        <div class="space-y-4">
                            <div class="flex items-center justify-between py-3 border-b border-border">
                                <span class="text-sm font-medium">"Notifications"</span>
                                <span class="text-xs text-muted-foreground">"Enabled"</span>
                            </div>
                            <div class="flex items-center justify-between py-3 border-b border-border">
                                <span class="text-sm font-medium">"Dark Mode"</span>
                                <span class="text-xs text-muted-foreground">"System"</span>
                            </div>
                            <div class="flex items-center justify-between py-3 border-b border-border">
                                <span class="text-sm font-medium">"Language"</span>
                                <span class="text-xs text-muted-foreground">"English"</span>
                            </div>
                        </div>
                        <div class="mt-auto pt-6">
                            <drawer::Close class="w-full px-4 py-2 text-sm font-medium rounded-md \
                                                  bg-primary text-primary-foreground \
                                                  hover:bg-primary/90 transition-colors">
                                "Save Changes"
                            </drawer::Close>
                        </div>
                    </div>
                </drawer::Content>
            </drawer::Root>
        </div>
    }
}

#[component]
pub fn DrawerSidesExample() -> impl IntoView {
    use biji_ui::components::drawer::{self, DrawerSide};

    view! {
        <div class="flex flex-wrap items-center justify-center gap-3 p-8">
            // Top
            <drawer::Root side={DrawerSide::Top}>
                <drawer::Trigger class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium \
                                        rounded-md border border-border bg-background \
                                        hover:bg-accent transition-colors">
                    "Top"
                </drawer::Trigger>
                <drawer::Overlay
                    class="fixed inset-0 z-[80] bg-black/50 transition"
                    show_class="opacity-100 duration-300 ease-out"
                    hide_class="opacity-0 duration-200 ease-in"
                />
                <drawer::Content
                    class="fixed inset-x-0 top-0 z-[90] h-48 bg-background border-b \
                           border-border shadow-xl transition-transform"
                    show_class="translate-y-0 duration-300 ease-out"
                    hide_class="-translate-y-full duration-200 ease-in"
                >
                    <div class="flex flex-col h-full p-6">
                        <div class="flex items-center justify-between">
                            <drawer::Title class="text-base font-semibold">"Top Drawer"</drawer::Title>
                            <drawer::Close class="flex items-center justify-center w-8 h-8 \
                                                  rounded-md hover:bg-accent transition-colors">
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
                                    stroke="currentColor" stroke-width="2" stroke-linecap="round"
                                    stroke-linejoin="round" class="w-4 h-4">
                                    <path d="M18 6 6 18"/><path d="m6 6 12 12"/>
                                </svg>
                            </drawer::Close>
                        </div>
                        <drawer::Description class="mt-2 text-sm text-muted-foreground">
                            "Slides in from the top edge of the screen."
                        </drawer::Description>
                    </div>
                </drawer::Content>
            </drawer::Root>

            // Right
            <drawer::Root side={DrawerSide::Right}>
                <drawer::Trigger class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium \
                                        rounded-md border border-border bg-background \
                                        hover:bg-accent transition-colors">
                    "Right"
                </drawer::Trigger>
                <drawer::Overlay
                    class="fixed inset-0 z-[80] bg-black/50 transition"
                    show_class="opacity-100 duration-300 ease-out"
                    hide_class="opacity-0 duration-200 ease-in"
                />
                <drawer::Content
                    class="fixed inset-y-0 right-0 z-[90] w-72 bg-background border-l \
                           border-border shadow-xl transition-transform"
                    show_class="translate-x-0 duration-300 ease-out"
                    hide_class="translate-x-full duration-200 ease-in"
                >
                    <div class="flex flex-col h-full p-6">
                        <div class="flex items-center justify-between">
                            <drawer::Title class="text-base font-semibold">"Right Drawer"</drawer::Title>
                            <drawer::Close class="flex items-center justify-center w-8 h-8 \
                                                  rounded-md hover:bg-accent transition-colors">
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
                                    stroke="currentColor" stroke-width="2" stroke-linecap="round"
                                    stroke-linejoin="round" class="w-4 h-4">
                                    <path d="M18 6 6 18"/><path d="m6 6 12 12"/>
                                </svg>
                            </drawer::Close>
                        </div>
                        <drawer::Description class="mt-2 text-sm text-muted-foreground">
                            "Slides in from the right edge of the screen."
                        </drawer::Description>
                    </div>
                </drawer::Content>
            </drawer::Root>

            // Bottom
            <drawer::Root side={DrawerSide::Bottom}>
                <drawer::Trigger class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium \
                                        rounded-md border border-border bg-background \
                                        hover:bg-accent transition-colors">
                    "Bottom"
                </drawer::Trigger>
                <drawer::Overlay
                    class="fixed inset-0 z-[80] bg-black/50 transition"
                    show_class="opacity-100 duration-300 ease-out"
                    hide_class="opacity-0 duration-200 ease-in"
                />
                <drawer::Content
                    class="fixed inset-x-0 bottom-0 z-[90] h-48 bg-background border-t \
                           border-border shadow-xl transition-transform"
                    show_class="translate-y-0 duration-300 ease-out"
                    hide_class="translate-y-full duration-200 ease-in"
                >
                    <div class="flex flex-col h-full p-6">
                        <div class="flex items-center justify-between">
                            <drawer::Title class="text-base font-semibold">"Bottom Drawer"</drawer::Title>
                            <drawer::Close class="flex items-center justify-center w-8 h-8 \
                                                  rounded-md hover:bg-accent transition-colors">
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
                                    stroke="currentColor" stroke-width="2" stroke-linecap="round"
                                    stroke-linejoin="round" class="w-4 h-4">
                                    <path d="M18 6 6 18"/><path d="m6 6 12 12"/>
                                </svg>
                            </drawer::Close>
                        </div>
                        <drawer::Description class="mt-2 text-sm text-muted-foreground">
                            "Slides in from the bottom edge of the screen."
                        </drawer::Description>
                    </div>
                </drawer::Content>
            </drawer::Root>

            // Left
            <drawer::Root side={DrawerSide::Left}>
                <drawer::Trigger class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium \
                                        rounded-md border border-border bg-background \
                                        hover:bg-accent transition-colors">
                    "Left"
                </drawer::Trigger>
                <drawer::Overlay
                    class="fixed inset-0 z-[80] bg-black/50 transition"
                    show_class="opacity-100 duration-300 ease-out"
                    hide_class="opacity-0 duration-200 ease-in"
                />
                <drawer::Content
                    class="fixed inset-y-0 left-0 z-[90] w-72 bg-background border-r \
                           border-border shadow-xl transition-transform"
                    show_class="translate-x-0 duration-300 ease-out"
                    hide_class="-translate-x-full duration-200 ease-in"
                >
                    <div class="flex flex-col h-full p-6">
                        <div class="flex items-center justify-between">
                            <drawer::Title class="text-base font-semibold">"Left Drawer"</drawer::Title>
                            <drawer::Close class="flex items-center justify-center w-8 h-8 \
                                                  rounded-md hover:bg-accent transition-colors">
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
                                    stroke="currentColor" stroke-width="2" stroke-linecap="round"
                                    stroke-linejoin="round" class="w-4 h-4">
                                    <path d="M18 6 6 18"/><path d="m6 6 12 12"/>
                                </svg>
                            </drawer::Close>
                        </div>
                        <drawer::Description class="mt-2 text-sm text-muted-foreground">
                            "Slides in from the left edge of the screen."
                        </drawer::Description>
                    </div>
                </drawer::Content>
            </drawer::Root>
        </div>
    }
}
