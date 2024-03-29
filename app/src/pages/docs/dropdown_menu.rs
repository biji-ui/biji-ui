use leptos::*;

#[component]
pub fn DropdownMenuDocPage() -> impl IntoView {
    use crate::pages::docs::DocPage;

    view! {
        <DocPage
            title="Dropdown Menu"
            description="Displays a menu of items that users can select from when triggered."
            example={|| view! { <DropdownMenuExample/> }}
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
                "..."
            </menu::Trigger>
            <menu::Content
                class="absolute left-1/2 flex w-56 min-w-[8rem] flex-col rounded-md border bg-popover p-1 text-popover-foreground shadow-md focus:outline-none"
                show_class="z-10 translate-y-0 -translate-x-1/2 opacity-100 transition duration-150 ease-in"
                hide_class="-z-10 translate-y-1 -translate-x-1/2 opacity-0 transition duration-200 ease-out"
                hide_delay={Duration::from_millis(200)}
            >
                <menu::Item class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground">
                    "Profile"
                </menu::Item>
                <menu::Item class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground">
                    "Billing"
                </menu::Item>
                <menu::Item
                    attr:data-disabled=true
                    class="flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 hover:bg-accent hover:text-accent-foreground"
                >
                    "Settings"
                </menu::Item>
            </menu::Content>
        </menu::Root>
    }
}
