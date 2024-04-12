use leptos::{logging::log, *};

use crate::icons;

#[component]
pub fn DropdownMenuDocPage() -> impl IntoView {
    use crate::pages::docs::DocPage;

    view! {
        <DocPage
            title="Dropdown Menu"
            description="Displays a menu of items that users can select from when triggered."
            example={DropdownMenuExample}
        />
    }
}

#[component]
pub fn DropdownMenuExample() -> impl IntoView {
    use std::time::Duration;

    use biji_ui::components::menu;

    view! {
        <menu::Root class="w-fit">
            <menu::Trigger class="inline-flex h-10 w-10 items-center justify-center rounded-full border text-sm font-medium text-foreground border-border-input bg-background-alt shadow-btn hover:bg-muted focus-visible focus-visible:ring-2 focus-visible:ring-foreground focus-visible:ring-offset-2 focus-visible:ring-offset-background active:scale-98">
                <icons::Ellipsis class="h-6 w-6 text-foreground"></icons::Ellipsis>
            </menu::Trigger>
            <menu::Content
                class="absolute left-1/2 flex w-56 min-w-[8rem] flex-col rounded-md border bg-popover p-1 text-popover-foreground shadow-md focus:outline-none"
                show_class="z-10 -translate-x-1/2 translate-y-0 opacity-100 transition duration-150 ease-in"
                hide_class="-z-10 -translate-x-1/2 translate-y-1 opacity-0 transition duration-200 ease-out"
                hide_delay={Duration::from_millis(200)}
            >
                <menu::Item
                    index=0
                    class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted"
                >
                    <button class="flex h-full w-full" on:click={|_| log!("Profile clicked")}>
                        "Profile"
                    </button>
                </menu::Item>
                <menu::Item
                    index=1
                    disabled=true
                    class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted"
                >
                    "Billing"
                </menu::Item>
                <menu::Item
                    index=2
                    class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted"
                >
                    "Settings"
                </menu::Item>
                <menu::Item
                    index=3
                    class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted"
                >
                    <a href="/docs/accordion" class="flex h-full w-full">
                        "Accordion"
                    </a>
                </menu::Item>
                <menu::SubRoot
                    index=4
                    class="flex relative cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted"
                >
                    <menu::Trigger class="w-full flex justify-between items-center">
                        "Sub menu" <icons::ChevronRight class="w-4"></icons::ChevronRight>
                    </menu::Trigger>
                    <menu::Content
                        class="absolute flex w-56 min-w-[8rem] flex-col rounded-md border bg-popover p-1 text-popover-foreground shadow-md focus:outline-none"
                        show_class="z-10 top-0 left-[105%] opacity-100 transition duration-150 ease-in"
                        hide_class="-z-10 top-0 left-[95%] opacity-0 transition duration-200 ease-out"
                        hide_delay={Duration::from_millis(200)}
                    >

                        <menu::Item
                            index=0
                            class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted"
                        >
                            <button
                                class="flex h-full w-full"
                                on:click={|_| log!("Profile clicked")}
                            >
                                "Profile"
                            </button>
                        </menu::Item>
                        <menu::Item
                            index=1
                            disabled=true
                            class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted"
                        >
                            "Billing"
                        </menu::Item>
                        <menu::Item
                            index=2
                            class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted"
                        >
                            "Settings"
                        </menu::Item>
                    </menu::Content>
                </menu::SubRoot>
            </menu::Content>
        </menu::Root>
    }
}
