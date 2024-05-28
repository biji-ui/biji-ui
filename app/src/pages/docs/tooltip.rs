use leptos::*;

#[component]
pub fn TooltipDocPage() -> impl IntoView {
    use crate::pages::docs::DocPage;

    view! {
        <DocPage
            title="Tooltip"
            description="A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it."
            example={TooltipExample}
        />
    }
}

#[component]
pub fn TooltipExample() -> impl IntoView {
    use biji_ui::components::tooltip;
    view! {
        <tooltip::Root class="relative">
            <tooltip::Trigger>"Hover me"</tooltip::Trigger>
            <tooltip::Content
                class="absolute z-10 inline-block rounded-lg bg-gray-900 px-3 py-2 text-sm font-medium text-white shadow-sm transition-opacity duration-200 left-[50%] bottom-full translate-x-[-50%] dark:bg-gray-700 w-max"
                hide_class="opacity-0"
                show_class="opacity-100"
            >
                "Hello, world!"
            </tooltip::Content>
        </tooltip::Root>
    }
}
