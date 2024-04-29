use std::time::Duration;

use leptos::*;

#[component]
pub fn DialogDocPage() -> impl IntoView {
    use crate::pages::docs::DocPage;

    view! { <DocPage title="Dialog" description="" example={DialogExample}/> }
}

#[component]
pub fn DialogExample() -> impl IntoView {
    use biji_ui::components::dialog;

    view! {
        <dialog::Root>
            <dialog::Trigger class="py-2 px-4 font-bold text-white bg-indigo-600 rounded hover:bg-indigo-500">
                "Confirm payment"
            </dialog::Trigger>
            <dialog::Overlay
                class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"
                show_class="ease-out opacity-100 duration-300"
                hide_class="ease-in opacity-0 duration-200"
                hide_delay={Duration::from_millis(200)}
            ></dialog::Overlay>
            <dialog::Content
                class="fixed left-[50%] top-[50%] z-50 w-full max-w-[94%] translate-x-[-50%] translate-y-[-50%] bg-background p-5 sm:max-w-[490px] md:w-full rounded-lg shadow-xl transition-all"
                show_class="ease-out duration-300 translate-y-0 opacity-100 sm:scale-100"
                hide_class="ease-in duration-200 translate-y-4 opacity-0 sm:translate-y-0 sm:scale-95"
                hide_delay={Duration::from_millis(200)}
            >
                <div>
                    <div class="flex justify-center items-center mx-auto w-12 h-12 bg-green-100 rounded-full">
                        <svg
                            class="w-6 h-6 text-green-600"
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
                            class="text-base font-semibold leading-6 text-gray-900"
                            id="modal-title"
                        >
                            Payment successful
                        </h3>
                        <div class="mt-2">
                            <p class="text-sm text-gray-500">
                                Lorem ipsum dolor sit amet consectetur adipisicing elit. Consequatur amet labore.
                            </p>
                        </div>
                    </div>
                </div>
                <div class="mt-5 sm:mt-6">
                    <dialog::Close class="inline-flex justify-center py-2 px-3 w-full text-sm font-semibold text-white bg-indigo-600 rounded-md shadow-sm hover:bg-indigo-500 focus-visible:outline-2 focus-visible:outline focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
                        "Go back to dashboard"
                    </dialog::Close>
                </div>
            </dialog::Content>
        </dialog::Root>
    }
}
