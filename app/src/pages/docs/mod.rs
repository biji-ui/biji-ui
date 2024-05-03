use std::time::Duration;

use leptos::*;
use leptos_router::*;

pub mod accordion;
pub mod dialog;
pub mod dropdown_menu;
pub mod getting_started;
pub mod menubar;

use biji_ui::components::{
    dialog::{self as dialogui, context::DialogContext},
    menu,
};
use leptos_use::{use_color_mode, use_media_query, ColorMode, UseColorModeReturn};

use crate::icons;

#[component]
pub fn ThemeMode() -> impl IntoView {
    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode();

    let modes = [("Light", &ColorMode::Light), ("Dark", &ColorMode::Dark)];

    view! {
        <menu::Menu class="relative">
            <menu::Trigger class="flex h-6 w-6 items-center cursor-pointer justify-center rounded-md transition dark:hover:bg-white/5 hover:bg-zinc-900/5">
                <icons::Sun class="h-5 w-5 stroke-zinc-900 dark:hidden"></icons::Sun>
                <icons::Moon class="hidden h-5 w-5 stroke-white dark:block"></icons::Moon>
            </menu::Trigger>
            <menu::Content
                class="transition flex absolute right-0 flex-col p-1 w-40 rounded-md border shadow-md min-w-[8rem] bg-popover text-popover-foreground focus:outline-none"
                show_class="z-10 translate-y-0 opacity-100 duration-150 ease-in"
                hide_class="-z-10 translate-y-1 opacity-0 duration-200 ease-out"
                hide_delay={Duration::from_millis(200)}
            >
                {modes
                    .into_iter()
                    .map(|(title, m)| {
                        view! {
                            <menu::Item class="flex items-center py-1.5 px-2 text-sm rounded-sm cursor-pointer outline-none select-none focus:outline-none hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[disabled]:pointer-events-none data-[disabled]:opacity-50 data-[highlighted]:bg-muted">
                                <button
                                    on:click={move |_| { set_mode.set(m.clone()) }}
                                    class="flex w-full justify-between align-center"
                                >
                                    <div class="flex gap-2">
                                        {match m.clone() {
                                            ColorMode::Light => {
                                                view! { <icons::Sun class="w-4"></icons::Sun> }
                                            }
                                            ColorMode::Dark => {
                                                view! { <icons::Moon class="w-4"></icons::Moon> }
                                            }
                                            _ => view! { <icons::SunMoon class="w-4"></icons::SunMoon> },
                                        }}
                                        {title}
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
            <div class="hidden lg:block lg:max-w-md lg:flex-auto"></div>
            <div class="flex items-center gap-5 lg:hidden">
                <Sidebar/>
                <a aria-label="Home" href="/">
                    "biji"
                </a>
            </div>
            <div class="flex items-center gap-5">
                <nav class="hidden md:block">
                    <ul role="list" class="flex items-center gap-8">
                        <li>
                            <a
                                class="text-sm leading-5 text-zinc-600 transition dark:text-zinc-400 dark:hover:text-white hover:text-zinc-900"
                                href="https://docs.rs"
                            >
                                "API"
                            </a>
                        </li>
                        <li>
                            <a
                                class="text-sm leading-5 text-zinc-600 transition dark:text-zinc-400 dark:hover:text-white hover:text-zinc-900"
                                href="/"
                            >
                                "Documentation"
                            </a>
                        </li>
                        <li>
                            <a
                                class="text-sm leading-5 text-zinc-600 transition dark:text-zinc-400 dark:hover:text-white hover:text-zinc-900"
                                href="https://github.com"
                            >
                                "Support"
                            </a>
                        </li>
                    </ul>
                </nav>
                <div class="hidden md:block md:h-5 md:w-px md:bg-zinc-900/10 md:dark:bg-white/15"></div>
                <div class="flex gap-4">
                    <ThemeMode/>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn SidebarTrigger() -> impl IntoView {
    let ctx = expect_context::<DialogContext>();

    let is_large_screen = use_media_query("(min-width: 1024px)");

    create_effect(move |_| {
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
            <dialogui::Trigger class="flex h-6 w-6 items-center justify-center rounded-md transition dark:hover:bg-white/5 hover:bg-zinc-900/5">
                <SidebarTrigger/>
            </dialogui::Trigger>
            <Portal>
                <dialogui::Overlay
                    class="transition-opacity ease-linear duration-300 fixed inset-0 top-14 bg-zinc-400/20 backdrop-blur-sm dark:bg-black/40"
                    show_class="opacity-100"
                    hide_class="opacity-0"
                ></dialogui::Overlay>
                <dialogui::Content
                    class="transition ease-in-out duration-300 fixed bottom-0 left-0 top-14 w-full overflow-y-auto bg-white px-4 pb-4 pt-6 shadow-lg shadow-zinc-900/10 ring-1 ring-zinc-900/10 min-[416px]:max-w-sm sm:px-6 sm:pb-10 dark:bg-zinc-900 dark:ring-zinc-800"
                    show_class="translate-x-0"
                    hide_class="-translate-x-full"
                >
                    <div>
                        <SidebarNav/>
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

    let components = [
        ("/docs/accordion", "Accordion"),
        ("/docs/dialog", "Dialog"),
        ("/docs/dropdown-menu", "Dropdown Menu"),
        ("/docs/menubar", "Menubar"),
    ];

    let dialog_ctx = use_context::<DialogContext>();

    view! {
        <nav class={class}>
            <ul role="list">
                <li class="relative mt-6 md:mt-0">
                    <h2 class="text-xs font-semibold text-zinc-900 dark:text-white">
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
                                            class="flex justify-between gap-2 py-1 pr-3 text-sm transition pl-4 text-zinc-600 dark:text-zinc-400 dark:hover:text-white hover:text-zinc-900"
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
                    <h2 class="text-xs font-semibold text-zinc-900 dark:text-white">
                        "Components"
                    </h2>
                    <ul class="border-l border-transparent">
                        {components
                            .into_iter()
                            .map(|(path, title)| {
                                view! {
                                    <li class="relative">
                                        <a
                                            href={path}
                                            class="flex justify-between gap-2 py-1 pr-3 text-sm transition pl-4 text-zinc-600 dark:text-zinc-400 dark:hover:text-white hover:text-zinc-900"
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
    view! {
        <div class="h-full lg:ml-72 xl:ml-80">
            <header class="contents lg:pointer-events-none lg:fixed lg:inset-0 lg:z-40 lg:flex">
                <div class="contents lg:pointer-events-auto lg:block lg:w-72 lg:overflow-y-auto lg:border-r lg:border-zinc-900/10 lg:px-6 lg:pb-8 lg:pt-4 lg:dark:border-white/10 xl:w-80">
                    <div class="hidden lg:flex">
                        <a aria-label="Home" href="/">
                            "biji"
                        </a>
                    </div>
                    <TopNav/>
                    <SidebarNav class="hidden lg:mt-10 lg:block"/>
                </div>
            </header>
            <div class="relative flex h-full flex-col px-4 pt-14 sm:px-6 lg:px-8">
                <Outlet/>
            </div>
        </div>
    }
}

#[component]
pub fn DocPage<F, IV>(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    example: F,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <article class="flex h-full flex-col pb-10 pt-16">
            <h1 class="font-bold text-2xl mb-2">{title}</h1>
            <p class="mb-11 mt-3 text-balance text-base">{description}</p>
            <div class="relative rounded-2xl border-2 border-muted bg-zinc-50 !ring-transparent dark:bg-neutral-900/50">
                <div class="flex min-h-[443px] w-full items-center justify-center p-12 preview">
                    {example()}
                </div>
            </div>
        </article>
    }
}
