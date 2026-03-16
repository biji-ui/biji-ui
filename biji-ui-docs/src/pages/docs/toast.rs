use std::time::Duration;

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
    "\", features = [\"toast\"] }",
);

const USAGE_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::toast::{Toaster, ToasterContext};

// 1. Wrap your app (or any subtree) with <Toaster>.
//    This provides ToasterContext to all descendants.
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Toaster>
            <MyApp />
        </Toaster>
    }
}

// 2. Anywhere inside, grab the context and call add() with your own type string.
#[component]
pub fn SaveButton() -> impl IntoView {
    let toaster = expect_context::<ToasterContext>();

    view! {
        <button on:click=move |_| {
            toaster.add("Changes saved successfully.", None, Some("success".to_string()), None);
        }>
            "Save"
        </button>
    }
}"#;

const STYLED_CODE: &str = r#"// Pass a toast_class to style individual toasts.
// Use data-[type=…], data-[entering=…], data-[dismissed=…], data-[paused=…] selectors.
// The type value is whatever string you pass to add() — define your own vocabulary.
<Toaster
    pause_on_hover=PauseOnHover::Single
    show_progress=true
    progress_class="absolute bottom-0 left-0 h-0.5 bg-current opacity-30 rounded-full"
    toast_class="relative flex items-start gap-3 w-full rounded-lg border px-4 py-3 \
        shadow-md text-sm font-medium bg-background border-border text-foreground \
        overflow-hidden transition-all duration-300 ease-in-out \
        data-[entering=true]:opacity-0 data-[entering=true]:translate-y-2 \
        data-[dismissed=true]:opacity-0 data-[dismissed=true]:translate-x-[calc(100%+1rem)] \
        data-[type=success]:border-green-500/30 data-[type=success]:bg-green-50 \
        data-[type=success]:text-green-900 dark:data-[type=success]:bg-green-950/30 \
        dark:data-[type=success]:text-green-100"
>
    <MyApp />
</Toaster>"#;

const TOASTER_PROPS: &[PropRow] = &[
    PropRow {
        name: "children",
        prop_type: "Children",
        default: "—",
        description: "The app subtree. ToasterContext is available to all descendants.",
    },
    PropRow {
        name: "default_duration",
        prop_type: "Option<Duration>",
        default: "4000 ms",
        description: "How long a toast stays visible before auto-dismissing.",
    },
    PropRow {
        name: "hide_delay",
        prop_type: "Option<Duration>",
        default: "300 ms",
        description: "Time between data-dismissed=\"true\" and the toast being removed from the DOM. Should match your CSS exit-animation duration.",
    },
    PropRow {
        name: "position",
        prop_type: "ToastPosition",
        default: "BottomRight",
        description: "Viewport corner where the toast stack appears. One of TopLeft, TopCenter, TopRight, BottomLeft, BottomCenter, BottomRight.",
    },
    PropRow {
        name: "max_toasts",
        prop_type: "usize",
        default: "5",
        description: "Maximum number of toasts shown at once.",
    },
    PropRow {
        name: "pause_on_hover",
        prop_type: "PauseOnHover",
        default: "Single",
        description: "Single — only the hovered toast pauses. All — hovering any toast pauses all. Disable — hover has no effect.",
    },
    PropRow {
        name: "show_progress",
        prop_type: "bool",
        default: "false",
        description: "When true, renders a countdown progress bar inside each toast. The bar's width transitions from 100% to 0% over the toast duration. Style it with progress_class.",
    },
    PropRow {
        name: "progress_class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class for the progress bar element. Control height, color, and position here — the inline style handles width.",
    },
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "Extra CSS class on the stack container div.",
    },
    PropRow {
        name: "toast_class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to every individual toast element. Style via data-[type=…], data-[entering=…], data-[dismissed=…], data-[paused=…] selectors.",
    },
];

const CONTEXT_METHODS: &[PropRow] = &[
    PropRow {
        name: "toast(title)",
        prop_type: "impl Into<String>",
        default: "—",
        description: "Add a toast with no type string.",
    },
    PropRow {
        name: "add(title, description, toast_type, duration)",
        prop_type: "String, Option<String>, Option<String>, Option<Duration>",
        default: "—",
        description: "Full control: title, optional description, optional type string (emitted as data-type), and optional custom duration.",
    },
    PropRow {
        name: "dismiss(id)",
        prop_type: "u32",
        default: "—",
        description: "Start the exit animation for a specific toast.",
    },
    PropRow {
        name: "dismiss_all()",
        prop_type: "—",
        default: "—",
        description: "Dismiss all active toasts.",
    },
];

const DATA_ATTRS: &[DataAttrRow] = &[
    DataAttrRow {
        name: "data-type",
        description: "The user-supplied type string passed to add(). Absent when no type was given. Use any value you want — e.g. \"success\", \"error\", or your own custom variants.",
    },
    DataAttrRow {
        name: "data-entering",
        description: "\"true\" for ~1 ms after mount so CSS transitions fire from the hidden state. Use data-[entering=true]:… to define your enter-from state.",
    },
    DataAttrRow {
        name: "data-dismissed",
        description: "\"true\" once the toast starts its exit animation. Use data-[dismissed=true]:… to define your exit state.",
    },
    DataAttrRow {
        name: "data-paused",
        description: "\"true\" while the toast's countdown is paused (e.g. on hover). Use data-[paused=true]:… to show a visual indicator.",
    },
];

const KEYBOARD: &[KeyboardRow] = &[KeyboardRow {
    key: "×",
    description: "Clicking the dismiss button triggers the exit animation and removes the toast.",
}];

#[component]
pub fn ToastDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    view! {
        <DocPage title="Toast">
            <p class="mt-3 mb-11 text-base text-balance">
                "Ephemeral notifications that slide in from the corner of the screen, auto-dismiss after a configurable duration, and pause on hover."
            </p>
            <DocPreview>
                <ToastExample />
            </DocPreview>
            <SectionHeading title="Installation" />
            <Code
                class="[&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={INSTALL_CODE}
                language="toml"
            />
            <SectionHeading title="Usage" />
            <p class="mb-4 text-sm text-muted-foreground">
                "Wrap your app (or any subtree) with "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"<Toaster>"</code>
                ". This provides "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"ToasterContext"</code>
                " to all descendants. Access it anywhere with "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">
                    "expect_context::<ToasterContext>()"
                </code> " and call "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"add()"</code>
                " with your own type string to show a toast."
            </p>
            <Code
                class="[&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={USAGE_CODE}
                language="rust"
            />
            <SectionHeading title="Styling" />
            <p class="mb-4 text-sm text-muted-foreground">
                "The component is fully headless — no styles are applied by default. Pass a "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"toast_class"</code>
                " string to "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"<Toaster>"</code>
                " and use "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">
                    "data-[type=…]"
                </code> ", "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">
                    "data-[entering=true]"
                </code> ", "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">
                    "data-[dismissed=true]"
                </code> ", and "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">
                    "data-[paused=true]"
                </code> " Tailwind arbitrary-variant selectors to control each state. "
                "Enable the progress bar with "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">
                    "show_progress=true"
                </code> " and style it with "
                <code class="py-0.5 px-1 font-mono text-xs rounded bg-muted">"progress_class"</code>
                "."
            </p>
            <Code
                class="[&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={STYLED_CODE}
                language="rust"
            />
            <SectionHeading title="API Reference" />
            <PropsTable title="Toaster" rows={TOASTER_PROPS} />
            <PropsTable title="ToasterContext methods" rows={CONTEXT_METHODS} />
            <DataAttrsTable rows={DATA_ATTRS} />
            <KeyboardTable rows={KEYBOARD} />
        </DocPage>
    }
}

const TOAST_CLS: &str = "relative flex items-start gap-3 w-full rounded-lg border px-4 py-3 \
    shadow-md text-sm font-medium bg-background border-border text-foreground \
    overflow-hidden transition-all duration-300 ease-in-out \
    data-[entering=true]:opacity-0 data-[entering=true]:translate-y-2 \
    data-[dismissed=true]:opacity-0 data-[dismissed=true]:translate-x-[calc(100%+1rem)] \
    data-[type=success]:border-green-500/30 data-[type=success]:bg-green-50 \
    data-[type=success]:text-green-900 dark:data-[type=success]:bg-green-950/30 \
    dark:data-[type=success]:text-green-100 \
    data-[type=error]:border-red-500/30 data-[type=error]:bg-red-50 \
    data-[type=error]:text-red-900 dark:data-[type=error]:bg-red-950/30 \
    dark:data-[type=error]:text-red-100 \
    data-[type=warning]:border-yellow-500/30 data-[type=warning]:bg-yellow-50 \
    data-[type=warning]:text-yellow-900 dark:data-[type=warning]:bg-yellow-950/30 \
    dark:data-[type=warning]:text-yellow-100 \
    data-[type=info]:border-blue-500/30 data-[type=info]:bg-blue-50 \
    data-[type=info]:text-blue-900 dark:data-[type=info]:bg-blue-950/30 \
    dark:data-[type=info]:text-blue-100";

const PROGRESS_CLS: &str = "absolute bottom-0 left-0 h-0.5 rounded-full \
    bg-current opacity-30 \
    data-[type=success]:bg-green-500 data-[type=error]:bg-red-500 \
    data-[type=warning]:bg-yellow-500 data-[type=info]:bg-blue-500";

const BTN_CLS: &str = "py-1.5 px-3 text-sm rounded-md border transition-colors \
    border-border bg-background hover:bg-muted text-foreground";

#[component]
pub fn ToastExample() -> impl IntoView {
    use biji_ui::components::toast::{PauseOnHover, ToastPosition, Toaster};

    view! {
        <Toaster
            position={ToastPosition::BottomRight}
            hide_delay={Duration::from_millis(300)}
            pause_on_hover={PauseOnHover::Single}
            show_progress=true
            progress_class={PROGRESS_CLS}
            toast_class={TOAST_CLS}
        >
            <ToastDemoButtons />
        </Toaster>
    }
}

#[component]
fn ToastDemoButtons() -> impl IntoView {
    use biji_ui::components::toast::ToasterContext;

    let toaster = expect_context::<ToasterContext>();

    view! {
        <div class="flex flex-wrap gap-2 justify-center">
            <button class={BTN_CLS} on:click={move |_| toaster.toast("Event has been created")}>
                "Default"
            </button>
            <button
                class={BTN_CLS}
                on:click={move |_| {
                    toaster
                        .add("Changes saved successfully", None, Some("success".to_string()), None)
                }}
            >
                "Success"
            </button>
            <button
                class={BTN_CLS}
                on:click={move |_| {
                    toaster.add("Something went wrong", None, Some("error".to_string()), None)
                }}
            >
                "Error"
            </button>
            <button
                class={BTN_CLS}
                on:click={move |_| {
                    toaster
                        .add(
                            "Your session will expire soon",
                            None,
                            Some("warning".to_string()),
                            None,
                        )
                }}
            >
                "Warning"
            </button>
            <button
                class={BTN_CLS}
                on:click={move |_| {
                    toaster.add("New version available", None, Some("info".to_string()), None)
                }}
            >
                "Info"
            </button>
            <button
                class={BTN_CLS}
                on:click={move |_| {
                    toaster
                        .add(
                            "File uploaded",
                            Some("document.pdf has been uploaded to your storage.".to_string()),
                            Some("success".to_string()),
                            None,
                        )
                }}
            >
                "With description"
            </button>
        </div>
    }
}
