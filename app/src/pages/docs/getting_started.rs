use leptos::prelude::*;

use crate::components::code::Code;

#[component]
pub fn GettingStartedPage() -> impl IntoView {
    let code = r#"use leptos::prelude::*;
use biji_ui::components::accordion;

#[component]
pub fn AccordionExample() -> impl IntoView {

    let items = [
        ("Item 1", "Content 1"),
        ("Item 2", "Content 2"),
        ("Item 3", "Content 3"),
    ];

    view! {
        <accordion::Root>
            {items
                .into_iter()
                .map(|(title, content)| {
                    view! {
                        <accordion::Item>
                            <accordion::Trigger>
                                {title}
                                <span>
                                    <icons::Caret>
                                </span>
                            </accordion::Trigger>
                            <accordion::Content>
                                {content}
                            </accordion::Content>
                        </accordion::Item>
                    }
                })
                .collect::<Vec<_>>()}
        </accordion::Root>
    }
}
"#;

    view! {
        <article class="flex h-full flex-col pt-16 pb-10">
            <h1 class="mb-2 text-2xl font-bold">"Getting Started"</h1>
            <p class="my-5 text-base">"Install biji-ui using Cargo."</p>
            <Code class="text-xs" code="cargo install biji-ui" language="bash" />
            <p class="my-5 text-base">"You can then import and start using them in your app."</p>
            <Code class="text-xs" code={code} language="rust" />
        </article>
    }
}
