use std::collections::HashMap;

use leptos::{
    html::{Button, Div},
    *,
};

use crate::components::menu::{
    contexts::{MenuContext, MenuItemContext},
    events::SubRootEvents,
};

#[component]
pub fn SubRoot(
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

    let sub_root_ctx = MenuContext {
        open: create_rw_signal(false),
        in_focus: create_rw_signal(false),
        close_on_outside_click: root_ctx.close_on_outside_click,
        menu_ref: item_ref,
        trigger_ref: create_node_ref::<Button>(),
        items: create_rw_signal(HashMap::new()),
        current_focus: create_rw_signal(None),
    };

    view! {
        <Provider value={ctx}>
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
                <Provider value={sub_root_ctx}>
                    <SubRootEvents parent_index={index} parent_ctx={root_ctx}>
                        {children()}
                    </SubRootEvents>
                </Provider>
            </div>
        </Provider>
    }
}
