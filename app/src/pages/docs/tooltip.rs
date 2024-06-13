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
pub fn Content() -> impl IntoView {
    use biji_ui::components::tooltip;
    view! {
        <tooltip::Content
            class="inline-block w-max rounded-lg border border-slate-500 bg-gray-900 px-3 py-2 text-sm font-medium text-white shadow-sm transition-opacity duration-200 dark:bg-gray-700"
            hide_class="opacity-0"
            show_class="opacity-100"
        >
            <tooltip::Arrow class="rounded-[2px] border-l border-t border-slate-500 border-dark-10"></tooltip::Arrow>
            "Hello,"
            <br/>
            "Massive World!"
        </tooltip::Content>
    }
}

pub const BUTTON_BASE_STYLE: &str = "inline-flex h-10 w-full items-center justify-center whitespace-nowrap rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground ring-offset-background transition-colors focus-visible:outline-none hover:bg-primary/90 focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";

#[component]
pub fn TooltipExample() -> impl IntoView {
    use biji_ui::components::tooltip;
    view! {
        <div class="grid grid-cols-3 gap-2">
            <tooltip::Root positioning={tooltip::Positioning::TopStart}>
                <tooltip::Trigger class={BUTTON_BASE_STYLE}>"Top start"</tooltip::Trigger>
                <Portal>
                    <Content/>
                </Portal>
            </tooltip::Root>
            <tooltip::Root positioning={tooltip::Positioning::Top}>
                <tooltip::Trigger class={BUTTON_BASE_STYLE}>"Top"</tooltip::Trigger>
                <Portal>
                    <Content/>
                </Portal>
            </tooltip::Root>
            <tooltip::Root positioning={tooltip::Positioning::TopEnd}>
                <tooltip::Trigger class={BUTTON_BASE_STYLE}>"Top end"</tooltip::Trigger>
                <Portal>
                    <Content/>
                </Portal>
            </tooltip::Root>
            <tooltip::Root positioning={tooltip::Positioning::RightStart}>
                <tooltip::Trigger class={BUTTON_BASE_STYLE}>"Right start"</tooltip::Trigger>
                <Portal>
                    <Content/>
                </Portal>
            </tooltip::Root>
            <tooltip::Root positioning={tooltip::Positioning::Right}>
                <tooltip::Trigger class={BUTTON_BASE_STYLE}>"Right"</tooltip::Trigger>
                <Portal>
                    <Content/>
                </Portal>
            </tooltip::Root>
            <tooltip::Root positioning={tooltip::Positioning::RightEnd}>
                <tooltip::Trigger class={BUTTON_BASE_STYLE}>"Right end"</tooltip::Trigger>
                <Portal>
                    <Content/>
                </Portal>
            </tooltip::Root>
            <tooltip::Root positioning={tooltip::Positioning::BottomStart}>
                <tooltip::Trigger class={BUTTON_BASE_STYLE}>"Bottom start"</tooltip::Trigger>
                <Portal>
                    <Content/>
                </Portal>
            </tooltip::Root>
            <tooltip::Root positioning={tooltip::Positioning::Bottom}>
                <tooltip::Trigger class={BUTTON_BASE_STYLE}>"Bottom"</tooltip::Trigger>
                <Portal>
                    <Content/>
                </Portal>
            </tooltip::Root>
            <tooltip::Root positioning={tooltip::Positioning::BottomEnd}>
                <tooltip::Trigger class={BUTTON_BASE_STYLE}>"Bottom end"</tooltip::Trigger>
                <Portal>
                    <Content/>
                </Portal>
            </tooltip::Root>
            <tooltip::Root positioning={tooltip::Positioning::LeftStart}>
                <tooltip::Trigger class={BUTTON_BASE_STYLE}>"Left start"</tooltip::Trigger>
                <Portal>
                    <Content/>
                </Portal>
            </tooltip::Root>
            <tooltip::Root positioning={tooltip::Positioning::Left}>
                <tooltip::Trigger class={BUTTON_BASE_STYLE}>"Left"</tooltip::Trigger>
                <Portal>
                    <Content/>
                </Portal>
            </tooltip::Root>
            <tooltip::Root positioning={tooltip::Positioning::LeftEnd}>
                <tooltip::Trigger class={BUTTON_BASE_STYLE}>"Left end"</tooltip::Trigger>
                <Portal>
                    <Content/>
                </Portal>
            </tooltip::Root>
        </div>
    }
}
