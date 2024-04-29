use leptos::*;

use crate::components::code::Code;

#[component]
pub fn GettingStartedPage() -> impl IntoView {
    let code = r#"use leptos::*;
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
                .enumerate()
                .map(|(index, (title, content))| {
                    view! {
                        <accordion::Item index>
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
        <main class="flex flex-col space-y-4">
            <h1 class="text-5xl font-semibold scroll-m-20 tracking-[-0.02em]">"Getting Started"</h1>
            <p>"Install biji-ui using Cargo."</p>
            <Code class="text-xs rounded-lg" code="cargo install biji-ui" language="bash"/>
            <p>"You can then import and start using them in your app."</p>
            <Code class="text-xs rounded-lg" code={code} language="rust"/>
        </main>
    }
}
