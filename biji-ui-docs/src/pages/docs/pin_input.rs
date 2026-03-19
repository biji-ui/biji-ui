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
    "\", features = [\"pin_input\"] }",
);

const USAGE_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::pin_input;

#[component]
pub fn MyPinInput() -> impl IntoView {
    view! {
        <pin_input::Root
            length=6
            class="flex gap-2"
            on_complete={Callback::new(|code: String| {
                leptos::logging::log!("PIN complete: {code}");
            })}
        >
            <pin_input::Cell index=0 class="w-10 h-12 text-center text-lg rounded-md border border-input bg-background focus:outline-none focus:ring-2 focus:ring-ring data-[filled]:border-primary" />
            <pin_input::Cell index=1 class="w-10 h-12 text-center text-lg rounded-md border border-input bg-background focus:outline-none focus:ring-2 focus:ring-ring data-[filled]:border-primary" />
            <pin_input::Cell index=2 class="w-10 h-12 text-center text-lg rounded-md border border-input bg-background focus:outline-none focus:ring-2 focus:ring-ring data-[filled]:border-primary" />
            <pin_input::Cell index=3 class="w-10 h-12 text-center text-lg rounded-md border border-input bg-background focus:outline-none focus:ring-2 focus:ring-ring data-[filled]:border-primary" />
            <pin_input::Cell index=4 class="w-10 h-12 text-center text-lg rounded-md border border-input bg-background focus:outline-none focus:ring-2 focus:ring-ring data-[filled]:border-primary" />
            <pin_input::Cell index=5 class="w-10 h-12 text-center text-lg rounded-md border border-input bg-background focus:outline-none focus:ring-2 focus:ring-ring data-[filled]:border-primary" />
        </pin_input::Root>
    }
}"#;

const ROOT_WITH_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::pin_input;

#[component]
pub fn MyPinInput() -> impl IntoView {
    const CELL_CLS: &str = "w-11 h-13 text-center text-xl font-mono rounded-md border \
        border-input bg-background focus:outline-none focus:ring-2 focus:ring-ring \
        data-[filled]:border-primary placeholder:text-muted-foreground/40";

    view! {
        <pin_input::RootWith length=4 class="flex flex-col items-center gap-4" let:s>
            <div class="flex gap-2">
                <pin_input::Cell index=0 class={CELL_CLS} />
                <pin_input::Cell index=1 class={CELL_CLS} />
                <pin_input::Cell index=2 class={CELL_CLS} />
                <pin_input::Cell index=3 class={CELL_CLS} />
            </div>
            <p class="text-xs text-muted-foreground">
                {move || if s.is_complete.get() {
                    format!("Code: {}", s.value.get())
                } else {
                    format!("{}/4 digits entered", s.value.get().len())
                }}
            </p>
        </pin_input::RootWith>
    }
}"#;

const ROOT_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the root wrapper element.",
    },
    PropRow {
        name: "length",
        prop_type: "usize",
        default: "4",
        description: "The total number of PIN cells.",
    },
    PropRow {
        name: "disabled",
        prop_type: "bool",
        default: "false",
        description: "When true, all cells are disabled.",
    },
    PropRow {
        name: "placeholder",
        prop_type: "String",
        default: "\"○\"",
        description: "Placeholder character shown in each empty cell.",
    },
    PropRow {
        name: "on_complete",
        prop_type: "Option<Callback<String>>",
        default: "None",
        description: "Callback fired with the full PIN string when every cell is filled.",
    },
];

const CELL_PROPS: &[PropRow] = &[
    PropRow {
        name: "index",
        prop_type: "usize",
        default: "",
        description: "Zero-based position of this cell within the PIN input.",
    },
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the input element.",
    },
];

const DATA_ATTRS: &[DataAttrRow] = &[
    DataAttrRow {
        name: "data-filled",
        description: "Present on Cell when the cell has a value entered.",
    },
    DataAttrRow {
        name: "data-disabled",
        description: "Present on Root and Cell when the input is disabled.",
    },
    DataAttrRow {
        name: "data-index",
        description: "The zero-based index of the cell. Present on Cell.",
    },
];

const KEYBOARD: &[KeyboardRow] = &[
    KeyboardRow {
        key: "0–9 / Any character",
        description: "Types a character into the current cell and advances focus to the next cell.",
    },
    KeyboardRow {
        key: "Backspace",
        description:
            "Clears the current cell. If already empty, clears the previous cell and moves focus back.",
    },
    KeyboardRow {
        key: "ArrowLeft",
        description: "Moves focus to the previous cell.",
    },
    KeyboardRow {
        key: "ArrowRight",
        description: "Moves focus to the next cell.",
    },
    KeyboardRow {
        key: "Paste (Ctrl+V / Cmd+V)",
        description:
            "Pastes a string of characters distributed across cells starting at the focused cell.",
    },
];

#[component]
pub fn PinInputDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    view! {
        <DocPage title="PIN Input">
            <p class="mt-3 mb-11 text-base text-balance">
                "A sequence of single-character input cells for entering PINs, OTPs, and verification codes."
            </p>
            <DocPreview>
                <PinInputExample />
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
            <SectionHeading title="RootWith" />
            <p class="mb-4 text-sm text-muted-foreground">
                "Use "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"<RootWith>"</code>
                " when you need direct access to "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"PinInputState"</code>
                " inside the children. The "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"let:s"</code>
                " binding exposes "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"s.value"</code>
                " and "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"s.is_complete"</code>
                " as reactive signals — no callbacks needed for reading state."
            </p>
            <DocPreview>
                <PinInputRootWithExample />
            </DocPreview>
            <Code
                class="mt-4 [&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={ROOT_WITH_CODE}
                language="rust"
            />
            <SectionHeading title="API Reference" />
            <PropsTable title="Root / RootWith" rows={ROOT_PROPS} />
            <PropsTable title="Cell" rows={CELL_PROPS} />
            <DataAttrsTable rows={DATA_ATTRS} />
            <KeyboardTable rows={KEYBOARD} />
        </DocPage>
    }
}

#[component]
pub fn PinInputRootWithExample() -> impl IntoView {
    use biji_ui::components::pin_input;

    const CELL_CLS: &str = "w-11 h-13 text-center text-xl font-mono rounded-md border border-input \
        bg-background focus:outline-none focus:ring-2 focus:ring-ring \
        data-[filled]:border-primary data-[filled]:text-foreground \
        disabled:opacity-50 disabled:cursor-not-allowed \
        placeholder:text-muted-foreground/40";

    view! {
        <div class="flex flex-col items-center gap-6">
            <div class="flex flex-col items-center gap-2">
                <p class="text-sm font-medium text-foreground">"Enter 4-digit code"</p>
            </div>
            <pin_input::RootWith length=4 class="flex flex-col items-center gap-4" let:s>
                <div class="flex gap-2">
                    <pin_input::Cell index=0 class={CELL_CLS} />
                    <pin_input::Cell index=1 class={CELL_CLS} />
                    <pin_input::Cell index=2 class={CELL_CLS} />
                    <pin_input::Cell index=3 class={CELL_CLS} />
                </div>
                <p class="text-xs text-muted-foreground h-4">
                    {move || if s.is_complete.get() {
                        format!("Code: {}", s.value.get())
                    } else if s.value.get().is_empty() {
                        String::new()
                    } else {
                        format!("{}/4 digits entered", s.value.get().len())
                    }}
                </p>
            </pin_input::RootWith>
        </div>
    }
}

#[component]
pub fn PinInputExample() -> impl IntoView {
    use biji_ui::components::pin_input;

    const CELL_CLS: &str = "w-11 h-13 text-center text-xl font-mono rounded-md border border-input \
        bg-background focus:outline-none focus:ring-2 focus:ring-ring \
        data-[filled]:border-primary data-[filled]:text-foreground \
        disabled:opacity-50 disabled:cursor-not-allowed \
        placeholder:text-muted-foreground/40";

    view! {
        <div class="flex flex-col items-center gap-6">
            <div class="flex flex-col items-center gap-2">
                <p class="text-sm font-medium text-foreground">"Enter verification code"</p>
                <p class="text-xs text-muted-foreground">"We sent a 6-digit code to your email."</p>
            </div>
            <pin_input::RootWith
                length=6
                class="flex flex-col items-center gap-4"
                on_complete={Callback::new(|_: String| {})}
                let:s
            >
                <div class="flex gap-2">
                    <pin_input::Cell index=0 class={CELL_CLS} />
                    <pin_input::Cell index=1 class={CELL_CLS} />
                    <pin_input::Cell index=2 class={CELL_CLS} />
                    <pin_input::Cell index=3 class={CELL_CLS} />
                    <pin_input::Cell index=4 class={CELL_CLS} />
                    <pin_input::Cell index=5 class={CELL_CLS} />
                </div>
                <Show when={move || !s.value.get().is_empty() && !s.is_complete.get()}>
                    <p class="text-sm text-muted-foreground">
                        "Typing: "
                        <span class="font-mono font-medium text-foreground">
                            {move || s.value.get()}
                        </span>
                    </p>
                </Show>
                <Show when={move || s.is_complete.get()}>
                    <p class="text-sm text-muted-foreground">
                        "Code entered: "
                        <span class="font-mono font-medium text-foreground">
                            {move || s.value.get()}
                        </span>
                    </p>
                </Show>
            </pin_input::RootWith>
        </div>
    }
}
