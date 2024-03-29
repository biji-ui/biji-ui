use leptos::*;
use leptos_router::*;

pub mod dropdown_menu;

#[component]
pub fn DocsPage() -> impl IntoView {
    let location = use_location();

    let components = [("/docs/dropdown-menu", "Dropdown Menu")];

    view! {
        <div class="relative mx-auto flex w-full flex-auto justify-center sm:px-2 max-w-8xl lg:px-8 xl:px-12">
            <div class="hidden lg:relative lg:block lg:flex-none">
                <div class="sticky top-[4.75rem] -ml-0.5 h-[calc(100vh-4.75rem)] w-64 overflow-y-auto overflow-x-hidden py-16 pl-0.5 pr-8 xl:w-72 xl:pr-16">
                    <nav class="text-base lg:text-sm">
                        <ul role="list" class="space-y-9">
                            <li>
                                <h2 class="font-medium text-slate-900 font-display dark:text-white">
                                    "Introduction"
                                </h2>
                                <ul class="mt-2 space-y-2 border-l-2 border-slate-100 lg:mt-4 lg:space-y-4 lg:border-slate-200 dark:border-slate-800">
                                    <li class="relative">
                                        <a
                                            href="/"
                                            class="block w-full pl-3.5 text-slate-500 before:pointer-events-none before:absolute before:-left-1 before:top-1/2 before:h-1.5 before:w-1.5 before:-translate-y-1/2 before:rounded-full before:hidden before:bg-slate-300 dark:text-slate-400 dark:before:bg-slate-700 dark:hover:text-slate-300 hover:text-slate-600 hover:before:block"
                                        >
                                            "Getting started"
                                        </a>
                                    </li>
                                </ul>
                            </li>
                            <li>
                                <h2 class="font-medium text-slate-900 font-display dark:text-white">
                                    "Components"
                                </h2>
                                <ul class="mt-2 space-y-2 border-l-2 border-slate-100 lg:mt-4 lg:space-y-4 lg:border-slate-200 dark:border-slate-800">
                                    {components
                                        .into_iter()
                                        .map(|c| {
                                            view! {
                                                <li class="relative">
                                                    <a
                                                        href={c.0}
                                                        class="block w-full pl-3.5 text-slate-500 before:pointer-events-none before:absolute before:-left-1 before:top-1/2 before:h-1.5 before:w-1.5 before:-translate-y-1/2 before:rounded-full before:hidden before:bg-slate-300 dark:text-slate-400 dark:before:bg-slate-700 dark:hover:text-slate-300 hover:text-slate-600 hover:before:block"
                                                        class:font-medium={move || {
                                                            location.pathname.get() == c.0
                                                        }}
                                                    >

                                                        {c.1}
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
            </div>
            <div class="min-w-0 max-w-2xl flex-auto px-4 py-16 lg:max-w-none lg:pl-8 lg:pr-0 xl:px-16">
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
        <article>
            <h1 class="scroll-m-20 text-5xl font-semibold tracking-[-0.02em]">{title}</h1>
            <p class="mb-11 mt-3 text-balance text-[21px] font-semibold leading-7 tracking-[-0.01em] text-foreground/40">
                {description}
            </p>
            <div class="relative rounded-card border-2 border-muted bg-zinc-50 !ring-transparent dark:bg-neutral-900/50">
                <div class="flex min-h-[443px] w-full items-center justify-center p-12 preview">
                    {example()}
                </div>
            </div>
        </article>
    }
}
