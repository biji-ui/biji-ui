use std::time::Duration;

use leptos::{portal::Portal, prelude::*};
use leptos_router::{components::*, hooks::use_location};

pub mod accordion;
pub mod alert_dialog;
pub mod calendar;
pub mod checkbox;
pub mod collapsible;
pub mod combobox;
pub mod command;
pub mod context_menu;
pub mod dialog;
pub mod dropdown_menu;
pub mod getting_started;
pub mod menubar;
pub mod pin_input;
pub mod popover;
pub mod progress;
pub mod radio_group;
pub mod select;
pub mod separator;
pub mod slider;
pub mod switch;
pub mod tabs;
pub mod toggle_group;
pub mod tooltip;

use biji_ui::components::{
    command as commandui,
    dialog::{self as dialogui, context::DialogContext},
    menu,
};
use biji_ui::custom_animated_show::CustomAnimatedShow;
use leptos::ev::keydown;
use leptos_router::hooks::use_navigate;
use leptos_use::{
    ColorMode, UseColorModeReturn, use_color_mode, use_document, use_event_listener,
    use_media_query,
};

use crate::icons;

/// All component doc pages — single source of truth for sidebar nav and command palette.
pub const COMPONENT_PAGES: &[(&str, &str)] = &[
    ("/docs/accordion", "Accordion"),
    ("/docs/alert-dialog", "Alert Dialog"),
    ("/docs/calendar", "Calendar"),
    ("/docs/checkbox", "Checkbox"),
    ("/docs/collapsible", "Collapsible"),
    ("/docs/combobox", "Combobox"),
    ("/docs/command", "Command"),
    ("/docs/context-menu", "Context Menu"),
    ("/docs/dialog", "Dialog"),
    ("/docs/dropdown-menu", "Dropdown Menu"),
    ("/docs/menubar", "Menubar"),
    ("/docs/pin-input", "PIN Input"),
    ("/docs/popover", "Popover"),
    ("/docs/progress", "Progress"),
    ("/docs/radio-group", "Radio Group"),
    ("/docs/select", "Select"),
    ("/docs/separator", "Separator"),
    ("/docs/slider", "Slider"),
    ("/docs/switch", "Switch"),
    ("/docs/tabs", "Tabs"),
    ("/docs/toggle-group", "Toggle Group"),
    ("/docs/tooltip", "Tooltip"),
];

#[derive(Copy, Clone)]
pub struct PaletteCtx {
    pub open: RwSignal<bool>,
}

impl PaletteCtx {
    pub fn open(&self) {
        self.open.set(true);
    }
    pub fn close(&self) {
        self.open.set(false);
    }
}

#[component]
pub fn ThemeMode() -> impl IntoView {
    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode();

    let modes = [("Light", &ColorMode::Light), ("Dark", &ColorMode::Dark)];

    view! {
        <menu::Menu
            positioning={menu::Positioning::BottomEnd}
            hide_delay={Duration::from_millis(200)}
        >
            <menu::Trigger class="flex justify-center items-center w-6 h-6 rounded-md transition cursor-pointer dark:hover:bg-white/5 hover:bg-zinc-900/5">
                <icons::Sun class="w-5 h-5 dark:hidden stroke-zinc-900"></icons::Sun>
                <icons::Moon class="hidden w-5 h-5 dark:block stroke-white"></icons::Moon>
            </menu::Trigger>
            <Portal>
                <menu::Content
                    class="flex z-40 flex-col p-1 w-40 rounded-md border shadow-md focus:outline-none transition-[opacity,transform] min-w-[8rem] border-border bg-background text-foreground"
                    show_class="opacity-100 duration-150 ease-in"
                    hide_class="opacity-0 duration-200 ease-out"
                >
                    {modes
                        .into_iter()
                        .map(|(title, m)| {
                            view! {
                                <menu::Item class="flex items-center text-sm rounded-sm cursor-pointer outline-none select-none focus:outline-none hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[disabled]:pointer-events-none data-[disabled]:opacity-50 data-[highlighted]:bg-muted">
                                    <button
                                        on:click={move |_| { set_mode.set(m.clone()) }}
                                        class="flex justify-between py-1.5 px-2 w-full align-center"
                                    >
                                        <div class="flex gap-2">
                                            {match m.clone() {
                                                ColorMode::Light => {
                                                    view! { <icons::Sun class="w-4"></icons::Sun> }.into_any()
                                                }
                                                ColorMode::Dark => {
                                                    view! { <icons::Moon class="w-4"></icons::Moon> }.into_any()
                                                }
                                                _ => {
                                                    view! { <icons::SunMoon class="w-4"></icons::SunMoon> }
                                                        .into_any()
                                                }
                                            }} {title}
                                        </div>
                                        <Show when={move || m.clone() == mode.get()}>
                                            <icons::Check class="w-4"></icons::Check>
                                        </Show>
                                    </button>
                                </menu::Item>
                            }
                        })
                        .collect_view()}
                </menu::Content>
            </Portal>
        </menu::Menu>
    }
}

#[component]
pub fn TopNav() -> impl IntoView {
    view! {
        <div
            style="--bg-opacity-light: 0.5; --bg-opacity-dark: 0.2; --scrollbar-width-nav: var(--scrollbar-width, 0px);"
            class="fixed inset-x-0 top-0 z-10 flex h-14 items-center justify-between gap-12 pl-4 pr-[calc(var(--scrollbar-width-nav)+1rem)] transition sm:pl-6 sm:pr-[calc(var(--scrollbar-width-nav)+1.5rem)] lg:left-72 lg:z-30 lg:pl-8 lg:pr-[calc(var(--scrollbar-width-nav)+2rem)] xl:left-80 backdrop-blur-sm lg:left-72 xl:left-80 dark:backdrop-blur bg-white/[var(--bg-opacity-light)] dark:bg-zinc-900/[var(--bg-opacity-dark)]"
        >
            <div class="absolute inset-x-0 top-full h-px transition bg-zinc-900/10 dark:bg-white/10"></div>
            // Desktop center: full-width search input
            <div class="hidden lg:block lg:flex-auto lg:max-w-md">
                <SearchButton />
            </div>
            <div class="flex gap-5 items-center lg:hidden">
                <Sidebar />
                <a aria-label="Home" href="/">
                    <icons::BijiUI class="w-auto h-5"></icons::BijiUI>
                </a>
            </div>
            <div class="flex gap-5 items-center">
                <nav class="hidden md:block">
                    <ul role="list" class="flex gap-5 items-center">
                        <li>
                            <a
                                class="text-sm leading-5 transition text-zinc-600 dark:text-zinc-400 dark:hover:text-white hover:text-zinc-900"
                                href="https://docs.rs/biji-ui/latest/biji_ui/"
                                title="Documentation"
                            >
                                <icons::BookText class="w-5 h-5 stroke-zinc-600 dark:stroke-zinc-400"></icons::BookText>
                            </a>
                        </li>
                        <li>
                            <a
                                class="text-sm leading-5 transition text-zinc-600 dark:text-zinc-400 dark:hover:text-white hover:text-zinc-900"
                                href="https://github.com/biji-ui/biji-ui"
                                title="Github"
                            >
                                <icons::Github class="w-5 h-5 stroke-zinc-600 dark:stroke-zinc-400"></icons::Github>
                            </a>
                        </li>
                        <li>
                            <a
                                class="text-sm leading-5 transition text-zinc-600 dark:text-zinc-400 dark:hover:text-white hover:text-zinc-900"
                                href="https://github.com/biji-ui/biji-ui/issues"
                                title="Report an issue"
                            >
                                <icons::Bug class="w-5 h-5 stroke-zinc-600 dark:stroke-zinc-400"></icons::Bug>
                            </a>
                        </li>
                    </ul>
                </nav>
                <div class="hidden md:block md:w-px md:h-5 md:bg-zinc-900/10 md:dark:bg-white/15"></div>
                <div class="flex gap-4 items-center">
                    // Mobile/tablet: compact icon-only search button
                    <div class="lg:hidden">
                        <SearchButtonCompact />
                    </div>
                    <ThemeMode />
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn SidebarTrigger() -> impl IntoView {
    let ctx = expect_context::<DialogContext>();

    let is_large_screen = use_media_query("(min-width: 1024px)");

    Effect::new(move |_| {
        if is_large_screen.get() {
            ctx.close();
        }
    });

    view! {
        <Show
            when={move || !ctx.open.get()}
            fallback={|| {
                view! { <icons::X class="w-2.5 stroke-zinc-900 dark:stroke-white"></icons::X> }
            }}
        >

            <icons::AlignJustify class="w-2.5 stroke-zinc-900 dark:stroke-white"></icons::AlignJustify>
        </Show>
    }
}

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <dialogui::Root hide_delay={Duration::from_millis(300)}>
            <dialogui::Trigger class="flex justify-center items-center w-6 h-6 rounded-md transition dark:hover:bg-white/5 hover:bg-zinc-900/5">
                <SidebarTrigger />
            </dialogui::Trigger>
            <Portal>
                <dialogui::Overlay
                    class="fixed inset-0 top-14 transition-opacity duration-300 ease-linear bg-zinc-400/20 backdrop-blur-sm dark:bg-black/40"
                    show_class="opacity-100"
                    hide_class="opacity-0"
                ></dialogui::Overlay>
                <dialogui::Content
                    class="overflow-y-auto fixed bottom-0 left-0 top-14 px-4 pt-6 pb-4 w-full bg-white ring-1 shadow-lg transition duration-300 ease-in-out sm:px-6 sm:pb-10 shadow-zinc-900/10 ring-zinc-900/10 min-[416px]:max-w-sm dark:bg-zinc-900 dark:ring-zinc-800"
                    show_class="translate-x-0"
                    hide_class="-translate-x-full"
                >
                    <div>
                        <SidebarNav />
                    </div>
                </dialogui::Content>
            </Portal>
        </dialogui::Root>
    }
}

#[component]
pub fn SidebarNav(#[prop(into, optional)] class: String) -> impl IntoView {
    let location = use_location();

    let introduction = [("/docs/getting-started", "Getting started")];

    let components = COMPONENT_PAGES;

    let dialog_ctx = use_context::<DialogContext>();

    view! {
        <nav class={class}>
            <ul role="list">
                <li class="md:hidden">
                    <a
                        class="block flex py-1 text-sm transition text-zinc-600 dark:text-zinc-400 dark:hover:text-white hover:text-zinc-900"
                        href="https://docs.rs/biji-ui/latest/biji_ui/"
                    >
                        "Documentation"
                    </a>
                </li>
                <li class="md:hidden">
                    <a
                        class="block flex py-1 text-sm transition text-zinc-600 dark:text-zinc-400 dark:hover:text-white hover:text-zinc-900"
                        href="https://github.com/biji-ui/biji-ui"
                    >
                        "Github"
                    </a>
                </li>
                <li class="md:hidden">
                    <a
                        class="block flex py-1 text-sm transition text-zinc-600 dark:text-zinc-400 dark:hover:text-white hover:text-zinc-900"
                        href="https://github.com/biji-ui/biji-ui/issues"
                    >
                        "Report an issue"
                    </a>
                </li>
            </ul>
            <ul role="list">
                <li class="relative mt-6 md:mt-0">
                    <h2 class="text-xs font-semibold dark:text-white text-zinc-900">
                        "Introduction"
                    </h2>
                    <ul class="border-l border-transparent">
                        {introduction
                            .into_iter()
                            .map(|(path, title)| {
                                view! {
                                    <li class="relative">
                                        <a
                                            href={path}
                                            class="flex gap-2 justify-between py-1 pr-3 pl-4 text-sm transition text-zinc-600 dark:text-zinc-400 dark:hover:text-white hover:text-zinc-900"
                                            class:font-medium={move || {
                                                location.pathname.get() == path
                                            }}

                                            on:click={move |_| {
                                                if let Some(ctx) = dialog_ctx {
                                                    ctx.close();
                                                }
                                            }}
                                        >

                                            {title}
                                        </a>
                                    </li>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </ul>
                </li>
                <li class="relative mt-6">
                    <h2 class="text-xs font-semibold dark:text-white text-zinc-900">
                        "Components"
                    </h2>
                    <ul class="border-l border-transparent">
                        {components
                            .iter()
                            .copied()
                            .map(|(path, title)| {
                                view! {
                                    <li class="relative">
                                        <a
                                            href={path}
                                            class="flex gap-2 justify-between py-1 pr-3 pl-4 text-sm transition text-zinc-600 dark:text-zinc-400 dark:hover:text-white hover:text-zinc-900"
                                            class:font-medium={move || {
                                                location.pathname.get() == path
                                            }}

                                            on:click={move |_| {
                                                if let Some(ctx) = dialog_ctx {
                                                    ctx.close();
                                                }
                                            }}
                                        >

                                            {title}
                                        </a>
                                    </li>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </ul>
                </li>
            </ul>
        </nav>
    }
}

#[component]
pub fn DocsPage() -> impl IntoView {
    let palette_ctx = PaletteCtx {
        open: RwSignal::new(false),
    };

    let _ = use_event_listener(use_document(), keydown, move |evt| {
        let key = evt.key();
        let meta = evt.meta_key() || evt.ctrl_key();
        if meta && key == "k" {
            evt.prevent_default();
            if palette_ctx.open.get_untracked() {
                palette_ctx.close();
            } else {
                palette_ctx.open();
            }
        }
    });

    view! {
        <leptos::context::Provider value={palette_ctx}>
            <div class="h-full lg:ml-72 xl:ml-80">
                <header class="contents lg:flex lg:fixed lg:inset-0 lg:z-40 lg:pointer-events-none">
                    <div class="contents lg:block lg:overflow-y-auto lg:px-6 lg:pt-4 lg:pb-8 lg:w-72 lg:border-r lg:pointer-events-auto xl:w-80 lg:border-zinc-900/10 lg:dark:border-white/10">
                        <div class="hidden lg:flex">
                            <a aria-label="Home" href="/">
                                <icons::BijiUI class="w-auto h-5"></icons::BijiUI>
                            </a>
                        </div>
                        <TopNav />
                        <SidebarNav class="hidden lg:block lg:mt-10" />
                    </div>
                </header>
                <div class="flex relative flex-col px-4 pt-14 h-full sm:px-6 lg:px-8">
                    <Outlet />
                    <icons::HeroPattern></icons::HeroPattern>
                </div>
            </div>
            <Portal>
                <CommandPalette />
            </Portal>
        </leptos::context::Provider>
    }
}

#[component]
pub fn DocPage(#[prop(into)] title: String, children: Children) -> impl IntoView {
    view! {
        <article class="flex flex-col pt-16 pb-10 h-full">
            <h1 class="mb-2 text-2xl font-bold">{title}</h1>
            {children()}
        </article>
    }
}

#[component]
pub fn DocPreview(children: Children) -> impl IntoView {
    view! {
        <div class="relative rounded-2xl border-2 border-muted bg-zinc-50 !ring-transparent dark:bg-neutral-900/50">
            <div class="flex justify-center items-center p-12 w-full min-h-[443px] preview">
                {children()}
            </div>
        </div>
    }
}

/// Full input-style search button for desktop (lg+).
#[component]
pub fn SearchButton() -> impl IntoView {
    let ctx = use_context::<PaletteCtx>();
    let is_touch = use_media_query("(pointer: coarse)");

    let is_mac = RwSignal::new(false);
    Effect::new(move |_| {
        if let Ok(ua) = window().navigator().user_agent() {
            is_mac.set(ua.contains("Macintosh"));
        }
    });

    view! {
        <button
            type="button"
            class="flex items-center w-full h-9 gap-2 px-3 rounded-lg border border-zinc-900/10 dark:border-white/10 bg-white/60 dark:bg-white/5 text-sm text-zinc-500 dark:text-zinc-400 transition hover:border-zinc-900/20 dark:hover:border-white/20 hover:bg-white dark:hover:bg-white/10 cursor-pointer"
            on:click={move |_| { if let Some(c) = ctx { c.open(); } }}
            aria-label="Search docs"
        >
            <icons::Search class="w-3.5 h-3.5 shrink-0 stroke-zinc-900 dark:stroke-white" />
            <span class="flex-1 text-left">"Search docs..."</span>
            <Show when={move || !is_touch.get()}>
                <span class="flex items-center gap-1 text-xs text-zinc-400 dark:text-zinc-500">
                    <kbd class="font-sans">{move || if is_mac.get() { "⌘" } else { "Ctrl" }}</kbd>
                    <kbd class="font-sans">"K"</kbd>
                </span>
            </Show>
        </button>
    }
}

/// Compact icon-only search button for mobile/tablet (< lg).
#[component]
pub fn SearchButtonCompact() -> impl IntoView {
    let ctx = use_context::<PaletteCtx>();
    view! {
        <button
            type="button"
            class="flex justify-center items-center w-6 h-6 rounded-md transition cursor-pointer dark:hover:bg-white/5 hover:bg-zinc-900/5"
            on:click={move |_| { if let Some(c) = ctx { c.open(); } }}
            aria-label="Search docs"
        >
            <icons::Search class="w-4 h-4 stroke-zinc-900 dark:stroke-white"></icons::Search>
        </button>
    }
}

#[component]
pub fn CommandPalette() -> impl IntoView {
    use biji_ui::utils::prevent_scroll::use_prevent_scroll;
    use std::time::Duration;
    let ctx = expect_context::<PaletteCtx>();
    let navigate = use_navigate();
    let navigate = StoredValue::new(navigate);

    let eff = use_prevent_scroll(move || ctx.open.get(), Duration::from_millis(150));
    on_cleanup(move || drop(eff));

    let _ = use_event_listener(use_document(), keydown, move |evt| {
        if evt.key() == "Escape" && ctx.open.get_untracked() {
            evt.prevent_default();
            ctx.close();
        }
    });

    const ITEM_CLS: &str = "flex items-center px-3 py-2 text-sm rounded-sm cursor-pointer outline-none \
        data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground \
        data-[disabled]:opacity-50 data-[disabled]:pointer-events-none";

    view! {
        <CustomAnimatedShow
            when={Signal::derive(move || ctx.open.get())}
            show_class="opacity-100"
            hide_class="opacity-0 pointer-events-none"
            hide_delay={Duration::from_millis(150)}
            style="position: fixed; inset: 0; z-index: 9999; transition: opacity 150ms ease;"
        >
            <div
                class="absolute inset-0 bg-black/50 backdrop-blur-sm"
                on:click={move |_| ctx.close()}
            />
            <div
                class="relative px-4 mx-auto w-full max-w-md mt-[15vh]"
                on:click={|e| e.stop_propagation()}
            >
                <commandui::Root class="overflow-hidden rounded-xl border shadow-2xl border-border bg-background">
                    <PaletteAutoFocus />
                    <commandui::Input
                        placeholder="Search components..."
                        class="py-3 px-4 w-full text-sm border-b outline-none border-border bg-background placeholder:text-muted-foreground"
                    />
                    <commandui::List class="overflow-y-auto p-1 max-h-72">
                        <commandui::Empty>
                            <div class="py-8 text-sm text-center text-muted-foreground">
                                "No components found."
                            </div>
                        </commandui::Empty>
                        {COMPONENT_PAGES
                            .iter()
                            .map(|(path, label)| {
                                let path = *path;
                                let label = label.to_string();
                                let label_for_item = label.clone();
                                view! {
                                    <commandui::Item
                                        value={path}
                                        label={label_for_item}
                                        class={ITEM_CLS}
                                        on_select={Callback::new(move |_| {
                                            navigate
                                                .with_value(|nav| {
                                                    nav(path, Default::default());
                                                });
                                            ctx.close();
                                        })}
                                    >
                                        <HighlightedText
                                        label={label}
                                        highlight_class="bg-yellow-200/80 dark:bg-yellow-500/30 rounded"
                                    />
                                    </commandui::Item>
                                }
                            })
                            .collect_view()}
                    </commandui::List>
                </commandui::Root>
                <p class="mt-2 text-xs text-center text-muted-foreground">
                    <kbd class="font-mono">{"Esc"}</kbd>
                    " to close · "
                    <kbd class="font-mono">{"↑↓"}</kbd>
                    " to navigate · "
                    <kbd class="font-mono">{"↵"}</kbd>
                    " to select"
                </p>
            </div>
        </CustomAnimatedShow>
    }
}

/// Focuses the command input and resets query+focus when the palette opens.
/// Must be rendered inside `commandui::Root`.
#[component]
fn PaletteAutoFocus() -> impl IntoView {
    use biji_ui::components::command::context::CommandContext;

    let palette_ctx = expect_context::<PaletteCtx>();
    let cmd_ctx = expect_context::<CommandContext>();

    Effect::new(move |prev: Option<bool>| {
        let open = palette_ctx.open.get();
        if open && !prev.unwrap_or(false) {
            cmd_ctx.query.set(String::new());
            cmd_ctx.item_focus.set(None);
            let input_ref = cmd_ctx.input_ref;
            set_timeout(
                move || {
                    if let Some(el) = input_ref.get() {
                        el.set_value("");
                        let _ = el.focus();
                    }
                },
                std::time::Duration::from_millis(10),
            );
        }
        open
    });
}

pub use commandui::HighlightedText;
