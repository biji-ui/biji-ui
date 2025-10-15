use std::time::Duration;

use biji_ui::cn;
use leptos::{logging::log, portal::Portal, prelude::*};

use crate::icons;

#[component]
pub fn MenubarDocPage() -> impl IntoView {
    use crate::pages::docs::DocPage;

    view! {
        <DocPage
            title="Menubar"
            description="Displays a menu to the user, which can consist of links or functions, triggered by a button."
            example={MenubarExample}
        />
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
                        <hr class="h-px border-border" />
                        <menubar::SubMenu
                            class="relative"
                            positioning={menubar::Positioning::RightStart}
                            hide_delay={Duration::from_millis(200)}
                        >
                            <menubar::SubMenuTrigger class={cn!(
                                "h-full w-full items-center justify-between", ITEM_STYLE
                            )}>
                                "Share" <icons::ChevronRight class="h-4"></icons::ChevronRight>
                            </menubar::SubMenuTrigger>
                            <Portal>
                                <menubar::SubMenuContent
                                    class="flex flex-col p-1 w-56 rounded-md border shadow-md focus:outline-none border-border min-w-[8rem] bg-background text-foreground"
                                    show_class="z-10 opacity-100 transition duration-150 ease-in"
                                    hide_class="opacity-0 transition duration-200 ease-out -z-10"
                                >
                                    <menubar::Item class={ITEM_STYLE}>"Email"</menubar::Item>
                                    <menubar::Item class={ITEM_STYLE} disabled=true>
                                        "Message"
                                    </menubar::Item>
                                    <menubar::Item class={ITEM_STYLE}>"Copy link"</menubar::Item>
                                    <hr class="h-px border-border" />
                                    <menubar::SubMenu
                                        class="relative"
                                        positioning={menubar::Positioning::RightStart}
                                        hide_delay={Duration::from_millis(200)}
                                    >
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
