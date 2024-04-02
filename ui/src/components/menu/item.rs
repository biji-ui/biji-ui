use leptos::{html::Div, *};

use crate::components::menu::events::ItemEvents;

use super::root::MenuContext;

#[derive(Copy, Clone)]
pub struct MenuItemContext {
    pub index: usize,
    pub disabled: bool,
    pub trigger_ref: NodeRef<Div>,
}

#[component]
pub fn Item(
    index: usize,
    children: Children,
    #[prop(default = false)] disabled: bool,
    #[prop(into, optional)] class: String,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let root_ctx = expect_context::<MenuContext>();

    let item_ref = create_node_ref::<Div>();

    let ctx = MenuItemContext {
        index: index.clone(),
        disabled,
        trigger_ref: item_ref.clone(),
    };

    root_ctx.items.update(|items| {
        *items.entry(index).or_insert(ctx.clone()) = ctx.clone();
    });

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
                        root_ctx.current_focus.get().map(|f| f == index).unwrap_or(false)
                    }}

                    tabindex=0
                >

                    {children()}
                </div>
            </ItemEvents>
        </Provider>
    }
    .into_view()
}
