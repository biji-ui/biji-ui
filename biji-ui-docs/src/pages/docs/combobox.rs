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
                            " function. The example splits responsibilities into two components: "
                            <code class="font-mono text-foreground">{"AsyncQueryManager"}</code>
                            " owns all reactive state and is mounted "
                            <em>"outside"</em>
                            " "
                            <code class="font-mono text-foreground">{"Content"}</code>
                            ", and "
                            <code class="font-mono text-foreground">{"ItemDisplay"}</code>
                            " is a pure render component placed inside "
                            <code class="font-mono text-foreground">{"Content"}</code>
                            "."
                        </p>
                        <p class="mb-3 text-sm text-muted-foreground">
                            "The query is debounced 300 ms before triggering a new call. An "
                            <code class="font-mono text-foreground">{"on_loading_change"}</code>
                            " callback fires "
                            <code class="font-mono text-foreground">{"true"}</code>
                            " when a fetch starts and "
                            <code class="font-mono text-foreground">{"false"}</code>
                            " when it settles, so the parent can render a spinner anywhere."
                        </p>
                        <p class="mb-3 text-sm text-muted-foreground">
                            "Infinite scroll is handled by an "
                            <code class="font-mono text-foreground">{"IntersectionObserver"}</code>
                            " on a sentinel element at the bottom of the list. When the sentinel becomes visible the next page is fetched and appended without re-rendering existing items."
                        </p>
                        <div class="mb-4 rounded-md border border-amber-300 bg-amber-50 dark:border-amber-700 dark:bg-amber-950/30 px-4 py-3 text-sm">
                            <p class="font-semibold text-amber-800 dark:text-amber-300 mb-1">"⚠ SSR + Hydrate: avoid Resource::new and LocalResource inside combobox"</p>
                            <p class="text-amber-700 dark:text-amber-400 mb-2">
                                <code class="font-mono bg-amber-100 dark:bg-amber-900/40 px-1 rounded">{"Resource::new"}</code>
                                " and "
                                <code class="font-mono bg-amber-100 dark:bg-amber-900/40 px-1 rounded">{"LocalResource"}</code>
                                " register with the nearest "
                                <code class="font-mono bg-amber-100 dark:bg-amber-900/40 px-1 rounded">{"<Suspense>"}</code>
                                " / "
                                <code class="font-mono bg-amber-100 dark:bg-amber-900/40 px-1 rounded">{"<Transition>"}</code>
                                " boundary in the tree. When the resource becomes pending (e.g. on a new search keystroke), the boundary shows its fallback — causing a full-page blank flash if you have an "
                                <code class="font-mono bg-amber-100 dark:bg-amber-900/40 px-1 rounded">{"AuthGuard"}</code>
                                " or similar layout wrapper using "
                                <code class="font-mono bg-amber-100 dark:bg-amber-900/40 px-1 rounded">{"<Transition>"}</code>
                                "."
                            </p>
                            <p class="text-amber-700 dark:text-amber-400">
                                "Use "
                                <code class="font-mono bg-amber-100 dark:bg-amber-900/40 px-1 rounded">{"spawn_local"}</code>
                                " with a generation counter instead. It is a detached async task with no knowledge of the reactive resource system and therefore never triggers any Suspense boundary. See the code sample below."
                            </p>
                        </div>
                        <div class="mb-4 rounded-md border border-blue-200 bg-blue-50 dark:border-blue-800 dark:bg-blue-950/30 px-4 py-3 text-sm">
                            <p class="font-semibold text-blue-800 dark:text-blue-300 mb-1">"💡 Lift async state outside Content"</p>
                            <p class="text-blue-700 dark:text-blue-400">
                                <code class="font-mono bg-blue-100 dark:bg-blue-900/40 px-1 rounded">{"combobox::Content"}</code>
                                " unmounts its children after the hide animation completes. Placing Effects or Resources inside "
                                <code class="font-mono bg-blue-100 dark:bg-blue-900/40 px-1 rounded">{"Content"}</code>
                                " means recreating them on every open cycle, which causes a WASM block and a blank frame. Mount an "
                                <code class="font-mono bg-blue-100 dark:bg-blue-900/40 px-1 rounded">{"AsyncQueryManager"}</code>
                                " component alongside — not inside — "
                                <code class="font-mono bg-blue-100 dark:bg-blue-900/40 px-1 rounded">{"Content"}</code>
                                " so signals and Effects are created exactly once."
                            </p>
                        </div>
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
    let mut fetched: Vec<_> = countries
        .iter()
        .filter(|(_, n)| q.is_empty() || n.to_lowercase().contains(&q))
        .skip(skip)
        .take(per_page as usize + 1)   // one extra → has_more signal
        .collect();
    let has_more = fetched.len() > per_page as usize;
    fetched.truncate(per_page as usize);
    Ok((fetched.into_iter().map(|(v, l)| (v.to_string(), l.to_string())).collect(), has_more))
}

// ── AsyncQueryManager ────────────────────────────────────────────────────────
//
// Manages debounced search and async fetching. Rendered OUTSIDE
// combobox::Content so it is never unmounted when the dropdown closes.
//
// combobox::Content uses an animated show/hide: it unmounts its children
// after the hide animation completes. Putting Effects or Resources inside
// Content means recreating them on every open cycle — WASM block → blank frame.
// By keeping this component outside Content, signals and Effects are created
// exactly once for the lifetime of the parent.
//
// spawn_local instead of Resource::new
// ─────────────────────────────────────
// Resource::new (and LocalResource) register with the nearest Suspense /
// Transition boundary in the tree. When the resource becomes pending (e.g.
// on a new search), a parent <Transition> sees a pending resource and briefly
// shows its fallback — a blank screen flash. This is especially noticeable
// in apps that wrap routes with an AuthGuard <Transition>.
// spawn_local is a detached async task: it has no knowledge of the reactive
// resource system and therefore never triggers any Suspense boundary.

#[component]
fn AsyncQueryManager(
    query_and_page: RwSignal<(String, u32)>,
    items: RwSignal<Vec<(String, String)>>,
    has_more: RwSignal<bool>,
    show_empty: RwSignal<bool>,
    is_loading: RwSignal<bool>,
    is_fetching: RwSignal<bool>,
    on_loading_change: Callback<bool>,
) -> impl IntoView {
    use biji_ui::components::combobox::ComboboxState;
    use leptos::reactive::spawn_local;
    use leptos_use::use_debounce_fn_with_arg;

    let ctx = expect_context::<ComboboxState>();

    let set_query = use_debounce_fn_with_arg(
        move |q: String| {
            // Skip if the query hasn't changed (avoids spurious reset at t=300ms).
            if q == query_and_page.get_untracked().0 { return; }
            batch(|| {
                query_and_page.set((q, 0));
                has_more.set(false);
                show_empty.set(false);
                is_loading.set(true);
                on_loading_change.run(true);
            });
        },
        300.0,
    );
    Effect::new(move |_| { set_query(ctx.query.get()); });

    // Generation counter: when a new fetch starts, any in-flight response
    // with an older sequence number is discarded. Prevents stale results
    // overwriting newer ones when the query changes quickly.
    let fetch_gen = RwSignal::new(0u64);

    // Signal loading on mount — the first fetch fires immediately.
    on_loading_change.run(true);

    Effect::new(move |_| {
        let (q, page) = query_and_page.get();
        let fetch_seq = fetch_gen.get_untracked() + 1;
        fetch_gen.set(fetch_seq);

        spawn_local(async move {
            let result = search_countries(q, page, 8).await;

            // Discard if a newer request has already started.
            if fetch_gen.get_untracked() != fetch_seq { return; }

            match result {
                Ok((page_items, more)) => {
                    let is_empty = page_items.is_empty();
                    batch(|| {
                        if page == 0 {
                            items.set(page_items);
                            show_empty.set(is_empty);
                        } else {
                            items.update(|v| v.extend(page_items));
                        }
                        has_more.set(more);
                        is_fetching.set(false);
                        is_loading.set(false);
                        on_loading_change.run(false);
                    });
                }
                Err(_) => {
                    batch(|| {
                        if page == 0 { items.set(vec![]); }
                        has_more.set(false);
                        show_empty.set(false);
                        is_fetching.set(false);
                        is_loading.set(false);
                        on_loading_change.run(false);
                    });
                }
            }
        });
    });
}

// ── ItemDisplay ───────────────────────────────────────────────────────────────
//
// Pure rendering — no Effects, no Resources, no Signals created here.
// Lives inside combobox::Content so it remounts on every open, but
// because it only creates DOM nodes + event listeners the cost is negligible.

#[component]
fn ItemDisplay(
    items: RwSignal<Vec<(String, String)>>,
    is_loading: RwSignal<bool>,
    show_empty: RwSignal<bool>,
    has_more: RwSignal<bool>,
    load_more: Callback<()>,
) -> impl IntoView {
    use leptos::html;
    use leptos_use::use_intersection_observer;

    let sentinel_ref = NodeRef::<html::Div>::new();
    use_intersection_observer(sentinel_ref, move |entries, _| {
        if entries.first().map(|e| e.is_intersecting()).unwrap_or(false) {
            load_more.run(());
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
                                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14"
                                    viewBox="0 0 24 24" fill="none" stroke="currentColor"
                                    stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M20 6 9 17l-5-5" />
                                </svg>
                            </combobox::ItemIndicator>
                        </combobox::Item>
                    }
                }}
            />
            <div
                node_ref={sentinel_ref}
                class={move || if has_more.get() { "py-2 flex justify-center text-muted-foreground" } else { "hidden" }}
            >
                <div class="w-4 h-4 rounded-full border-2 border-current animate-spin border-t-transparent" />
            </div>
        </div>
    }
}

// ── MyAsyncCombobox ───────────────────────────────────────────────────────────
//
// All async state lives here — outside combobox::Content — so signals and
// Effects are created once and persist across open/close cycles.

#[component]
pub fn MyAsyncCombobox() -> impl IntoView {
    let query_and_page = RwSignal::<(String, u32)>::new((String::new(), 0));
    let items = RwSignal::<Vec<(String, String)>>::new(vec![]);
    let has_more = RwSignal::new(false);
    let show_empty = RwSignal::new(false);
    let is_loading = RwSignal::new(true);
    let is_fetching = RwSignal::new(false);
    let trigger_is_loading = RwSignal::new(false);

    let load_more = Callback::new(move |_: ()| {
        if is_fetching.get_untracked() || !has_more.get_untracked() { return; }
        is_fetching.set(true);
        query_and_page.update(|(_, p)| *p += 1);
    });

    view! {
        <combobox::RootWith inline=true let:_c>
            // Always-mounted query manager — outside Content, never torn down.
            <AsyncQueryManager
                query_and_page=query_and_page
                items=items
                has_more=has_more
                show_empty=show_empty
                is_loading=is_loading
                is_fetching=is_fetching
                on_loading_change=Callback::new(move |v| trigger_is_loading.set(v))
            />
            <div class="relative">
                <combobox::InputTrigger class="…" placeholder="Search countries…" />
                <Show when=move || trigger_is_loading.get()>
                    <div class="absolute right-2 top-1/2 -translate-y-1/2 pointer-events-none">
                        <div class="w-3.5 h-3.5 rounded-full border-2 animate-spin border-muted-foreground border-t-transparent" />
                    </div>
                </Show>
            </div>
            <combobox::Content class="…"
                show_class="opacity-100 scale-100"
                hide_class="opacity-0 scale-95"
            >
                // Cheap display — no reactive setup on mount, just DOM nodes.
                <ItemDisplay
                    items=items
                    is_loading=is_loading
                    show_empty=show_empty
                    has_more=has_more
                    load_more=load_more
                />
            </combobox::Content>
        </combobox::RootWith>
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
fn AsyncQueryManager(
    query_and_page: RwSignal<(String, u32)>,
    items: RwSignal<Vec<(String, String)>>,
    has_more: RwSignal<bool>,
    show_empty: RwSignal<bool>,
    is_loading: RwSignal<bool>,
    is_fetching: RwSignal<bool>,
    on_loading_change: Callback<bool>,
) -> impl IntoView {
    use biji_ui::components::combobox;
    use leptos::reactive::spawn_local;
    use leptos_use::use_debounce_fn_with_arg;

    let ctx = expect_context::<combobox::ComboboxState>();

    let set_query = use_debounce_fn_with_arg(
        move |q: String| {
            if q == query_and_page.get_untracked().0 {
                return;
            }
            batch(|| {
                query_and_page.set((q, 0));
                has_more.set(false);
                show_empty.set(false);
                is_loading.set(true);
                on_loading_change.run(true);
            });
        },
        300.0,
    );
    Effect::new(move |_| {
        set_query(ctx.query.get());
    });

    let fetch_gen = RwSignal::new(0u64);

    on_loading_change.run(true);

    Effect::new(move |_| {
        let (q, page) = query_and_page.get();
        let fetch_seq = fetch_gen.get_untracked() + 1;
        fetch_gen.set(fetch_seq);

        spawn_local(async move {
            let result = search_countries(q, page, 8).await;

            if fetch_gen.get_untracked() != fetch_seq {
                return;
            }

            match result {
                Ok((page_items, more)) => {
                    let is_empty = page_items.is_empty();
                    batch(|| {
                        if page == 0 {
                            items.set(page_items);
                            show_empty.set(is_empty);
                        } else {
                            items.update(|v| v.extend(page_items));
                        }
                        has_more.set(more);
                        is_fetching.set(false);
                        is_loading.set(false);
                        on_loading_change.run(false);
                    });
                }
                Err(_) => {
                    batch(|| {
                        if page == 0 {
                            items.set(vec![]);
                        }
                        has_more.set(false);
                        show_empty.set(false);
                        is_fetching.set(false);
                        is_loading.set(false);
                        on_loading_change.run(false);
                    });
                }
            }
        });
    });
}

#[cfg(not(feature = "csr"))]
#[component]
fn ItemDisplay(
    items: RwSignal<Vec<(String, String)>>,
    is_loading: RwSignal<bool>,
    show_empty: RwSignal<bool>,
    has_more: RwSignal<bool>,
    load_more: Callback<()>,
) -> impl IntoView {
    use biji_ui::components::combobox;
    use leptos::html;
    use leptos_use::use_intersection_observer;

    let sentinel_ref = NodeRef::<html::Div>::new();
    use_intersection_observer(sentinel_ref, move |entries, _| {
        if entries
            .first()
            .map(|e| e.is_intersecting())
            .unwrap_or(false)
        {
            load_more.run(());
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

    let query_and_page = RwSignal::<(String, u32)>::new((String::new(), 0));
    let items = RwSignal::<Vec<(String, String)>>::new(vec![]);
    let has_more = RwSignal::new(false);
    let show_empty = RwSignal::new(false);
    let is_loading = RwSignal::new(true);
    let is_fetching = RwSignal::new(false);
    let trigger_is_loading = RwSignal::new(false);

    let load_more = Callback::new(move |_: ()| {
        if is_fetching.get_untracked() || !has_more.get_untracked() {
            return;
        }
        is_fetching.set(true);
        query_and_page.update(|(_, p)| *p += 1);
    });

    view! {
        <div class="flex flex-col gap-3 items-center">
            <combobox::RootWith inline=true let:c>
                // Always-mounted query manager — outside Content, never torn down.
                <AsyncQueryManager
                    query_and_page=query_and_page
                    items=items
                    has_more=has_more
                    show_empty=show_empty
                    is_loading=is_loading
                    is_fetching=is_fetching
                    on_loading_change=Callback::new(move |v| trigger_is_loading.set(v))
                />
                <div class="relative">
                    <combobox::InputTrigger
                        class={INPUT_TRIGGER_CLS}
                        placeholder="Search countries\u{2026}"
                    />
                    <Show when={move || trigger_is_loading.get()}>
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
                    // Cheap display — no reactive setup on mount, just DOM nodes.
                    <ItemDisplay
                        items=items
                        is_loading=is_loading
                        show_empty=show_empty
                        has_more=has_more
                        load_more=load_more
                    />
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
