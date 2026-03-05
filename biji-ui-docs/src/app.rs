use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone() />
                <MetaTags/>
                <Stylesheet id="leptos" href="/pkg/biji-ui-docs.css"/>
            </head>
            <body class="bg-background text-foreground">
                <App/>
            </body>
        </html>
    }
}

use crate::pages::docs::{
    DocsPage, accordion::AccordionDocPage, calendar::CalendarDocPage,
    checkbox::CheckboxDocPage, collapsible::CollapsibleDocPage, dialog::DialogDocPage,
    dropdown_menu::DropdownMenuDocPage, getting_started::GettingStartedPage,
    menubar::MenubarDocPage, progress::ProgressDocPage, radio_group::RadioGroupDocPage,
    separator::SeparatorDocPage, slider::SliderDocPage, switch::SwitchDocPage,
    tooltip::TooltipDocPage,
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
                        <Route path={path!("calendar")} view={CalendarDocPage} />
                        <Route path={path!("checkbox")} view={CheckboxDocPage} />
                        <Route path={path!("collapsible")} view={CollapsibleDocPage} />
                        <Route path={path!("dialog")} view={DialogDocPage} />
                        <Route path={path!("dropdown-menu")} view={DropdownMenuDocPage} />
                        <Route path={path!("menubar")} view={MenubarDocPage} />
                        <Route path={path!("progress")} view={ProgressDocPage} />
                        <Route path={path!("radio-group")} view={RadioGroupDocPage} />
                        <Route path={path!("separator")} view={SeparatorDocPage} />
                        <Route path={path!("slider")} view={SliderDocPage} />
                        <Route path={path!("switch")} view={SwitchDocPage} />
                        <Route path={path!("tooltip")} view={TooltipDocPage} />
                    </ParentRoute>
                </Routes>
            </div>
        </Router>
    }
}
