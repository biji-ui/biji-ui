use leptos::{html::Div, *};

use crate::components::menubar::{
    contexts::{MenubarMenuContext, MenubarMenuItemContext},
    events::ItemEvents,
};

#[component]
pub fn Item(
    index: usize,
    children: Children,
    #[prop(default = false)] disabled: bool,
    #[prop(into, optional)] class: String,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let menu_ctx = expect_context::<MenubarMenuContext>();

    let item_ref = create_node_ref::<Div>();

    let ctx = MenubarMenuItemContext {
        index: index.clone(),
        disabled,
        trigger_ref: item_ref.clone(),
    };

    menu_ctx.upsert_item(index, ctx);

    view! {
        <Provider value={ctx}>
            <ItemEvents>
                <div
                    {..attrs}
                    _ref={item_ref}
                    class={class}
                    data-state={ctx.index}
                    data-disabled={disabled}
                    data-highlighted={move || {
                        menu_ctx.current_focus.get().map(|f| f == index).unwrap_or(false)
                    }}

                    tabindex=0
                >

                    {children()}
                </div>
            </ItemEvents>
        </Provider>
    }
}
