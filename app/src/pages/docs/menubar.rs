use std::time::Duration;

use biji_ui::cn;
use leptos::{logging::log, *};

use crate::icons;

#[component]
pub fn MenubarDocPage() -> impl IntoView {
    use crate::pages::docs::DocPage;

    view! { <DocPage title="Menubar" description="" example={MenubarExample}/> }
}

#[component]
pub fn MenubarExample() -> impl IntoView {
    use biji_ui::components::menubar;

    const ITEM_STYLE: &str = "flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted";

    view! {
        <menubar::Root class="flex">
            <menubar::Menu index=0 class="relative">
                <menubar::MenuTrigger class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted">
                    "File"
                </menubar::MenuTrigger>
                <menubar::MenuContent
                    class="absolute flex w-56 min-w-[8rem] flex-col rounded-md border bg-popover p-1 text-popover-foreground shadow-md focus:outline-none"
                    show_class="z-10 opacity-100 transition duration-150 ease-in"
                    hide_class="-z-10 opacity-0 transition duration-200 ease-out"
                    hide_delay={Duration::from_millis(200)}
                >
                    <menubar::Item index=0 class={ITEM_STYLE}>
                        <button class="flex h-full w-full" on:click={|_| log!("New file clicked")}>
                            "New file..."
                        </button>
                    </menubar::Item>
                    <menubar::Item index=1 class={ITEM_STYLE} disabled=true>
                        "New window"
                    </menubar::Item>
                    <menubar::SubMenuItem index=2 class="relative">
                        <menubar::SubMenuItemTrigger class={cn!(
                            "h-full w-full items-center justify-between", ITEM_STYLE
                        )}>
                            "Share" <icons::ChevronRight class="h-4"></icons::ChevronRight>
                        </menubar::SubMenuItemTrigger>
                        <menubar::SubMenuItemContent
                            class="absolute flex w-56 min-w-[8rem] flex-col rounded-md border bg-popover p-1 text-popover-foreground shadow-md left-full top-0 focus:outline-none"
                            show_class="z-10 opacity-100 transition duration-150 ease-in"
                            hide_class="-z-10 opacity-0 transition duration-200 ease-out"
                            hide_delay={Duration::from_millis(200)}
                        >
                            <menubar::Item index=0 class={ITEM_STYLE}>
                                "Email"
                            </menubar::Item>
                            <menubar::Item index=1 class={ITEM_STYLE} disabled=true>
                                "Message"
                            </menubar::Item>
                            <menubar::Item index=2 class={ITEM_STYLE}>
                                "Copy link"
                            </menubar::Item>
                            <menubar::SubMenuItem index=3 class="relative">
                                <menubar::SubMenuItemTrigger class={cn!(
                                    "h-full w-full items-center justify-between", ITEM_STYLE
                                )}>
                                    "Share" <icons::ChevronRight class="h-4"></icons::ChevronRight>
                                </menubar::SubMenuItemTrigger>
                                <menubar::SubMenuItemContent
                                    class="absolute flex w-56 min-w-[8rem] flex-col rounded-md border bg-popover p-1 text-popover-foreground shadow-md left-full top-0 focus:outline-none"
                                    show_class="z-10 opacity-100 transition duration-150 ease-in"
                                    hide_class="-z-10 opacity-0 transition duration-200 ease-out"
                                    hide_delay={Duration::from_millis(200)}
                                >
                                    <menubar::Item index=0 class={ITEM_STYLE}>
                                        "Email"
                                    </menubar::Item>
                                    <menubar::Item index=1 class={ITEM_STYLE}>
                                        "Message"
                                    </menubar::Item>
                                    <menubar::Item index=2 class={ITEM_STYLE}>
                                        "Copy link"
                                    </menubar::Item>
                                </menubar::SubMenuItemContent>
                            </menubar::SubMenuItem>
                        </menubar::SubMenuItemContent>
                    </menubar::SubMenuItem>
                </menubar::MenuContent>
            </menubar::Menu>
            <menubar::Menu index=1 class="relative">
                <menubar::MenuTrigger class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted">
                    "Edit"
                </menubar::MenuTrigger>
                <menubar::MenuContent
                    class="absolute flex w-56 min-w-[8rem] flex-col rounded-md border bg-popover p-1 text-popover-foreground shadow-md focus:outline-none"
                    show_class="z-10 opacity-100 transition duration-150 ease-in"
                    hide_class="-z-10 opacity-0 transition duration-200 ease-out"
                    hide_delay={Duration::from_millis(200)}
                >
                    <menubar::Item index=0 class={ITEM_STYLE}>
                        "New file..."
                    </menubar::Item>
                    <menubar::Item index=1 class={ITEM_STYLE}>
                        "New window"
                    </menubar::Item>
                </menubar::MenuContent>
            </menubar::Menu>
            <menubar::Menu index=2 class="relative">
                <menubar::MenuTrigger class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted">
                    "View"
                </menubar::MenuTrigger>
                <menubar::MenuContent
                    class="absolute flex w-56 min-w-[8rem] flex-col rounded-md border bg-popover p-1 text-popover-foreground shadow-md focus:outline-none"
                    show_class="z-10 opacity-100 transition duration-150 ease-in"
                    hide_class="-z-10 opacity-0 transition duration-200 ease-out"
                    hide_delay={Duration::from_millis(200)}
                >
                    <menubar::Item index=0 class={ITEM_STYLE}>
                        "New file..."
                    </menubar::Item>
                    <menubar::Item index=1 class={ITEM_STYLE}>
                        "New window"
                    </menubar::Item>
                </menubar::MenuContent>
            </menubar::Menu>
        </menubar::Root>
    }
}
