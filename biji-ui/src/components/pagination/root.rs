use leptos::{context::Provider, prelude::*};

/// Reactive state for a pagination control. Available via [`use_pagination`] or the `let:` binding on [`RootWith`].
///
/// All fields are `Copy`, so it is safe to pass this struct to child components as a prop.
#[derive(Copy, Clone)]
pub struct PaginationState {
    /// Current 1-indexed page number.
    pub page: Signal<i32>,
    /// Number of items per page.
    pub page_size: i32,
    /// Total number of matching items.
    pub total: Signal<i32>,
    /// Total number of pages (derived from `total` and `page_size`).
    pub total_pages: Signal<i32>,
    /// Whether a next page exists.
    pub has_next: Signal<bool>,
    /// Whether a previous page exists.
    pub has_prev: Signal<bool>,
    /// 0-based offset of the first item on the current page.
    pub offset: Signal<i32>,
}

impl PaginationState {
    fn new(page: Signal<i32>, page_size: i32, total: Signal<i32>) -> Self {
        let total_pages = Signal::derive(move || {
            let t = total.get();
            if page_size <= 0 || t <= 0 {
                0
            } else {
                (t + page_size - 1) / page_size
            }
        });
        let has_next = Signal::derive(move || page.get() < total_pages.get());
        let has_prev = Signal::derive(move || page.get() > 1);
        let offset = Signal::derive(move || (page.get() - 1).max(0) * page_size);
        Self {
            page,
            page_size,
            total,
            total_pages,
            has_next,
            has_prev,
            offset,
        }
    }
}

/// Returns the [`PaginationState`] from the nearest [`Root`] or [`RootWith`] ancestor.
///
/// Call this inside any child component that needs access to pagination state.
pub fn use_pagination() -> PaginationState {
    expect_context::<PaginationState>()
}

/// The render-prop variant of [`Root`]. Use this when you need access to [`PaginationState`]
/// directly inside the children via the `let:` binding.
///
/// ```rust
/// <pagination::RootWith page=page page_size=20 total=total let:p>
///     <button disabled=move || !p.has_prev.get()>"Previous"</button>
///     <span>{move || format!("Page {} of {}", p.page.get(), p.total_pages.get())}</span>
///     <button disabled=move || !p.has_next.get()>"Next"</button>
/// </pagination::RootWith>
/// ```
#[component]
pub fn RootWith<IV: IntoView + 'static>(
    children: impl Fn(PaginationState) -> IV + Send + Sync + 'static,
    #[prop(into, optional)] class: String,
    #[prop(into)] page: Signal<i32>,
    page_size: i32,
    #[prop(into)] total: Signal<i32>,
) -> impl IntoView {
    let state = PaginationState::new(page, page_size, total);

    view! {
        <Provider value={state}>
            <nav
                role="navigation"
                aria-label="Pagination"
                data-page={move || state.page.get().to_string()}
                data-total-pages={move || state.total_pages.get().to_string()}
                class={class}
            >
                {children(state)}
            </nav>
        </Provider>
    }
}

/// The standard pagination root. Renders a `<nav role="navigation">` and provides
/// [`PaginationState`] to all descendants via context.
///
/// Use [`RootWith`] instead when you need to access [`PaginationState`] inline via `let:p`.
#[component]
pub fn Root(
    children: ChildrenFn,
    #[prop(into, optional)] class: String,
    #[prop(into)] page: Signal<i32>,
    page_size: i32,
    #[prop(into)] total: Signal<i32>,
) -> impl IntoView {
    view! {
        <RootWith page=page page_size=page_size total=total class=class let:_>
            {children()}
        </RootWith>
    }
}
