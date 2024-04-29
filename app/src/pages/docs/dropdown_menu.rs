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
            <menu::Trigger class="inline-flex justify-center items-center w-10 h-10 text-sm font-medium rounded-full border text-foreground border-border-input bg-background-alt shadow-btn hover:bg-muted focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible focus-visible:ring-foreground focus-visible:ring-offset-background active:scale-98">
                <icons::Ellipsis class="w-6 h-6 text-foreground"></icons::Ellipsis>
            </menu::Trigger>
            <menu::Content
                class="flex absolute left-1/2 flex-col p-1 w-56 rounded-md border shadow-md min-w-[8rem] bg-popover text-popover-foreground focus:outline-none"
                show_class="z-10 -translate-x-1/2 translate-y-0 opacity-100 transition duration-150 ease-in"
                hide_class="-z-10 -translate-x-1/2 translate-y-1 opacity-0 transition duration-200 ease-out"
                hide_delay={Duration::from_millis(200)}
            >
                <menu::Item
                    index=0
                    class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none  hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[disabled]:pointer-events-none data-[disabled]:opacity-50 data-[highlighted]:bg-muted"
                >
                    <button class="flex w-full h-full" on:click={|_| log!("Profile clicked")}>
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
                    <a href="/docs/accordion" class="flex w-full h-full">
                        "Accordion"
                    </a>
                </menu::Item>
                <menu::SubRoot
                    index=4
                    class="flex relative cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted"
                >
                    <menu::Trigger class="flex justify-between items-center w-full">
                        "Sub menu" <icons::ChevronRight class="w-4"></icons::ChevronRight>
                    </menu::Trigger>
                    <menu::Content
                        class="flex absolute flex-col p-1 w-56 rounded-md border shadow-md min-w-[8rem] bg-popover text-popover-foreground focus:outline-none"
                        show_class="z-10 top-0 left-[105%] opacity-100 transition duration-150 ease-in"
                        hide_class="-z-10 top-0 left-[95%] opacity-0 transition duration-200 ease-out"
                        hide_delay={Duration::from_millis(200)}
                    >

                        <menu::Item
                            index=0
                            class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[highlighted]:bg-muted"
                        >
                            <button
                                class="flex w-full h-full"
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
