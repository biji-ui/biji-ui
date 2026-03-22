use leptos::prelude::*;

use crate::components::{
    api_table::{
        DataAttrRow, DataAttrsTable, KeyboardRow, KeyboardTable, PropRow, PropsTable,
        SectionHeading,
    },
    code::Code,
};

const INSTALL_CODE: &str = concat!(
    "biji-ui = { version = \"",
    env!("CARGO_PKG_VERSION"),
    "\", features = [\"combobox\"] }",
);

const USAGE_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::combobox;

#[component]
pub fn MyCombobox() -> impl IntoView {
    view! {
        <combobox::Root inline=true>
            <combobox::InputTrigger
                class="py-2 px-3 w-48 text-sm rounded-md border outline-none border-border bg-background"
                placeholder="Search a fruit..."
            />
            <combobox::Content class="overflow-hidden z-50 py-1 w-48 text-sm rounded-md border shadow-md border-border bg-background">
                <combobox::Empty>
                    <div class="py-4 px-3 text-center text-muted-foreground">"No results."</div>
                </combobox::Empty>
                <combobox::Item
                    value="apple"
                    label="Apple"
                    class="py-1.5 px-3 cursor-default outline-none select-none data-[highlighted]:bg-accent"
                >
                    "Apple"
                </combobox::Item>
                <combobox::Item
                    value="banana"
                    label="Banana"
                    class="py-1.5 px-3 cursor-default outline-none select-none data-[highlighted]:bg-accent"
                >
                    "Banana"
                </combobox::Item>
            </combobox::Content>
        </combobox::Root>
    }
}"#;

const USAGE_CODE_BUTTON: &str = r#"// Alternative: button trigger with search inside the dropdown
<combobox::Root>
    <combobox::Trigger class="...">
        <combobox::Value placeholder="Select a fruit..." />
        "▾"
    </combobox::Trigger>
    <combobox::Content class="...">
        <combobox::Input placeholder="Search..." class="..." />
        <combobox::Item value="apple" label="Apple" class="...">
            "Apple"
        </combobox::Item>
    </combobox::Content>
</combobox::Root>"#;

const ROOT_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the root wrapper element.",
    },
    PropRow {
        name: "value",
        prop_type: "Option<String>",
        default: "None",
        description: "The initially selected value.",
    },
    PropRow {
        name: "positioning",
        prop_type: "Positioning",
        default: "BottomStart",
        description: "Where the dropdown panel appears relative to the trigger.",
    },
    PropRow {
        name: "hide_delay",
        prop_type: "Duration",
        default: "200ms",
        description: "Delay before the panel is removed from the DOM after closing.",
    },
    PropRow {
        name: "avoid_collisions",
        prop_type: "AvoidCollisions",
        default: "Flip",
        description: "Strategy used to keep the panel within the viewport.",
    },
    PropRow {
        name: "on_value_change",
        prop_type: "Option<Callback<String>>",
        default: "None",
        description: "Fired with the selected value when selection changes.",
    },
    PropRow {
        name: "inline",
        prop_type: "bool",
        default: "false",
        description: "Set to true when using InputTrigger (the input sits above the dropdown).",
    },
];

const TRIGGER_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the trigger button.",
}];

const INPUT_TRIGGER_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the inline input trigger.",
    },
    PropRow {
        name: "placeholder",
        prop_type: "String",
        default: "\"\"",
        description: "Placeholder text shown when nothing is selected.",
    },
];

const VALUE_PROPS: &[PropRow] = &[PropRow {
    name: "placeholder",
    prop_type: "String",
    default: "\"\"",
    description: "Text shown when no item is selected.",
}];

const CONTENT_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the floating dropdown panel.",
    },
    PropRow {
        name: "show_class",
        prop_type: "String",
        default: "\"\"",
        description: "Extra class added while the panel is visible.",
    },
    PropRow {
        name: "hide_class",
        prop_type: "String",
        default: "\"\"",
        description: "Extra class added while the panel is animating out.",
    },
];

const INPUT_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the search input.",
    },
    PropRow {
        name: "placeholder",
        prop_type: "String",
        default: "\"\"",
        description: "Placeholder text for the search input.",
    },
];

const ITEM_PROPS: &[PropRow] = &[
    PropRow {
        name: "value",
        prop_type: "String",
        default: "—",
        description: "The value submitted when this item is selected.",
    },
    PropRow {
        name: "label",
        prop_type: "Option<String>",
        default: "value",
        description: "Display label used for filtering and shown in the trigger. Defaults to value.",
    },
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the item element.",
    },
    PropRow {
        name: "disabled",
        prop_type: "bool",
        default: "false",
        description: "When true, the item cannot be selected.",
    },
];

const DATA_ATTRS: &[DataAttrRow] = &[
    DataAttrRow {
        name: "data-state",
        description: "\"open\" or \"closed\". Present on Trigger.",
    },
    DataAttrRow {
        name: "data-state",
        description: "\"checked\" or \"unchecked\". Present on Item.",
    },
    DataAttrRow {
        name: "data-highlighted",
        description: "Present on Item when it has keyboard focus or the mouse is over it.",
    },
    DataAttrRow {
        name: "data-disabled",
        description: "Present on Item when it is disabled.",
    },
];

const KEYBOARD: &[KeyboardRow] = &[
    KeyboardRow {
        key: "ArrowDown",
        description: "Moves focus to the next visible item.",
    },
    KeyboardRow {
        key: "ArrowUp",
        description: "Moves focus to the previous visible item.",
    },
    KeyboardRow {
        key: "Home",
        description: "Moves focus to the first visible item.",
    },
    KeyboardRow {
        key: "End",
        description: "Moves focus to the last visible item.",
    },
    KeyboardRow {
        key: "Enter",
        description: "Selects the focused item and closes the dropdown.",
    },
    KeyboardRow {
        key: "Escape",
        description: "Closes the dropdown and returns focus to the trigger.",
    },
    KeyboardRow {
        key: "Tab",
        description: "Closes the dropdown.",
    },
];

const ROOT_WITH_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::combobox;

#[component]
pub fn MyCombobox() -> impl IntoView {
    view! {
        <combobox::RootWith let:c>
            <p class="text-sm text-muted-foreground">
                {move || {
                    c.value.get()
                        .map(|v| format!("Selected: {v}"))
                        .unwrap_or_else(|| "Nothing selected".to_string())
                }}
            </p>
            <combobox::Trigger class="...">
                <combobox::Value placeholder="Select a fruit..." />
            </combobox::Trigger>
            <combobox::Content class="..." show_class="..." hide_class="...">
                <combobox::Input placeholder="Search..." class="..." />
                <combobox::Item value="apple" class="...">"Apple"</combobox::Item>
                <combobox::Item value="banana" class="...">"Banana"</combobox::Item>
            </combobox::Content>
        </combobox::RootWith>
    }
}"#;

#[component]
pub fn ComboboxDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    view! {
        <DocPage title="Combobox">
            <p class="mt-3 mb-11 text-base text-balance">
                "A searchable input that filters a list of options. Type to narrow results, navigate with arrow keys, and select with Enter or click."
            </p>
            <DocPreview>
                <ComboboxInlineExample />
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
            <SectionHeading title="Button Trigger Variant" />
            <p class="mb-4 text-sm text-muted-foreground">
                "For a select-style trigger with a hidden search field that appears inside the dropdown, omit "
                <code class="font-mono text-foreground">{"inline"}</code> " and use "
                <code class="font-mono text-foreground">{"Trigger"}</code> " + "
                <code class="font-mono text-foreground">{"Value"}</code> " + "
                <code class="font-mono text-foreground">{"Input"}</code>
                " (inside Content) instead."
            </p>
            <Code
                class="[&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={USAGE_CODE_BUTTON}
                language="rust"
            />
            <DocPreview>
                <ComboboxExample />
            </DocPreview>
            {
                #[cfg(not(feature = "csr"))]
                {
                    view! {
                        <SectionHeading title="Async / Server-side Search" />
                        <p class="mb-3 text-sm text-muted-foreground">
                            "Items are fetched from a Leptos "
                            <code class="font-mono text-foreground">{"#[server]"}</code>
                            " function and rendered inside a dedicated "
                            <code class="font-mono text-foreground">{"AsyncItems"}</code>
                            " component placed directly inside "
                            <code class="font-mono text-foreground">{"Content"}</code>
                            ". The component owns pagination state and wires infinite scroll itself."
                        </p>
                        <p class="mb-3 text-sm text-muted-foreground">
                            "The query is debounced 300 ms before triggering a new call. " "An "
                            <code class="font-mono text-foreground">{"on_loading_change"}</code>
                            " callback fires "
                            <code class="font-mono text-foreground">{"true"}</code>
                            " when a fetch starts and "
                            <code class="font-mono text-foreground">{"false"}</code>
                            " when it settles, so the parent can render a spinner anywhere — here it appears on the right edge of the input."
                        </p>
                        <p class="mb-4 text-sm text-muted-foreground">
                            "Infinite scroll is handled by an "
                            <code class="font-mono text-foreground">{"IntersectionObserver"}</code>
                            " on a sentinel element at the bottom of the list. When the sentinel becomes visible the next page is fetched and appended without re-rendering existing items."
                        </p>
                        <Code
                            class="[&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                            code={ASYNC_EXAMPLE_CODE}
                            language="rust"
                        />
                        <DocPreview>
                            <ComboboxAsyncExample />
                        </DocPreview>
                    }
                        .into_any()
                }
                #[cfg(feature = "csr")] { ().into_any() }
            }
            <SectionHeading title="RootWith" />
            <p class="mb-5 text-sm text-muted-foreground">
                "Use "
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"RootWith"</code>
                " to access "
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"ComboboxState"</code>
                " inline via the "
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"let:"</code>
                " binding. The state exposes "
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"open"</code>
                ", "
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"value"</code>
                ", and "
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"query"</code>
                " as reactive signals."
            </p>
            <DocPreview>
                <ComboboxRootWithExample />
            </DocPreview>
            <Code
                class="[&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={ROOT_WITH_CODE}
                language="rust"
            />
            <SectionHeading title="API Reference" />
            <PropsTable title="Root / RootWith" rows={ROOT_PROPS} />
            <PropsTable title="InputTrigger" rows={INPUT_TRIGGER_PROPS} />
            <PropsTable title="Trigger" rows={TRIGGER_PROPS} />
            <PropsTable title="Value" rows={VALUE_PROPS} />
            <PropsTable title="Content" rows={CONTENT_PROPS} />
            <PropsTable title="Input" rows={INPUT_PROPS} />
            <PropsTable title="Item" rows={ITEM_PROPS} />
            <DataAttrsTable rows={DATA_ATTRS} />
            <KeyboardTable rows={KEYBOARD} />
        </DocPage>
    }
}

#[component]
pub fn ComboboxRootWithExample() -> impl IntoView {
    use biji_ui::components::combobox;

    const TRIGGER_CLS: &str = "flex items-center justify-between w-56 px-3 py-2 text-sm \
        rounded-md border border-border bg-background text-foreground \
        hover:bg-accent hover:text-accent-foreground \
        data-[state=open]:bg-accent data-[state=open]:text-accent-foreground \
        cursor-default select-none";

    const CONTENT_CLS: &str = "z-50 w-56 overflow-hidden rounded-md border border-border \
        bg-background shadow-md text-sm \
        transition origin-[var(--biji-transform-origin)]";

    const INPUT_CLS: &str = "w-full px-3 py-2 text-sm outline-none bg-transparent \
        border-b border-border placeholder:text-muted-foreground";

    const ITEM_CLS: &str = "flex items-center px-3 py-2 cursor-default select-none \
        data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground \
        data-[state=checked]:font-medium";

    const FRUITS: &[(&str, &str)] = &[
        ("apple", "Apple"),
        ("banana", "Banana"),
        ("cherry", "Cherry"),
        ("mango", "Mango"),
        ("pineapple", "Pineapple"),
    ];

    view! {
        <div class="flex flex-col items-center gap-3 p-8">
            <combobox::RootWith let:c>
                <p class="text-sm text-muted-foreground">
                    {move || {
                        c.value
                            .get()
                            .map(|v| format!("Selected: {v}"))
                            .unwrap_or_else(|| "Nothing selected".to_string())
                    }}
                </p>
                <combobox::Trigger class={TRIGGER_CLS}>
                    <combobox::Value placeholder="Select a fruit..." />
                    <svg class="h-4 w-4 opacity-50" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7"/>
                    </svg>
                </combobox::Trigger>
                <combobox::Content
                    class={CONTENT_CLS}
                    show_class="opacity-100 scale-100 duration-150 ease-out"
                    hide_class="opacity-0 scale-95 duration-100 ease-in"
                >
                    <combobox::Input placeholder="Search..." class={INPUT_CLS} />
                    {FRUITS
                        .iter()
                        .map(|(value, label)| {
                            view! {
                                <combobox::Item value={*value} label={*label} class={ITEM_CLS}>
                                    <combobox::ItemText>{*label}</combobox::ItemText>
                                </combobox::Item>
                            }
                        })
                        .collect::<Vec<_>>()}
                    <combobox::Empty>
                        <p class="px-3 py-2 text-sm text-muted-foreground">"No results."</p>
                    </combobox::Empty>
                </combobox::Content>
            </combobox::RootWith>
        </div>
    }
}

#[component]
pub fn ComboboxExample() -> impl IntoView {
    use biji_ui::components::combobox;

    const TRIGGER_CLS: &str = "flex items-center justify-between w-56 px-3 py-2 text-sm \
        rounded-md border border-border bg-background text-foreground \
        hover:bg-accent hover:text-accent-foreground \
        data-[state=open]:bg-accent data-[state=open]:text-accent-foreground \
        cursor-default select-none";

    const CONTENT_CLS: &str = "z-50 w-56 overflow-hidden rounded-md border border-border \
        bg-background shadow-md text-sm \
        transition origin-[var(--biji-transform-origin)]";

    const INPUT_CLS: &str = "w-full px-3 py-2 text-sm outline-none bg-transparent \
        border-b border-border placeholder:text-muted-foreground";

    const ITEM_CLS: &str = "flex items-center px-3 py-1.5 cursor-default select-none outline-none \
        hover:bg-accent hover:text-accent-foreground \
        data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground \
        data-[disabled]:pointer-events-none data-[disabled]:opacity-50";

    let fruits = [
        ("apple", "Apple"),
        ("apricot", "Apricot"),
        ("avocado", "Avocado"),
        ("banana", "Banana"),
        ("blueberry", "Blueberry"),
        ("cherry", "Cherry"),
        ("grape", "Grape"),
        ("kiwi", "Kiwi"),
        ("lemon", "Lemon"),
        ("mango", "Mango"),
        ("orange", "Orange"),
        ("peach", "Peach"),
        ("pear", "Pear"),
        ("pineapple", "Pineapple"),
        ("strawberry", "Strawberry"),
    ];

    view! {
        <div class="flex flex-col gap-3 items-center">
            <combobox::RootWith let:c>
                <combobox::Trigger class={TRIGGER_CLS}>
                    <combobox::Value placeholder="Select a fruit..." />
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="14"
                        height="14"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="text-muted-foreground"
                    >
                        <path d="m7 15 5 5 5-5" />
                        <path d="m7 9 5-5 5 5" />
                    </svg>
                </combobox::Trigger>
                <combobox::Content
                    class={CONTENT_CLS}
                    show_class="opacity-100 scale-100"
                    hide_class="opacity-0 scale-95"
                >
                    <combobox::Input class={INPUT_CLS} placeholder="Search fruit..." />
                    <div class="overflow-y-auto py-1 max-h-60">
                        <combobox::Empty>
                            <div class="py-6 px-3 text-sm text-center text-muted-foreground">
                                "No fruit found."
                            </div>
                        </combobox::Empty>
                        {fruits
                            .into_iter()
                            .map(|(value, label)| {
                                view! {
                                    <combobox::Item value={value} label={label} class={ITEM_CLS}>
                                        <span class="flex-1">{label}</span>
                                        <combobox::ItemIndicator>
                                            <svg
                                                xmlns="http://www.w3.org/2000/svg"
                                                width="14"
                                                height="14"
                                                viewBox="0 0 24 24"
                                                fill="none"
                                                stroke="currentColor"
                                                stroke-width="2.5"
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                            >
                                                <path d="M20 6 9 17l-5-5" />
                                            </svg>
                                        </combobox::ItemIndicator>
                                    </combobox::Item>
                                }
                            })
                            .collect_view()}
                    </div>
                </combobox::Content>
                <p class="text-xs text-muted-foreground">
                    "Selected: "
                    <span class="font-medium text-foreground">
                        {move || c.value.get().unwrap_or_else(|| "None".to_string())}
                    </span>
                </p>
            </combobox::RootWith>
        </div>
    }
}

// ── Async / server-side search ──────────────────────────────────────────────

#[allow(dead_code)]
const ASYNC_SSR_CODE: &str = r#"// Returns one extra item to detect whether more pages exist.
#[server]
pub async fn search_countries(
    query: String,
    page: u32,
    per_page: u32,
) -> Result<(Vec<(String, String)>, bool), ServerFnError> {
    let countries: &[(&str, &str)] = &[("de", "Germany"), ("fr", "France") /* … */];
    let q = query.to_lowercase();
    let skip = (page * per_page) as usize;
    let mut batch: Vec<_> = countries
        .iter()
        .filter(|(_, n)| q.is_empty() || n.to_lowercase().contains(&q))
        .skip(skip)
        .take(per_page as usize + 1)   // one extra → has_more signal
        .collect();
    let has_more = batch.len() > per_page as usize;
    batch.truncate(per_page as usize);
    Ok((batch.into_iter().map(|(v, l)| (v.to_string(), l.to_string())).collect(), has_more))
}

// AsyncItems owns the scroll container and wires infinite scroll itself.
// `on_loading_change` lets the parent show a header spinner.
#[component]
fn AsyncItems(
    #[prop(optional)] on_loading_change: Option<Callback<bool>>,
) -> impl IntoView {
    use biji_ui::components::combobox::{self, ComboboxState};
    use leptos_use::{use_debounce_fn_with_arg, use_intersection_observer};

    let ctx = expect_context::<ComboboxState>();

    // (query, page) drives the resource — query resets page to 0.
    let query_and_page = RwSignal::new((String::new(), 0u32));

    let update_query = use_debounce_fn_with_arg(
        move |q: String| {
            query_and_page.set((q, 0));
            if let Some(cb) = on_loading_change { cb.run(true); }
        },
        300.0,
    );
    Effect::new(move |_| { update_query(ctx.query.get()); });

    let results = Resource::new(
        move || query_and_page.get(),
        |(q, p)| search_countries(q, p, 8),
    );

    let items = RwSignal::<Vec<(String, String)>>::new(vec![]);
    let fetch_error = RwSignal::<Option<String>>::new(None);
    let has_more = RwSignal::new(false);
    let is_fetching = RwSignal::new(false);

    Effect::new(move |_| match results.get() {
        None => {}
        Some(Ok((batch, more))) => {
            if query_and_page.get_untracked().1 == 0 { items.set(batch); }
            else { items.update(|v| v.extend(batch)); }
            has_more.set(more);
            fetch_error.set(None);
            is_fetching.set(false);
            if let Some(cb) = on_loading_change { cb.run(false); }
        }
        Some(Err(e)) => {
            if query_and_page.get_untracked().1 == 0 { items.set(vec![]); }
            fetch_error.set(Some(e.to_string()));
            has_more.set(false);
            is_fetching.set(false);
            if let Some(cb) = on_loading_change { cb.run(false); }
        }
    });

    // Guarded load_more — no-op if already in flight or no more pages.
    let load_more = move || {
        if is_fetching.get_untracked() || !has_more.get_untracked() { return; }
        is_fetching.set(true);
        query_and_page.update(|(_, p)| *p += 1);
        if let Some(cb) = on_loading_change { cb.run(true); }
    };

    // Sentinel div at the bottom of the list.
    // IntersectionObserver fires load_more when it scrolls into view.
    // Always rendered (but hidden via `hidden` class) so the observer
    // can detect when it transitions from hidden→visible.
    let sentinel_ref = NodeRef::<html::Div>::new();
    use_intersection_observer(sentinel_ref, move |entries, _| {
        if entries.first().map(|e| e.is_intersecting()).unwrap_or(false) {
            load_more();
        }
    });

    view! {
        <div class="overflow-y-auto py-1 max-h-60">
            // … items + sentinel …
        </div>
    }
}

// Parent wires up the header spinner wherever it wants:
#[component]
pub fn MyAsyncCombobox() -> impl IntoView {
    let is_loading = RwSignal::new(false);
    view! {
        <combobox::Root inline=true>
            <div class="relative">
                <combobox::InputTrigger class="…" placeholder="Search…" />
                <Show when=move || is_loading.get()>
                    <div class="absolute right-2 top-1/2 -translate-y-1/2">
                        // your spinner here
                    </div>
                </Show>
            </div>
            <combobox::Content class="…">
                <AsyncItems on_loading_change={Callback::new(move |v| is_loading.set(v))} />
            </combobox::Content>
        </combobox::Root>
    }
}"#;

#[allow(dead_code)]
const ASYNC_EXAMPLE_CODE: &str = ASYNC_SSR_CODE;

#[server]
pub async fn search_countries(
    query: String,
    page: u32,
    per_page: u32,
) -> Result<(Vec<(String, String)>, bool), ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(80)).await;

    static COUNTRIES: &[(&str, &str)] = &[
        ("af", "Afghanistan"),
        ("al", "Albania"),
        ("dz", "Algeria"),
        ("ar", "Argentina"),
        ("au", "Australia"),
        ("at", "Austria"),
        ("be", "Belgium"),
        ("br", "Brazil"),
        ("ca", "Canada"),
        ("cl", "Chile"),
        ("cn", "China"),
        ("co", "Colombia"),
        ("hr", "Croatia"),
        ("cz", "Czechia"),
        ("dk", "Denmark"),
        ("eg", "Egypt"),
        ("fi", "Finland"),
        ("fr", "France"),
        ("de", "Germany"),
        ("gr", "Greece"),
        ("hu", "Hungary"),
        ("in", "India"),
        ("id", "Indonesia"),
        ("ie", "Ireland"),
        ("il", "Israel"),
        ("it", "Italy"),
        ("jp", "Japan"),
        ("my", "Malaysia"),
        ("mx", "Mexico"),
        ("nl", "Netherlands"),
        ("nz", "New Zealand"),
        ("no", "Norway"),
        ("pk", "Pakistan"),
        ("pe", "Peru"),
        ("ph", "Philippines"),
        ("pl", "Poland"),
        ("pt", "Portugal"),
        ("ro", "Romania"),
        ("ru", "Russia"),
        ("sa", "Saudi Arabia"),
        ("za", "South Africa"),
        ("es", "Spain"),
        ("se", "Sweden"),
        ("ch", "Switzerland"),
        ("th", "Thailand"),
        ("tr", "Turkey"),
        ("ua", "Ukraine"),
        ("gb", "United Kingdom"),
        ("us", "United States"),
        ("ve", "Venezuela"),
        ("vn", "Vietnam"),
    ];

    let q = query.to_lowercase();
    let skip = (page * per_page) as usize;

    // Fetch one extra to know whether another page exists.
    let mut batch: Vec<_> = COUNTRIES
        .iter()
        .filter(|(_, name)| q.is_empty() || name.to_lowercase().contains(&q))
        .skip(skip)
        .take(per_page as usize + 1)
        .collect();

    let has_more = batch.len() > per_page as usize;
    batch.truncate(per_page as usize);

    Ok((
        batch
            .into_iter()
            .map(|(code, name)| (code.to_string(), name.to_string()))
            .collect(),
        has_more,
    ))
}

// ── AsyncItems ───────────────────────────────────────────────────────────────
//
// `on_loading_change` fires with `true` when a fetch starts and `false` when it
// settles.  The parent decides where/how to render a loading indicator.

#[cfg(not(feature = "csr"))]
#[component]
fn AsyncItems(#[prop(optional)] on_loading_change: Option<Callback<bool>>) -> impl IntoView {
    use biji_ui::components::combobox;
    use leptos::html;
    use leptos_use::{use_debounce_fn_with_arg, use_intersection_observer};

    let ctx = expect_context::<combobox::ComboboxState>();

    // (query, page) drives the resource — query changes reset page to 0.
    let query_and_page = RwSignal::new((String::new(), 0u32));
    let has_more = RwSignal::new(false);
    // Set to true exactly when a page-0 fetch returns an empty result.
    // Reset to false as soon as a new fetch begins.
    let show_empty = RwSignal::new(false);
    // True while a fetch is in flight. Starts true (resource fires on mount).
    let is_loading = RwSignal::new(true);

    let update_query = use_debounce_fn_with_arg(
        move |q: String| {
            // Skip if the query hasn't actually changed (avoids the initial
            // debounce at t=300ms spuriously resetting state when query is
            // still "" and the resource has already loaded).
            if q == query_and_page.get_untracked().0 {
                return;
            }
            query_and_page.set((q, 0));
            has_more.set(false);
            show_empty.set(false);
            is_loading.set(true);
            if let Some(cb) = on_loading_change {
                cb.run(true);
            }
        },
        300.0,
    );
    Effect::new(move |_| {
        update_query(ctx.query.get());
    });

    let results = Resource::new(
        move || query_and_page.get(),
        |(q, p)| search_countries(q, p, 8),
    );

    // Signal the parent we're loading as soon as this component mounts —
    // the resource fires with ("", 0) immediately, before any debounce.
    if let Some(cb) = on_loading_change {
        cb.run(true);
    }

    // Stable item list: page=0 replaces, page>0 appends (no flicker).
    let items = RwSignal::<Vec<(String, String)>>::new(vec![]);
    let fetch_error = RwSignal::<Option<String>>::new(None);
    let is_fetching = RwSignal::new(false);

    Effect::new(move |_| match results.get() {
        None => {
            // New fetch in flight — hide any stale empty state immediately.
            show_empty.set(false);
        }
        Some(Ok((batch, more))) => {
            let page = query_and_page.get_untracked().1;
            if page == 0 {
                let is_empty = batch.is_empty();
                items.set(batch);
                show_empty.set(is_empty);
            } else {
                items.update(|v| v.extend(batch));
            }
            has_more.set(more);
            fetch_error.set(None);
            is_fetching.set(false);
            is_loading.set(false);
            if let Some(cb) = on_loading_change {
                cb.run(false);
            }
        }
        Some(Err(e)) => {
            if query_and_page.get_untracked().1 == 0 {
                items.set(vec![]);
            }
            fetch_error.set(Some(e.to_string()));
            has_more.set(false);
            show_empty.set(false);
            is_fetching.set(false);
            is_loading.set(false);
            if let Some(cb) = on_loading_change {
                cb.run(false);
            }
        }
    });

    // Guarded load_more — no-op if already in flight or no more pages.
    let load_more = move || {
        if is_fetching.get_untracked() || !has_more.get_untracked() {
            return;
        }
        is_fetching.set(true);
        query_and_page.update(|(_, p)| *p += 1);
        if let Some(cb) = on_loading_change {
            cb.run(true);
        }
    };

    // Sentinel div at the bottom of the list. When it scrolls into the viewport
    // the IntersectionObserver fires load_more. It's always in the DOM (the
    // `hidden` class hides it) so the observer can detect the hidden→visible
    // transition without needing to be re-created.
    let sentinel_ref = NodeRef::<html::Div>::new();
    use_intersection_observer(sentinel_ref, move |entries, _| {
        if entries
            .first()
            .map(|e| e.is_intersecting())
            .unwrap_or(false)
        {
            load_more();
        }
    });

    const ITEM_CLS: &str = "flex items-center px-3 py-1.5 cursor-default select-none outline-none \
        hover:bg-accent hover:text-accent-foreground \
        data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground \
        data-[disabled]:pointer-events-none data-[disabled]:opacity-50";

    view! {
        <div class="overflow-y-auto py-1 max-h-60">
            <Show when={move || is_loading.get() && items.with(|v| v.is_empty())}>
                <div class="flex justify-center py-4">
                    <div class="w-4 h-4 rounded-full border-2 animate-spin border-muted-foreground border-t-transparent" />
                </div>
            </Show>
            <Show when={move || fetch_error.with(|e| e.is_some())}>
                <div class="flex flex-col gap-1 items-center py-4 px-3 text-sm">
                    <span class="font-medium text-destructive">"Server error"</span>
                    <span class="text-xs text-muted-foreground">
                        {move || fetch_error.with(|e| e.clone().unwrap_or_default())}
                    </span>
                </div>
            </Show>
            <Show when={move || show_empty.get()}>
                <div class="py-4 px-3 text-sm text-center text-muted-foreground">
                    "No countries found."
                </div>
            </Show>
            <For
                each={move || items.get()}
                key={|(value, _)| value.clone()}
                children={move |(value, label)| {
                    view! {
                        <combobox::Item value={value} label={label.clone()} class={ITEM_CLS}>
                            <span class="flex-1">{label}</span>
                            <combobox::ItemIndicator>
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    width="14"
                                    height="14"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2.5"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                >
                                    <path d="M20 6 9 17l-5-5" />
                                </svg>
                            </combobox::ItemIndicator>
                        </combobox::Item>
                    }
                }}
            />
            // Sentinel: always in DOM, hidden when no more pages exist.
            // The IntersectionObserver detects the hidden→visible transition
            // and fires load_more when the user scrolls near the bottom.
            <div
                node_ref={sentinel_ref}
                class={move || {
                    if has_more.get() {
                        "py-2 flex justify-center text-muted-foreground"
                    } else {
                        "hidden"
                    }
                }}
            >
                <div class="w-4 h-4 rounded-full border-2 border-current animate-spin border-t-transparent" />
            </div>
        </div>
    }
}

// ── ComboboxAsyncExample ─────────────────────────────────────────────────────

#[cfg(not(feature = "csr"))]
#[component]
pub fn ComboboxAsyncExample() -> impl IntoView {
    use biji_ui::components::combobox;

    const INPUT_TRIGGER_CLS: &str = "w-56 px-3 py-2 pr-8 text-sm rounded-md border border-border \
        bg-background text-foreground outline-none \
        focus:ring-2 focus:ring-ring focus:ring-offset-0 \
        placeholder:text-muted-foreground";

    const CONTENT_CLS: &str = "z-50 w-56 overflow-hidden rounded-md border border-border \
        bg-background shadow-md text-sm \
        transition origin-[var(--biji-transform-origin)]";

    // Owned by the parent — place the spinner anywhere you like.
    let is_loading = RwSignal::new(false);

    view! {
        <div class="flex flex-col gap-3 items-center">
            <combobox::RootWith inline=true let:c>
                // Wrap the trigger so we can overlay the spinner on the right edge.
                <div class="relative">
                    <combobox::InputTrigger
                        class={INPUT_TRIGGER_CLS}
                        placeholder="Search countries\u{2026}"
                    />
                    <Show when={move || is_loading.get()}>
                        <div class="absolute right-2 top-1/2 -translate-y-1/2 pointer-events-none">
                            <div class="w-4 h-4 rounded-full border-2 animate-spin border-muted-foreground border-t-transparent" />
                        </div>
                    </Show>
                </div>
                <combobox::Content
                    class={CONTENT_CLS}
                    show_class="opacity-100 scale-100"
                    hide_class="opacity-0 scale-95"
                >
                    <AsyncItems on_loading_change={Callback::new(move |v| is_loading.set(v))} />
                </combobox::Content>
                <p class="text-xs text-muted-foreground">
                    "Selected: "
                    <span class="font-medium text-foreground">
                        {move || c.value.get().unwrap_or_else(|| "None".to_string())}
                    </span>
                </p>
            </combobox::RootWith>
        </div>
    }
}

// ── Static examples (existing) ────────────────────────────────────────────────

#[component]
pub fn ComboboxInlineExample() -> impl IntoView {
    use biji_ui::components::combobox;

    const INPUT_TRIGGER_CLS: &str = "w-56 px-3 py-2 text-sm rounded-md border border-border \
        bg-background text-foreground outline-none \
        focus:ring-2 focus:ring-ring focus:ring-offset-0 \
        placeholder:text-muted-foreground";

    const CONTENT_CLS: &str = "z-50 w-56 overflow-hidden rounded-md border border-border \
        bg-background shadow-md text-sm \
        transition origin-[var(--biji-transform-origin)]";

    const ITEM_CLS: &str = "flex items-center px-3 py-1.5 cursor-default select-none outline-none \
        hover:bg-accent hover:text-accent-foreground \
        data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground \
        data-[disabled]:pointer-events-none data-[disabled]:opacity-50";

    let fruits = [
        ("apple", "Apple"),
        ("apricot", "Apricot"),
        ("avocado", "Avocado"),
        ("banana", "Banana"),
        ("blueberry", "Blueberry"),
        ("cherry", "Cherry"),
        ("grape", "Grape"),
        ("kiwi", "Kiwi"),
        ("lemon", "Lemon"),
        ("mango", "Mango"),
        ("orange", "Orange"),
        ("peach", "Peach"),
        ("pear", "Pear"),
        ("pineapple", "Pineapple"),
        ("strawberry", "Strawberry"),
    ];

    view! {
        <div class="flex flex-col gap-3 items-center">
            <combobox::RootWith inline=true let:c>
                <combobox::InputTrigger class={INPUT_TRIGGER_CLS} placeholder="Search a fruit..." />
                <combobox::Content
                    class={CONTENT_CLS}
                    show_class="opacity-100 scale-100"
                    hide_class="opacity-0 scale-95"
                >
                    <div class="overflow-y-auto py-1 max-h-60">
                        <combobox::Empty>
                            <div class="py-6 px-3 text-sm text-center text-muted-foreground">
                                "No fruit found."
                            </div>
                        </combobox::Empty>
                        {fruits
                            .into_iter()
                            .map(|(value, label)| {
                                view! {
                                    <combobox::Item value={value} label={label} class={ITEM_CLS}>
                                        <span class="flex-1">{label}</span>
                                        <combobox::ItemIndicator>
                                            <svg
                                                xmlns="http://www.w3.org/2000/svg"
                                                width="14"
                                                height="14"
                                                viewBox="0 0 24 24"
                                                fill="none"
                                                stroke="currentColor"
                                                stroke-width="2.5"
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                            >
                                                <path d="M20 6 9 17l-5-5" />
                                            </svg>
                                        </combobox::ItemIndicator>
                                    </combobox::Item>
                                }
                            })
                            .collect_view()}
                    </div>
                </combobox::Content>
                <p class="text-xs text-muted-foreground">
                    "Selected: "
                    <span class="font-medium text-foreground">
                        {move || c.value.get().unwrap_or_else(|| "None".to_string())}
                    </span>
                </p>
            </combobox::RootWith>
        </div>
    }
}
