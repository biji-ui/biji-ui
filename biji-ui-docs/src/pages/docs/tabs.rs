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
    "\", features = [\"tabs\"] }",
);

const USAGE_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::tabs;

#[component]
pub fn MyTabs() -> impl IntoView {
    view! {
        <tabs::Root default_value="account" class="w-full max-w-md">
            <tabs::List class="flex border-b border-border">
                <tabs::Trigger
                    value="account"
                    class="px-4 py-2 text-sm font-medium border-b-2 -mb-px data-[state=active]:border-primary data-[state=inactive]:border-transparent"
                >
                    "Account"
                </tabs::Trigger>
                <tabs::Trigger
                    value="password"
                    class="px-4 py-2 text-sm font-medium border-b-2 -mb-px data-[state=active]:border-primary data-[state=inactive]:border-transparent"
                >
                    "Password"
                </tabs::Trigger>
            </tabs::List>
            <tabs::Content value="account" class="p-4 text-sm">
                "Manage your account settings here."
            </tabs::Content>
            <tabs::Content value="password" class="p-4 text-sm">
                "Change your password here."
            </tabs::Content>
        </tabs::Root>
    }
}"#;

const ROOT_WITH_CODE: &str = r#"use leptos::prelude::*;
use biji_ui::components::tabs;

#[component]
pub fn MyTabs() -> impl IntoView {
    view! {
        <tabs::RootWith default_value="account" class="w-full max-w-md" let:t>
            <p class="mb-2 text-sm text-muted-foreground">
                {move || format!("Active tab: {}", t.value.get().unwrap_or_default())}
            </p>
            <tabs::List class="flex border-b border-border">
                <tabs::Trigger
                    value="account"
                    class="px-4 py-2 text-sm font-medium border-b-2 -mb-px data-[state=active]:border-primary data-[state=inactive]:border-transparent"
                >
                    "Account"
                </tabs::Trigger>
                <tabs::Trigger
                    value="password"
                    class="px-4 py-2 text-sm font-medium border-b-2 -mb-px data-[state=active]:border-primary data-[state=inactive]:border-transparent"
                >
                    "Password"
                </tabs::Trigger>
            </tabs::List>
            <tabs::Content value="account" class="p-4 text-sm">
                "Manage your account settings here."
            </tabs::Content>
            <tabs::Content value="password" class="p-4 text-sm">
                "Change your password here."
            </tabs::Content>
        </tabs::RootWith>
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
        name: "value",
        prop_type: "Option<String>",
        default: "None",
        description: "The initially active tab value.",
    },
    PropRow {
        name: "orientation",
        prop_type: "Orientation",
        default: "Horizontal",
        description: "The axis along which tabs are arranged.",
    },
    PropRow {
        name: "activation_mode",
        prop_type: "ActivationMode",
        default: "Automatic",
        description: "Whether arrow key navigation also activates tabs (Automatic) or only moves focus (Manual).",
    },
    PropRow {
        name: "on_value_change",
        prop_type: "Option<Callback<String>>",
        default: "None",
        description: "Callback fired when the active tab changes.",
    },
];

const LIST_PROPS: &[PropRow] = &[PropRow {
    name: "class",
    prop_type: "String",
    default: "\"\"",
    description: "CSS class applied to the tab list element.",
}];

const TRIGGER_PROPS: &[PropRow] = &[
    PropRow {
        name: "value",
        prop_type: "String",
        default: "",
        description: "The value this tab trigger activates.",
    },
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the tab trigger button.",
    },
    PropRow {
        name: "disabled",
        prop_type: "bool",
        default: "false",
        description: "When true, the tab cannot be activated.",
    },
];

const CONTENT_PROPS: &[PropRow] = &[
    PropRow {
        name: "value",
        prop_type: "String",
        default: "",
        description: "The tab value whose panel this renders.",
    },
    PropRow {
        name: "class",
        prop_type: "String",
        default: "\"\"",
        description: "CSS class applied to the tab panel element.",
    },
];

const ORIENTATION_PROPS: &[PropRow] = &[
    PropRow {
        name: "Horizontal",
        prop_type: "Orientation",
        default: "default",
        description: "Tabs are arranged left-to-right. Arrow Left/Right navigates.",
    },
    PropRow {
        name: "Vertical",
        prop_type: "Orientation",
        default: "",
        description: "Tabs are arranged top-to-bottom. Arrow Up/Down navigates.",
    },
];

const ACTIVATION_MODE_PROPS: &[PropRow] = &[
    PropRow {
        name: "Automatic",
        prop_type: "ActivationMode",
        default: "default",
        description: "Arrow key navigation both moves focus and activates the tab.",
    },
    PropRow {
        name: "Manual",
        prop_type: "ActivationMode",
        default: "",
        description: "Arrow key navigation only moves focus; Enter or Space activates the tab.",
    },
];

const DATA_ATTRS: &[DataAttrRow] = &[
    DataAttrRow {
        name: "data-state",
        description: "\"active\" when the tab is selected; \"inactive\" otherwise. Present on Trigger and Content.",
    },
    DataAttrRow {
        name: "data-disabled",
        description: "Present on Trigger when the tab is disabled.",
    },
    DataAttrRow {
        name: "data-orientation",
        description: "\"horizontal\" or \"vertical\". Present on Root, Trigger, and Content.",
    },
];

const KEYBOARD: &[KeyboardRow] = &[
    KeyboardRow {
        key: "ArrowRight / ArrowDown",
        description: "Moves focus to the next tab (ArrowRight for horizontal, ArrowDown for vertical). Activates it in Automatic mode.",
    },
    KeyboardRow {
        key: "ArrowLeft / ArrowUp",
        description: "Moves focus to the previous tab. Activates it in Automatic mode.",
    },
    KeyboardRow {
        key: "Home",
        description: "Moves focus to the first tab. Activates it in Automatic mode.",
    },
    KeyboardRow {
        key: "End",
        description: "Moves focus to the last tab. Activates it in Automatic mode.",
    },
    KeyboardRow {
        key: "Enter / Space",
        description: "Activates the focused tab (primarily useful in Manual activation mode).",
    },
];

#[component]
pub fn TabsDocPage() -> impl IntoView {
    use crate::pages::docs::{DocPage, DocPreview};

    view! {
        <DocPage title="Tabs">
            <p class="mt-3 mb-11 text-base text-balance">
                "A set of layered panels that display one at a time, activated by corresponding tab triggers."
            </p>
            <DocPreview>
                <TabsExample />
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
            <p class="mb-5 text-sm text-muted-foreground">
                "Use "
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"RootWith"</code>
                " to access "
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"TabsState"</code>
                " inline via the "
                <code class="text-xs font-mono bg-muted px-1 py-0.5 rounded">"let:"</code>
                " binding — useful for driving external UI from the active tab value."
            </p>
            <DocPreview>
                <TabsRootWithExample />
            </DocPreview>
            <Code
                class="[&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
                code={ROOT_WITH_CODE}
                language="rust"
            />
            <SectionHeading title="API Reference" />
            <PropsTable title="Root / RootWith" rows={ROOT_PROPS} />
            <PropsTable title="List" rows={LIST_PROPS} />
            <PropsTable title="Trigger" rows={TRIGGER_PROPS} />
            <PropsTable title="Content" rows={CONTENT_PROPS} />
            <PropsTable title="Orientation" rows={ORIENTATION_PROPS} />
            <PropsTable title="ActivationMode" rows={ACTIVATION_MODE_PROPS} />
            <DataAttrsTable rows={DATA_ATTRS} />
            <KeyboardTable rows={KEYBOARD} />
        </DocPage>
    }
}

#[component]
pub fn TabsRootWithExample() -> impl IntoView {
    use biji_ui::components::tabs;

    const TRIGGER_CLS: &str =
        "px-4 py-2 text-sm font-medium border-b-2 -mb-px transition-colors \
         focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring \
         data-[state=active]:border-primary data-[state=active]:text-foreground \
         data-[state=inactive]:border-transparent data-[state=inactive]:text-muted-foreground \
         hover:text-foreground";

    view! {
        <div class="flex flex-col items-center gap-4 p-8">
            <tabs::RootWith default_value="account".to_string() class="w-full max-w-sm" let:t>
                <p class="text-sm text-muted-foreground">
                    {move || format!("Active: {}", t.value.get().unwrap_or_default())}
                </p>
                <tabs::List class="flex border-b border-border">
                    <tabs::Trigger value="account" class={TRIGGER_CLS}>"Account"</tabs::Trigger>
                    <tabs::Trigger value="password" class={TRIGGER_CLS}>"Password"</tabs::Trigger>
                    <tabs::Trigger value="settings" class={TRIGGER_CLS}>"Settings"</tabs::Trigger>
                </tabs::List>
                <tabs::Content value="account" class="p-4 text-sm text-muted-foreground">
                    "Manage your account settings here."
                </tabs::Content>
                <tabs::Content value="password" class="p-4 text-sm text-muted-foreground">
                    "Change your password here."
                </tabs::Content>
                <tabs::Content value="settings" class="p-4 text-sm text-muted-foreground">
                    "Configure your preferences here."
                </tabs::Content>
            </tabs::RootWith>
        </div>
    }
}

#[component]
pub fn TabsExample() -> impl IntoView {
    use biji_ui::components::tabs;

    const TRIGGER_CLS: &str =
        "px-4 py-2 text-sm font-medium border-b-2 -mb-px transition-colors \
         focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring \
         data-[state=active]:border-primary data-[state=active]:text-foreground \
         data-[state=inactive]:border-transparent data-[state=inactive]:text-muted-foreground \
         hover:text-foreground";

    const INPUT_CLS: &str = "px-3 h-9 w-full text-sm rounded-md border border-input \
        bg-background focus:ring-2 focus:outline-none focus:ring-ring";

    const BTN_CLS: &str = "inline-flex items-center justify-center rounded-md px-4 h-9 text-sm \
        font-medium bg-primary text-primary-foreground hover:opacity-90 transition-opacity";

    view! {
        <div class="w-full max-w-md">
            <tabs::Root default_value="account".to_string() class="w-full">
                <tabs::List class="flex border-b border-border">
                    <tabs::Trigger value="account" class={TRIGGER_CLS}>"Account"</tabs::Trigger>
                    <tabs::Trigger value="password" class={TRIGGER_CLS}>"Password"</tabs::Trigger>
                    <tabs::Trigger value="notifications" class={TRIGGER_CLS}>"Notifications"</tabs::Trigger>
                </tabs::List>

                <tabs::Content value="account" class="p-4 flex flex-col gap-4">
                    <p class="text-sm text-muted-foreground">"Update your profile details."</p>
                    <div class="flex flex-col gap-1">
                        <label class="text-xs font-medium">"Display name"</label>
                        <input type="text" value="Jane Doe" class={INPUT_CLS} />
                    </div>
                    <div class="flex flex-col gap-1">
                        <label class="text-xs font-medium">"Email"</label>
                        <input type="email" value="jane@example.com" class={INPUT_CLS} />
                    </div>
                    <div>
                        <button class={BTN_CLS}>"Save changes"</button>
                    </div>
                </tabs::Content>

                <tabs::Content value="password" class="p-4 flex flex-col gap-4">
                    <p class="text-sm text-muted-foreground">"Change your password."</p>
                    <div class="flex flex-col gap-1">
                        <label class="text-xs font-medium">"Current password"</label>
                        <input type="password" class={INPUT_CLS} />
                    </div>
                    <div class="flex flex-col gap-1">
                        <label class="text-xs font-medium">"New password"</label>
                        <input type="password" class={INPUT_CLS} />
                    </div>
                    <div class="flex flex-col gap-1">
                        <label class="text-xs font-medium">"Confirm new password"</label>
                        <input type="password" class={INPUT_CLS} />
                    </div>
                    <div>
                        <button class={BTN_CLS}>"Update password"</button>
                    </div>
                </tabs::Content>

                <tabs::Content value="notifications" class="p-4 flex flex-col gap-1">
                    <p class="text-sm text-muted-foreground mb-2">
                        "Manage how you receive notifications."
                    </p>
                    {[
                        ("Comments", "Get notified when someone comments on your post."),
                        ("Mentions", "Get notified when you are mentioned."),
                        ("Updates", "Receive product news and announcements."),
                    ]
                    .into_iter()
                    .map(|(label, desc)| {
                        view! {
                            <div class="flex items-center justify-between py-2.5 border-b border-border last:border-0">
                                <div>
                                    <p class="text-sm font-medium">{label}</p>
                                    <p class="text-xs text-muted-foreground">{desc}</p>
                                </div>
                                <input
                                    type="checkbox"
                                    checked
                                    class="h-4 w-4 rounded border-input accent-primary cursor-pointer"
                                />
                            </div>
                        }
                    })
                    .collect_view()}
                </tabs::Content>
            </tabs::Root>
        </div>
    }
}
