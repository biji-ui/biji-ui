use std::time::Duration;

use leptos::{portal::Portal, prelude::*};

use crate::components::{
    api_table::{KeyboardRow, KeyboardTable, PropRow, PropsTable, SectionHeading},
    button::{Variant, btn},
    code::Code,
};

const INSTALL_CODE: &str = concat!(
    "biji-ui = { version = \"",
    env!("CARGO_PKG_VERSION"),
    "\", features = [\"dialog\"] }",
);

const USAGE_CODE: &str = r#"use std::time::Duration;
use leptos::{portal::Portal, prelude::*};
use biji_ui::components::dialog;

#[component]
pub fn MyDialog() -> impl IntoView {
    view! {
        <dialog::Root hide_delay={Duration::from_millis(200)}>
            <dialog::Trigger class="rounded bg-indigo-600 px-4 py-2 text-white">
                "Open dialog"
            </dialog::Trigger>
            <Portal>
                <dialog::Overlay
                    class="fixed inset-0 z-[80] bg-black/40"
                    show_class="opacity-100 duration-300 ease-out"
                    hide_class="opacity-0 duration-200 ease-in"
                />
                <dialog::Content
                    class="fixed left-1/2 top-1/2 z-[90] -translate-x-1/2 -translate-y-1/2 rounded-lg bg-background p-6 shadow-xl"
                    show_class="opacity-100 scale-100 duration-300 ease-out"
                    hide_class="opacity-0 scale-95 duration-200 ease-in"
                >
                    <h2 class="text-lg font-semibold">"Payment successful"</h2>
                    <p class="mt-2 text-sm">"Your payment has been processed."</p>
                    <dialog::Close class="mt-4 inline-flex w-full justify-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white hover:bg-indigo-500">
                        "Go back to dashboard"
                    </dialog::Close>
                </dialog::Content>
            </Portal>
        </dialog::Root>
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
        description: "How long to wait before unmounting content and overlay after closing begins. Should match your CSS transition duration.",
    },
    PropRow {
        name: "prevent_scroll",
        prop_type: "bool",
        default: "true",
        description: "When true, prevents the page from scrolling while the dialog is open.",
    },
];

const TRIGGER_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the trigger button.",
}];

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

const CLOSE_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the close button.",
}];

const KEYBOARD: &[KeyboardRow] = &[
    KeyboardRow {
        key: "Tab",
        description: "Moves focus to the next focusable element inside the dialog. Focus is trapped within the dialog.",
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
pub fn DialogDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    view! {
        <DocPage title="Dialog">
            <p class="mt-3 mb-11 text-base text-balance">
                "A window overlaid on either the primary window or another dialog window, rendering the content underneath inert."
            </p>
            <DocPreview>
                <DialogExample />
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
            <PropsTable title="Content" rows={CONTENT_PROPS} />
            <PropsTable title="Overlay" rows={OVERLAY_PROPS} />
            <PropsTable title="Close" rows={CLOSE_PROPS} />
            <KeyboardTable rows={KEYBOARD} />
        </DocPage>
    }
}

#[component]
pub fn DialogExample() -> impl IntoView {
    use biji_ui::components::dialog;

    view! {
        <dialog::Root hide_delay={Duration::from_millis(200)}>
            <dialog::Trigger class={btn(Variant::Default)}>
                "Confirm payment"
            </dialog::Trigger>
            <Portal>
                <dialog::Overlay
                    class="fixed inset-0 z-[80] bg-zinc-400/20 backdrop-blur-sm transition-opacity duration-300 ease-linear dark:bg-black/40"
                    show_class="opacity-100 duration-300 ease-out"
                    hide_class="opacity-0 duration-200 ease-in"
                ></dialog::Overlay>
                <dialog::Content
                    class="z-[90] fixed top-[calc(100%-1rem)] left-[50%] md:top-[50%] w-full max-w-[94%] translate-x-[-50%] translate-y-[-100%] md:translate-y-[-50%] bg-background p-5 sm:max-w-[490px] md:w-full rounded-lg shadow-xl transition-all"
                    show_class="translate-y-0 opacity-100 duration-300 ease-out sm:scale-100"
                    hide_class="translate-y-4 opacity-0 duration-200 ease-in sm:translate-y-0 sm:scale-95"
                >
                    <div>
                        <div class="mx-auto flex h-12 w-12 items-center justify-center rounded-full bg-green-100">
                            <svg
                                class="h-6 w-6 text-green-600"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="currentColor"
                                aria-hidden="true"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M4.5 12.75l6 6 9-13.5"
                                ></path>
                            </svg>
                        </div>
                        <div class="mt-3 text-center sm:mt-5">
                            <h3
                                class="text-base leading-6 font-semibold text-gray-900 dark:text-gray-100"
                                id="modal-title"
                            >
                                "Payment successful"
                            </h3>
                            <div class="mt-2">
                                <p class="text-sm text-gray-500 dark:text-gray-400">
                                    "Lorem ipsum dolor sit amet consectetur adipisicing elit. Consequatur amet labore."
                                </p>
                            </div>
                        </div>
                    </div>
                    <div class="mt-5 sm:mt-6">
                        <dialog::Close class="inline-flex w-full justify-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline-2 focus-visible:outline focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
                            "Go back to dashboard"
                        </dialog::Close>
                    </div>
                </dialog::Content>
            </Portal>
        </dialog::Root>
    }
}
