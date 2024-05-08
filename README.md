# Biji UI

Effortless UI Components for Your Leptos Projects

Build beautiful and functional user interfaces faster with Biji UI, a collection of essential UI components designed specifically for the Leptos framework.

Seamlessly integrated with any CSS framework, Biji UI allows you to leverage the power of utility-first styling to bring your components to life with ease. Focus on building the core functionality of your application while Biji UI handles the UI foundation.

Biji UI is inspired by popular headless UI libraries like HeadlessUI for React and Melt UI for Svelte.

[Read the docs](https://biji-ui.fly.dev/)

## Usage

Installation

```bash
cargo add biji-ui
```

Example

```rust
use leptos::*;
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
                            <accordion::Toggle>
                                {title}
                                <span>
                                    <icons::Caret>
                                </span>
                            </accordion::Toggle>
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
```

