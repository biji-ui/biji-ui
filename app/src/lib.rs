use crate::{
    error_template::{AppError, ErrorTemplate},
    pages::docs::{
        accordion::AccordionDocPage, dialog::DialogDocPage, dropdown_menu::DropdownMenuDocPage,
        getting_started::GettingStartedPage, menubar::MenubarDocPage, DocsPage,
    },
};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod components;
pub mod error_template;
pub mod icons;
pub mod pages;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/biji-ui.css"/>

        // sets the document title
        <Title text="Biji UI"/>

        // content for this welcome page
        <Router fallback={|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }}>
            <div class="w-full">
                <Routes>
                    <Route path="/" view={|| view! { <Redirect path="/docs/getting-started"/> }}/>
                    <Route path="/docs/" view={DocsPage}>
                        <Route path="getting-started" view={GettingStartedPage}/>
                        <Route path="accordion" view={AccordionDocPage}/>
                        <Route path="dialog" view={DialogDocPage}/>
                        <Route path="dropdown-menu" view={DropdownMenuDocPage}/>
                        <Route path="menubar" view={MenubarDocPage}/>
                    </Route>
                </Routes>
            </div>
        </Router>
    }
}
