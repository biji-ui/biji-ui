use leptos::*;
use leptos_router::*;

pub mod accordion;
pub mod dialog;
pub mod dropdown_menu;
pub mod getting_started;
pub mod menubar;

#[component]
pub fn DocsPage() -> impl IntoView {
    let location = use_location();

    let introduction = [("/docs/getting-started", "Getting started")];

    let components = [
        ("/docs/accordion", "Accordion"),
        ("/docs/dialog", "Dialog"),
        ("/docs/dropdown-menu", "Dropdown Menu"),
        ("/docs/menubar", "Menubar"),
    ];

    view! {
        <div class="h-full lg:ml-72 xl:ml-80">
            <header class="contents lg:pointer-events-none lg:fixed lg:inset-0 lg:z-40 lg:flex">
                <div class="contents lg:pointer-events-auto lg:block lg:w-72 lg:overflow-y-auto lg:border-r lg:border-zinc-900/10 lg:px-6 lg:pb-8 lg:pt-4 lg:dark:border-white/10 xl:w-80">
                    <div class="hidden lg:flex">
                        <a aria-label="Home" href="/">
                            "Biji UI"
                        </a>
                    </div>
                    <div
                        class="fixed inset-x-0 top-0 z-50 flex h-14 items-center justify-between gap-12 px-4 transition sm:px-6 lg:left-72 lg:z-30 lg:px-8 xl:left-80 backdrop-blur-sm lg:left-72 xl:left-80 dark:backdrop-blur bg-white/[var(--bg-opacity-light)] dark:bg-zinc-900/[var(--bg-opacity-dark)]"
                        style="--bg-opacity-light: 0.5; --bg-opacity-dark: 0.2;"
                    >
                        <div class="absolute inset-x-0 top-full h-px transition bg-zinc-900/10 dark:bg-white/10"></div>
                        <div class=""></div>
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
                        </div>
                    </div>

                    <nav class="hidden lg:mt-10 lg:block">
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
