use std::time::Duration;

use leptos::{portal::Portal, prelude::*};

use crate::components::{
    api_table::{DataAttrRow, DataAttrsTable, KeyboardRow, KeyboardTable, PropRow, PropsTable, SectionHeading},
    button::{Variant, btn},
    code::Code,
};

const INSTALL_CODE: &str = concat!(
    "biji-ui = { version = \"",
    env!("CARGO_PKG_VERSION"),
    "\", features = [\"alert_dialog\"] }",
);

const USAGE_CODE: &str = r#"use std::time::Duration;
use leptos::{portal::Portal, prelude::*};
use biji_ui::components::alert_dialog;

#[component]
pub fn MyAlertDialog() -> impl IntoView {
    view! {
        <alert_dialog::Root hide_delay={Duration::from_millis(200)}>
            <alert_dialog::Trigger class="rounded bg-red-600 px-4 py-2 text-white hover:bg-red-500">
                "Delete account"
            </alert_dialog::Trigger>
            <Portal>
                <alert_dialog::Overlay
                    class="fixed inset-0 z-50 bg-black/40"
                    show_class="opacity-100 duration-300 ease-out"
                    hide_class="opacity-0 duration-200 ease-in"
                />
                <alert_dialog::Content
                    class="fixed left-1/2 top-1/2 z-50 -translate-x-1/2 -translate-y-1/2 w-full max-w-md rounded-lg bg-background p-6 shadow-xl"
                    show_class="opacity-100 scale-100 duration-300 ease-out"
                    hide_class="opacity-0 scale-95 duration-200 ease-in"
                >
                    <alert_dialog::Title class="text-lg font-semibold">
                        "Are you absolutely sure?"
                    </alert_dialog::Title>
                    <alert_dialog::Description class="mt-2 text-sm text-muted-foreground">
                        "This action cannot be undone. This will permanently delete your account."
                    </alert_dialog::Description>
                    <div class="mt-6 flex justify-end gap-3">
                        <alert_dialog::Cancel class="rounded-md border px-4 py-2 text-sm font-medium hover:bg-accent">
                            "Cancel"
                        </alert_dialog::Cancel>
                        <alert_dialog::Action class="rounded-md bg-red-600 px-4 py-2 text-sm font-medium text-white hover:bg-red-500">
                            "Continue"
                        </alert_dialog::Action>
                    </div>
                </alert_dialog::Content>
            </Portal>
        </alert_dialog::Root>
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
        name: "hide_delay",
        prop_type: "Duration",
        default: "200ms",
        description: "How long to wait before unmounting content after closing begins. Should match your CSS transition duration.",
    },
    PropRow {
        name: "prevent_scroll",
        prop_type: "bool",
        default: "true",
        description: "When true, prevents the page from scrolling while the dialog is open.",
    },
    PropRow {
        name: "open",
        prop_type: "bool",
        default: "false",
        description: "Initial open state.",
    },
    PropRow {
        name: "on_open_change",
        prop_type: "Option<Callback<bool>>",
        default: "None",
        description: "Callback fired when the open state changes.",
    },
];

const TRIGGER_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the trigger button.",
}];

const OVERLAY_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied in both open and closed states.",
    },
    PropRow {
        name: "show_class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied when the overlay is visible.",
    },
    PropRow {
        name: "hide_class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied while the overlay is hiding.",
    },
];

const CONTENT_PROPS: &[PropRow] = &[
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied in both open and closed states.",
    },
    PropRow {
        name: "show_class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied when the dialog is open.",
    },
    PropRow {
        name: "hide_class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied while the dialog is closing.",
    },
];

const TITLE_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the title heading.",
}];

const DESCRIPTION_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the description paragraph.",
}];

const CANCEL_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the cancel button. Clicking closes the dialog.",
}];

const ACTION_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the action button. Does not auto-close; wire your own handler.",
}];

const DATA_ATTRS: &[DataAttrRow] = &[DataAttrRow {
    name: "data-state",
    description: "\"open\" when the dialog is visible; \"closed\" when hidden. Present on Trigger.",
}];

const KEYBOARD: &[KeyboardRow] = &[
    KeyboardRow {
        key: "Tab",
        description: "Moves focus to the next focusable element inside the dialog. Focus is trapped.",
    },
    KeyboardRow {
        key: "Shift + Tab",
        description: "Moves focus to the previous focusable element inside the dialog.",
    },
    KeyboardRow {
        key: "Escape",
        description: "Closes the dialog and returns focus to the trigger.",
    },
];

#[component]
pub fn AlertDialogDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    view! {
        <DocPage title="Alert Dialog">
            <p class="mt-3 mb-11 text-base text-balance">
                "A modal dialog that interrupts the user with important content and requires a deliberate response."
            </p>
            <DocPreview>
                <AlertDialogExample />
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
            <SectionHeading title="API Reference" />
            <PropsTable title="Root" rows={ROOT_PROPS} />
            <PropsTable title="Trigger" rows={TRIGGER_PROPS} />
            <PropsTable title="Overlay" rows={OVERLAY_PROPS} />
            <PropsTable title="Content" rows={CONTENT_PROPS} />
            <PropsTable title="Title" rows={TITLE_PROPS} />
            <PropsTable title="Description" rows={DESCRIPTION_PROPS} />
            <PropsTable title="Cancel" rows={CANCEL_PROPS} />
            <PropsTable title="Action" rows={ACTION_PROPS} />
            <DataAttrsTable rows={DATA_ATTRS} />
            <KeyboardTable rows={KEYBOARD} />
        </DocPage>
    }
}

#[component]
pub fn AlertDialogExample() -> impl IntoView {
    use biji_ui::components::alert_dialog;

    view! {
        <alert_dialog::Root hide_delay={Duration::from_millis(200)}>
            <alert_dialog::Trigger class={btn(Variant::Destructive)}>
                "Delete account"
            </alert_dialog::Trigger>
            <Portal>
                <alert_dialog::Overlay
                    class="fixed inset-0 z-50 bg-zinc-400/20 backdrop-blur-sm transition-opacity duration-300 ease-linear dark:bg-black/40"
                    show_class="opacity-100"
                    hide_class="opacity-0"
                />
                <alert_dialog::Content
                    class="fixed left-1/2 top-1/2 z-50 w-full max-w-[calc(100%-2rem)] sm:max-w-md rounded-lg bg-background p-6 shadow-xl transition-all"
                    show_class="opacity-100 scale-100 duration-300 ease-out translate-x-[-50%] translate-y-[-50%]"
                    hide_class="opacity-0 scale-95 duration-200 ease-in translate-x-[-50%] translate-y-[-50%]"
                >
                    <alert_dialog::Title class="text-lg font-semibold text-foreground">
                        "Are you absolutely sure?"
                    </alert_dialog::Title>
                    <alert_dialog::Description class="mt-2 text-sm text-muted-foreground">
                        "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                    </alert_dialog::Description>
                    <div class="mt-6 flex flex-col-reverse sm:flex-row justify-end gap-3">
                        <alert_dialog::Cancel class={btn(Variant::Outline)}>
                            "Cancel"
                        </alert_dialog::Cancel>
                        <alert_dialog::Action class={btn(Variant::Destructive)}>
                            "Yes, delete account"
                        </alert_dialog::Action>
                    </div>
                </alert_dialog::Content>
            </Portal>
        </alert_dialog::Root>
    }
}
