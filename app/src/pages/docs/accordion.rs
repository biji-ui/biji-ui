use std::time::Duration;

use leptos::prelude::*;

use crate::icons;

#[component]
pub fn AccordionDocPage() -> impl IntoView {
    use crate::pages::docs::DocPage;

    view! {
        <DocPage
            title="Accordion"
            description="An accordion is a vertically stacked list of items, such as labels or thumbnails, that can be toggled to reveal or hide additional content."
            example={AccordionExample}
        />
    }
}

#[component]
pub fn AccordionExample() -> impl IntoView {
    use biji_ui::components::accordion;

    let items = [
        ("What is the meaning of life?", "To become a better person, to help others, and to leave the world a better place than you found it."),
        ("How do I become a better person?", "Read books, listen to podcasts, and surround yourself with people who inspire you."),
        ("What is the best way to help others?", "Give them your time, attention, and love."),
    ];

    view! {
        <accordion::Root class="w-full sm:max-w-[70%]">
            {items
                .into_iter()
                .map(|(title, content)| {
                    view! {
                        <accordion::Item class="border-b group border-dark-10">
                            <accordion::Toggle class="flex w-full px-1.5 flex-1 items-center justify-between py-5 text-[15px] font-medium outline-none transition-all focus:rounded-xl focus:outline-none [&[data-state=open]>span>svg]:rotate-180 data-[highlighted]:bg-muted !ring-0 !ring-transparent">
                                {title}
                                <span class="inline-flex size-8 items-center justify-center rounded-[7px] bg-transparent transition-all hover:bg-dark-10">
                                    <icons::Caret class="size-[18px] transition-all duration-200"></icons::Caret>
                                </span>
                            </accordion::Toggle>
                            <accordion::Content
                                class="px-1.5 pb-[25px] pt-1.5 text-sm tracking-[-0.01em]"
                                show_class="opacity-100 transition duration-150 ease-in"
                                hide_class="opacity-0 transition duration-200 ease-out"
                                hide_delay={Duration::from_millis(200)}
                            >
                                {content}
                            </accordion::Content>
                        </accordion::Item>
                    }
                })
                .collect::<Vec<_>>()}
        </accordion::Root>
    }
}
