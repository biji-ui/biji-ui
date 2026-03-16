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
                <link rel="icon" href="/logo.svg" type="image/svg+xml"/>
            </head>
            <body class="bg-background text-foreground">
                <App/>
            </body>
        </html>
    }
}

use crate::pages::docs::{
    DocsPage, accordion::AccordionDocPage, alert_dialog::AlertDialogDocPage,
    calendar::CalendarDocPage, checkbox::CheckboxDocPage, collapsible::CollapsibleDocPage,
    command::CommandDocPage, combobox::ComboboxDocPage, context_menu::ContextMenuDocPage,
    dialog::DialogDocPage, drawer::DrawerDocPage, dropdown_menu::DropdownMenuDocPage,
    getting_started::GettingStartedPage, hover_card::HoverCardDocPage, menubar::MenubarDocPage,
    navigation_menu::NavigationMenuDocPage,
    pin_input::PinInputDocPage, popover::PopoverDocPage, progress::ProgressDocPage,
    radio_group::RadioGroupDocPage, select::SelectDocPage, separator::SeparatorDocPage,
    slider::SliderDocPage, switch::SwitchDocPage, tabs::TabsDocPage, toast::ToastDocPage,
    toggle_group::ToggleGroupDocPage, tooltip::TooltipDocPage,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Biji UI — Headless UI Components for Leptos" />
        <Meta name="description" content="Biji UI is a headless, accessible UI component library for Leptos. Unstyled primitives that work with Tailwind CSS or any CSS framework." />
        <Meta name="robots" content="index, follow" />
        <Meta property="og:type" content="website" />
        <Meta property="og:site_name" content="Biji UI" />
        <Meta property="og:title" content="Biji UI — Headless UI Components for Leptos" />
        <Meta property="og:description" content="Biji UI is a headless, accessible UI component library for Leptos. Unstyled primitives that work with Tailwind CSS or any CSS framework." />
        <Meta property="og:image" content="https://raw.githubusercontent.com/biji-ui/biji-ui/main/biji-ui-docs/public/full-logo.svg" />
        <Meta name="twitter:card" content="summary" />
        <Meta name="twitter:title" content="Biji UI — Headless UI Components for Leptos" />
        <Meta name="twitter:description" content="Biji UI is a headless, accessible UI component library for Leptos. Unstyled primitives that work with Tailwind CSS or any CSS framework." />

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
                        <Route path={path!("alert-dialog")} view={AlertDialogDocPage} />
                        <Route path={path!("calendar")} view={CalendarDocPage} />
                        <Route path={path!("checkbox")} view={CheckboxDocPage} />
                        <Route path={path!("collapsible")} view={CollapsibleDocPage} />
                        <Route path={path!("command")} view={CommandDocPage} />
                        <Route path={path!("combobox")} view={ComboboxDocPage} />
                        <Route path={path!("context-menu")} view={ContextMenuDocPage} />
                        <Route path={path!("dialog")} view={DialogDocPage} />
                        <Route path={path!("drawer")} view={DrawerDocPage} />
                        <Route path={path!("dropdown-menu")} view={DropdownMenuDocPage} />
                        <Route path={path!("hover-card")} view={HoverCardDocPage} />
                        <Route path={path!("menubar")} view={MenubarDocPage} />
                        <Route path={path!("navigation-menu")} view={NavigationMenuDocPage} />
                        <Route path={path!("pin-input")} view={PinInputDocPage} />
                        <Route path={path!("popover")} view={PopoverDocPage} />
                        <Route path={path!("progress")} view={ProgressDocPage} />
                        <Route path={path!("radio-group")} view={RadioGroupDocPage} />
                        <Route path={path!("select")} view={SelectDocPage} />
                        <Route path={path!("separator")} view={SeparatorDocPage} />
                        <Route path={path!("slider")} view={SliderDocPage} />
                        <Route path={path!("switch")} view={SwitchDocPage} />
                        <Route path={path!("tabs")} view={TabsDocPage} />
                        <Route path={path!("toast")} view={ToastDocPage} />
                        <Route path={path!("toggle-group")} view={ToggleGroupDocPage} />
                        <Route path={path!("tooltip")} view={TooltipDocPage} />
                    </ParentRoute>
                </Routes>
            </div>
        </Router>
    }
}
