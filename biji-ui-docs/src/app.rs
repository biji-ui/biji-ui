use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

use crate::pages::docs::{
    DocsPage, accordion::AccordionDocPage, dialog::DialogDocPage,
    dropdown_menu::DropdownMenuDocPage, getting_started::GettingStartedPage,
    menubar::MenubarDocPage, tooltip::TooltipDocPage,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Biji UI" />

        <Router>
            <div class="w-full">
                <Routes fallback={|| "Not found."}>
                    <Route
                        path={path!("/")}
                        view={|| view! { <Redirect path="/docs/getting-started" /> }}
                    />
                    <ParentRoute path={path!("/docs")} view={DocsPage}>
                        <Route path={path!("getting-started")} view={GettingStartedPage} />
                        <Route path={path!("accordion")} view={AccordionDocPage} />
                        <Route path={path!("dialog")} view={DialogDocPage} />
                        <Route path={path!("dropdown-menu")} view={DropdownMenuDocPage} />
                        <Route path={path!("menubar")} view={MenubarDocPage} />
                        <Route path={path!("tooltip")} view={TooltipDocPage} />
                    </ParentRoute>
                </Routes>
            </div>
        </Router>
    }
}
