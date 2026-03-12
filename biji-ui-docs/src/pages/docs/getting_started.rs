use leptos::prelude::*;
use leptos_meta::*;

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

    let install_specific = concat!(
        "# Enable only the components you need\n",
        "biji-ui = { version = \"",
        env!("CARGO_PKG_VERSION"),
        "\", features = [\"accordion\", \"dialog\"] }\n\n",
        "# Or enable everything\n",
        "biji-ui = { version = \"",
        env!("CARGO_PKG_VERSION"),
        "\", features = [\"full\"] }",
    );

    view! {
        <Title text="Getting Started — Biji UI" />
        <Meta name="description" content="Get started with Biji UI — a headless, accessible component library for Leptos. Install via Cargo and add components with feature flags." />
        <article class="flex flex-col pt-16 pb-10 h-full">
            <h1 class="mb-2 text-2xl font-bold">"Getting Started"</h1>
            <p class="my-5 text-base">"Add biji-ui to your Cargo.toml. Components are opt-in via feature flags."</p>
            <Code
                class="[&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={install_specific}
                language="toml"
            />
            <p class="my-5 text-base">"You can then import and start using them in your app."</p>
            <Code
                class="[&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={code}
                language="rust"
            />
        </article>
    }
}
