use crate::{
    error_template::{AppError, ErrorTemplate},
    pages::docs::{
        accordion::AccordionDocPage, dialog::DialogDocPage, dropdown_menu::DropdownMenuDocPage,
        getting_started::GettingStartedPage, menubar::MenubarDocPage, tooltip::TooltipDocPage,
        DocsPage,
    },
};

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

pub mod components;
pub mod error_template;
pub mod icons;
pub mod pages;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/biji-ui.css"/>

        // sets the document title
        <Title text="Biji UI"/>

        // content for this welcome page
        <Router>
            <div class="w-full">
                <Routes  fallback={|| {
                    let mut outside_errors = Errors::default();
                    outside_errors.insert_with_default_key(AppError::NotFound);
                    view! { <ErrorTemplate outside_errors/> }.into_view()
                }}>
                    <Route path=path!("/") view={|| view! { <Redirect path="/docs/getting-started"/> }}/>
                    <ParentRoute path=path!("/docs") view={DocsPage}>
                        <Route path=path!("getting-started") view={GettingStartedPage}/>
                        <Route path=path!("accordion") view={AccordionDocPage}/>
                        <Route path=path!("dialog") view={DialogDocPage}/>
                        <Route path=path!("dropdown-menu") view={DropdownMenuDocPage}/>
                        <Route path=path!("menubar") view={MenubarDocPage}/>
                        <Route path=path!("tooltip") view={TooltipDocPage}/>
                    </ParentRoute>
                </Routes>
            </div>
        </Router>
    }
}
