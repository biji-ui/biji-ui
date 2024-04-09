use std::time::Duration;

use leptos::{logging::log, *};

#[component]
pub fn MenubarDocPage() -> impl IntoView {
    use crate::pages::docs::DocPage;

    view! { <DocPage title="Menubar" description="" example={MenubarExample}/> }
}

#[component]
pub fn MenubarExample() -> impl IntoView {
    use biji_ui::components::menubar;

    view! {
        <menubar::Root class="flex">
            <menubar::Menu index=0 class="relative">
                <menubar::Trigger class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted">
                    "File"
                </menubar::Trigger>
                <menubar::Content
                    class="absolute flex w-56 min-w-[8rem] flex-col rounded-md border bg-popover p-1 text-popover-foreground shadow-md focus:outline-none"
                    show_class="z-10 opacity-100 transition duration-150 ease-in"
                    hide_class="-z-10 opacity-0 transition duration-200 ease-out"
                    hide_delay={Duration::from_millis(200)}
                >
                    <menubar::Item
                        index=0
                        class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted"
                    >
                        <button
                            class="flex h-full w-full"
                            on:click={|_| log!("Content of File 1 clicked")}
                        >
                            "Content of File 1"
                        </button>
                    </menubar::Item>
                    <menubar::Item
                        index=1
                        class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted"
                    >
                        "Content of File 2"
                    </menubar::Item>
                </menubar::Content>
            </menubar::Menu>
            <menubar::Menu index=1 class="relative">
                <menubar::Trigger class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted">
                    "Edit"
                </menubar::Trigger>
                <menubar::Content
                    class="absolute flex w-56 min-w-[8rem] flex-col rounded-md border bg-popover p-1 text-popover-foreground shadow-md focus:outline-none"
                    show_class="z-10 opacity-100 transition duration-150 ease-in"
                    hide_class="-z-10 opacity-0 transition duration-200 ease-out"
                    hide_delay={Duration::from_millis(200)}
                >
                    <menubar::Item
                        index=0
                        class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted"
                    >
                        "Content of Edit 1"
                    </menubar::Item>
                    <menubar::Item
                        index=1
                        class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted"
                    >
                        "Content of Edit 2"
                    </menubar::Item>
                </menubar::Content>
            </menubar::Menu>
            <menubar::Menu index=2 class="relative">
                <menubar::Trigger class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted">
                    "Selection"
                </menubar::Trigger>
                <menubar::Content
                    class="absolute flex w-56 min-w-[8rem] flex-col rounded-md border bg-popover p-1 text-popover-foreground shadow-md focus:outline-none"
                    show_class="z-10 opacity-100 transition duration-150 ease-in"
                    hide_class="-z-10 opacity-0 transition duration-200 ease-out"
                    hide_delay={Duration::from_millis(200)}
                >
                    <menubar::Item
                        index=0
                        class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted"
                    >
                        "Content of Selection 1"
                    </menubar::Item>
                    <menubar::Item
                        index=1
                        class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted"
                    >
                        "Content of Selection 2"
                    </menubar::Item>
                </menubar::Content>
            </menubar::Menu>
            <menubar::Menu index=3 class="relative">
                <menubar::Trigger class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted">
                    "Components"
                </menubar::Trigger>
                <menubar::Content
                    class="absolute flex w-56 min-w-[8rem] flex-col rounded-md border bg-popover p-1 text-popover-foreground shadow-md focus:outline-none"
                    show_class="z-10 opacity-100 transition duration-150 ease-in"
                    hide_class="-z-10 opacity-0 transition duration-200 ease-out"
                    hide_delay={Duration::from_millis(200)}
                >
                    <menubar::Item
                        index=0
                        class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted"
                    >
                        <a class="flex h-full w-full" href="/docs/accordion">
                            "Accordion"
                        </a>
                    </menubar::Item>
                    <menubar::Item
                        index=1
                        class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted"
                    >
                        <a class="flex h-full w-full" href="/docs/dropdown-menu">
                            "Dropdown Menu"
                        </a>
                    </menubar::Item>
                </menubar::Content>
            </menubar::Menu>
        </menubar::Root>
    }
}
