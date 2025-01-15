use std::time::Duration;

use leptos::{portal::Portal, prelude::*};

#[component]
pub fn DialogDocPage() -> impl IntoView {
    use crate::pages::docs::DocPage;

    view! {
        <DocPage
            title="Dialog"
            description="A window overlaid on either the primary window or another dialog window, rendering the content underneath inert."
            example={DialogExample}
        />
    }
}

#[component]
pub fn DialogExample() -> impl IntoView {
    use biji_ui::components::dialog;

    view! {
        <dialog::Root hide_delay={Duration::from_millis(200)}>
            <dialog::Trigger class="rounded bg-indigo-600 px-4 py-2 font-bold text-white hover:bg-indigo-500">
                "Confirm payment"
            </dialog::Trigger>
            <Portal>
                <dialog::Overlay
                    class="fixed inset-0 z-50 bg-zinc-400/20 backdrop-blur-sm transition-opacity duration-300 ease-linear dark:bg-black/40"
                    show_class="opacity-100 duration-300 ease-out"
                    hide_class="opacity-0 duration-200 ease-in"
                ></dialog::Overlay>
                <dialog::Content
                    class="z-50 fixed top-[calc(100%-1rem)] left-[50%] md:top-[50%] z-50 w-full max-w-[94%] translate-x-[-50%] translate-y-[-100%] md:translate-y-[-50%] bg-background p-5 sm:max-w-[490px] md:w-full rounded-lg shadow-xl transition-all"
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
                                class="text-base font-semibold leading-6 text-gray-900 dark:text-gray-100"
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
