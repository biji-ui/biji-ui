# Biji UI

Effortless UI Components for Your Leptos Projects

Build beautiful and functional user interfaces faster with Biji UI, a collection of essential UI components designed specifically for the Leptos framework.

Seamlessly integrated with any CSS framework, Biji UI allows you to leverage the power of utility-first styling to bring your components to life with ease. Focus on building the core functionality of your application while Biji UI handles the UI foundation.

Biji UI is inspired by popular headless UI libraries like HeadlessUI for React and Melt UI for Svelte.

[Read the docs](https://biji-ui.fly.dev/)

## Usage

### Installation

Components are opt-in via feature flags. Add only what you need:

```toml
# Enable specific components
biji-ui = { version = "0.4", features = ["accordion", "dialog"] }

# Or enable everything
biji-ui = { version = "0.4", features = ["full"] }
```

Available features: `accordion`, `calendar`, `dialog`, `menu`, `menubar`, `tooltip`, `full`

### Example

```rust
use leptos::prelude::*;
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

## Developing the Docs

Prerequisites: [Rust](https://rustup.rs/), [cargo-leptos](https://github.com/leptos-rs/cargo-leptos), [pnpm](https://pnpm.io/)

```bash
# Install cargo-leptos
cargo install cargo-leptos

# Install JS dependencies (Shiki syntax highlighter)
make jspackages-install

# SSR dev server with hot reload (recommended)
make dev

# CSR-only dev server via Trunk
make dev-csr
```

The docs site (`biji-ui-docs`) supports two modes:
- **SSR** (`make dev`) — Axum server with hydration, matches production
- **CSR** (`make dev-csr`) — Trunk, WASM-only, no server required

## Version Compatibility

Biji UI | Leptos
--------|-------
0.1.x   | 0.6.x
0.2.x   | 0.7.x
0.3.x   | 0.8.x
0.4.x   | 0.8.x
