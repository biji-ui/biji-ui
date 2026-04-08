use leptos::prelude::*;

use crate::components::{
    api_table::{DataAttrRow, DataAttrsTable, PropRow, PropsTable, SectionHeading},
    code::Code,
};

const INSTALL_CODE: &str = concat!(
    "biji-ui = { version = \"",
    env!("CARGO_PKG_VERSION"),
    "\", features = [\"pagination\"] }",
);

const USAGE_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::pagination;

#[component]
pub fn MyPagination() -> impl IntoView {
    let page = RwSignal::new(1_i32);
    let total = RwSignal::new(100_i32);

    view! {
        <pagination::RootWith
            class="flex gap-2 items-center"
            page=page
            page_size=20
            total=total
            let:p
        >
            <button
                disabled=move || !p.has_prev.get()
                on:click=move |_| page.update(|n| *n -= 1)
                class="px-3 py-1.5 rounded border border-border disabled:opacity-50"
            >
                "Previous"
            </button>
            <span class="text-sm text-muted-foreground">
                {move || format!("Page {} of {}", p.page.get(), p.total_pages.get())}
            </span>
            <button
                disabled=move || !p.has_next.get()
                on:click=move |_| page.update(|n| *n += 1)
                class="px-3 py-1.5 rounded border border-border disabled:opacity-50"
            >
                "Next"
            </button>
        </pagination::RootWith>
    }
}"#;

const USE_PAGINATION_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::pagination::{self, use_pagination};

// Child component reads state from context via use_pagination()
#[component]
fn PageInfo() -> impl IntoView {
    let p = use_pagination();
    view! {
        <span class="text-sm tabular-nums text-muted-foreground">
            {move || format!(
                "Showing {}\u{2013}{} of {}",
                p.offset.get() + 1,
                (p.offset.get() + p.page_size).min(p.total.get()),
                p.total.get(),
            )}
        </span>
    }
}

#[component]
pub fn TableFooter() -> impl IntoView {
    let page = RwSignal::new(1_i32);
    let total = RwSignal::new(47_i32);

    view! {
        <pagination::Root
            class="flex justify-between items-center"
            page=page
            page_size=10
            total=total
        >
            <PageInfo />
            // ... prev/next buttons
        </pagination::Root>
    }
}"#;

const ROOT_PROPS: &[PropRow] = &[
    PropRow {
        name: "page",
        prop_type: "Signal<i32>",
        default: "—",
        description: "Current 1-indexed page number. Accepts an RwSignal or any Signal<i32>.",
    },
    PropRow {
        name: "page_size",
        prop_type: "i32",
        default: "—",
        description: "Number of items per page.",
    },
    PropRow {
        name: "total",
        prop_type: "Signal<i32>",
        default: "—",
        description: "Total number of items. Accepts a static i32 or a reactive signal.",
    },
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the root <nav> element.",
    },
];

const STATE_FIELDS: &[PropRow] = &[
    PropRow {
        name: "page",
        prop_type: "Signal<i32>",
        default: "—",
        description: "Current 1-indexed page number (mirrors the prop).",
    },
    PropRow {
        name: "page_size",
        prop_type: "i32",
        default: "—",
        description: "Items per page (mirrors the prop).",
    },
    PropRow {
        name: "total",
        prop_type: "Signal<i32>",
        default: "—",
        description: "Total item count (mirrors the prop).",
    },
    PropRow {
        name: "total_pages",
        prop_type: "Signal<i32>",
        default: "—",
        description: "Derived total number of pages.",
    },
    PropRow {
        name: "has_next",
        prop_type: "Signal<bool>",
        default: "—",
        description: "Whether a next page exists.",
    },
    PropRow {
        name: "has_prev",
        prop_type: "Signal<bool>",
        default: "—",
        description: "Whether a previous page exists.",
    },
    PropRow {
        name: "offset",
        prop_type: "Signal<i32>",
        default: "—",
        description: "0-based index of the first item on the current page. Use as a query offset.",
    },
];

const DATA_ATTRS: &[DataAttrRow] = &[
    DataAttrRow {
        name: "data-page",
        description: "The current page number as a string. Present on the root <nav> element.",
    },
    DataAttrRow {
        name: "data-total-pages",
        description: "The total number of pages as a string. Present on the root <nav> element.",
    },
];

#[component]
pub fn PaginationDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    view! {
        <DocPage title="Pagination">
            <p class="mt-3 mb-11 text-base text-balance">
                "A headless pagination state container. Derives page count, prev/next availability, and item offset from controlled "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"page"</code>
                ", "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"page_size"</code>
                ", and "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"total"</code>
                " signals."
            </p>
            <DocPreview>
                <PaginationExample />
            </DocPreview>
            <SectionHeading title="Installation" />
            <Code
                class="[&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={INSTALL_CODE}
                language="toml"
            />
            <SectionHeading title="Usage" />
            <Code
                class="[&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={USAGE_CODE}
                language="rust"
            />
            <SectionHeading title="use_pagination()" />
            <p class="mb-4 text-sm text-muted-foreground">
                "Use "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"use_pagination()"</code>
                " inside any descendant of "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"<Root>"</code>
                " to retrieve "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"PaginationState"</code>
                " from context. Prefer this when your controls live in a separate component."
            </p>
            <DocPreview>
                <UsePaginationExample />
            </DocPreview>
            <Code
                class="mt-4 [&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={USE_PAGINATION_CODE}
                language="rust"
            />
            <SectionHeading title="API Reference" />
            <PropsTable title="Root / RootWith" rows={ROOT_PROPS} />
            <PropsTable title="PaginationState" rows={STATE_FIELDS} />
            <DataAttrsTable rows={DATA_ATTRS} />
        </DocPage>
    }
}

#[component]
pub fn PaginationExample() -> impl IntoView {
    use biji_ui::components::pagination;

    let page = RwSignal::new(1_i32);
    let total = Signal::from(47_i32);
    const PAGE_SIZE: i32 = 5;

    const BTN: &str = "inline-flex items-center justify-center h-9 min-w-9 px-3 rounded-md border \
        border-border bg-background text-sm font-medium transition-colors \
        hover:bg-accent hover:text-accent-foreground \
        disabled:opacity-50 disabled:pointer-events-none";

    view! {
        <div class="flex flex-col gap-4 w-full sm:max-w-[70%] mx-auto">
            <pagination::RootWith
                page=page
                page_size={PAGE_SIZE}
                total=total
                let:p
            >
                <div class="divide-y divide-border rounded-lg border border-border overflow-hidden text-sm">
                    {move || {
                        let start = p.offset.get();
                        let end = (start + p.page_size).min(p.total.get());
                        (start + 1..=end)
                            .map(|i| view! {
                                <div class="flex items-center justify-between px-4 py-2.5 hover:bg-muted/50">
                                    <span class="font-medium">{format!("Item {:02}", i)}</span>
                                    <span class="text-xs text-muted-foreground">{format!("#{}", i)}</span>
                                </div>
                            })
                            .collect_view()
                    }}
                </div>
                <div class="flex flex-col gap-2 items-center sm:flex-row sm:justify-between">
                    <p class="text-sm text-muted-foreground">
                        {move || format!(
                            "Showing {}\u{2013}{} of {}",
                            p.offset.get() + 1,
                            (p.offset.get() + p.page_size).min(p.total.get()),
                            p.total.get(),
                        )}
                    </p>
                    <div class="flex gap-1.5 items-center">
                        <button
                            class={BTN}
                            disabled=move || !p.has_prev.get()
                            on:click=move |_| page.update(|n| *n -= 1)
                        >
                            "Previous"
                        </button>
                        <span class="text-sm text-muted-foreground tabular-nums px-1">
                            {move || format!("{} / {}", p.page.get(), p.total_pages.get())}
                        </span>
                        <button
                            class={BTN}
                            disabled=move || !p.has_next.get()
                            on:click=move |_| page.update(|n| *n += 1)
                        >
                            "Next"
                        </button>
                    </div>
                </div>
            </pagination::RootWith>
        </div>
    }
}

#[component]
fn ItemRangeLabel() -> impl IntoView {
    use biji_ui::components::pagination::use_pagination;
    let p = use_pagination();
    view! {
        <span class="text-sm text-muted-foreground tabular-nums">
            {move || format!(
                "Showing {}\u{2013}{} of {}",
                p.offset.get() + 1,
                (p.offset.get() + p.page_size).min(p.total.get()),
                p.total.get(),
            )}
        </span>
    }
}

#[component]
pub fn UsePaginationExample() -> impl IntoView {
    use biji_ui::components::pagination::{self, use_pagination};

    let page = RwSignal::new(1_i32);
    let total = Signal::from(47_i32);

    const BTN: &str = "inline-flex items-center justify-center h-9 px-3 rounded-md border \
        border-border bg-background text-sm font-medium transition-colors \
        hover:bg-accent hover:text-accent-foreground \
        disabled:opacity-50 disabled:pointer-events-none";

    view! {
        <div class="w-full sm:max-w-[70%]">
            <pagination::Root
                class="flex justify-between items-center"
                page=page
                page_size=10
                total=total
            >
                <ItemRangeLabel />
                <div class="flex gap-1.5">
                    <button
                        class={BTN}
                        disabled=move || !use_pagination().has_prev.get()
                        on:click=move |_| page.update(|n| *n -= 1)
                    >
                        "Previous"
                    </button>
                    <button
                        class={BTN}
                        disabled=move || !use_pagination().has_next.get()
                        on:click=move |_| page.update(|n| *n += 1)
                    >
                        "Next"
                    </button>
                </div>
            </pagination::Root>
        </div>
    }
}
