use crate::components::menubar::{context::RootContext, menu::Menu as MenuComponent};
use leptos::*;

pub use crate::components::menubar::item::Item;
pub use crate::components::menubar::item::SubMenuItem as SubMenu;
pub use crate::components::menubar::item::SubMenuItemContent as SubMenuContent;
pub use crate::components::menubar::item::SubMenuItemTrigger as SubMenuTrigger;
pub use crate::components::menubar::menu::MenuContent as Content;
pub use crate::components::menubar::menu::MenuTrigger as Trigger;

#[component]
pub fn Menu(
    #[prop(default = false)] disabled: bool,
    #[prop(into, optional)] class: String,
    #[prop(default = false)] allow_loop: bool,
    children: Children,
) -> impl IntoView {
    let ctx = RootContext {
        allow_item_loop: allow_loop,
        ..RootContext::default()
    };

    provide_context(ctx);

    view! {
        <MenuComponent disabled={disabled} class={class}>
            {children()}
        </MenuComponent>
    }
}
