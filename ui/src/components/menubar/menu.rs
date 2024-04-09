use std::collections::HashMap;

use leptos::{
    html::{Button, Div},
    *,
};

use crate::components::menubar::{
    contexts::{MenubarContext, MenubarMenuContext},
    events::MenuEvents,
};

#[component]
pub fn Menu(
    index: usize,
    children: ChildrenFn,
    #[prop(default = false)] disabled: bool,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let menu_ref = create_node_ref::<Div>();
    let menubar_ctx = expect_context::<MenubarContext>();
    let trigger_ref = create_node_ref::<Button>();
    let ctx = MenubarMenuContext {
        index,
        disabled,
        menu_ref,
        close_on_outside_click: menubar_ctx.close_on_outside_click,
        trigger_ref,
        open: create_rw_signal(false),
        in_focus: create_rw_signal(true),
        items: create_rw_signal(HashMap::new()),
        current_focus: create_rw_signal(None),
    };

    menubar_ctx.upsert_item(index, ctx);

    view! {
        <Provider value={ctx}>
            <MenuEvents>
                <div _ref={menu_ref} class={class}>
                    {children()}
                </div>
            </MenuEvents>
        </Provider>
    }
}
