use std::collections::HashMap;

pub trait FilterActiveItems<T> {
    fn filter_active_items(&self) -> Vec<T>;
}

pub trait NavigateItems<T> {
    fn navigate_first_item(&self) -> Option<T>;
    fn navigate_last_item(&self) -> Option<T>;
    fn navigate_next_item(&self) -> Option<T>;
    fn navigate_previous_item(&self) -> Option<T>;
}

pub trait Toggle {
    fn toggle(&self);

    fn open(&self);

    fn close(&self);
}

pub trait Focus {
    /// Focus on element
    fn focus(&self) -> bool;
}

pub trait GetIndex<T> {
    fn get_index(&self) -> T;
}

pub trait ManageFocus {
    fn set_focus(&self, index: Option<usize>);

    /// Check if item is in focus
    fn item_in_focus(&self, index: usize) -> bool;
}

pub trait IsActive {
    fn is_active(&self) -> bool;
}

pub fn filter_active<T>(items: HashMap<usize, T>) -> Vec<T>
where
    T: GetIndex<usize> + IsActive + Clone,
{
    let mut items = items
        .values()
        .filter(|item| item.is_active())
        .cloned()
        .collect::<Vec<T>>();

    items.sort_by(|a, b| a.get_index().cmp(&b.get_index()));

    items
}

/// Get next item in items list. If current_focus is None, return first item if `allow_loop` is `true`.
pub fn next_item<T>(items: Vec<T>, current_focus: Option<usize>, allow_loop: bool) -> Option<T>
where
    T: GetIndex<usize> + Clone,
{
    let Some(item_focus) = current_focus else {
        if let Some(first) = items.get(0) {
            return Some(first.clone());
        }
        return None;
    };

    let Some(current_pos) = items.iter().position(|item| item.get_index() == item_focus) else {
        return None;
    };

    items.get(current_pos + 1).cloned().or_else(|| {
        if allow_loop {
            items.get(0).cloned()
        } else {
            None
        }
    })
}

/// Get previous item in items list. If current_focus is None, return last item.
pub fn previous_item<T>(items: Vec<T>, current_focus: Option<usize>, allow_loop: bool) -> Option<T>
where
    T: GetIndex<usize> + Clone,
{
    let Some(item_focus) = current_focus else {
        if let Some(last) = items.last() {
            return Some(last.clone());
        }
        return None;
    };

    let Some(current_pos) = items.iter().position(|item| item.get_index() == item_focus) else {
        return None;
    };

    if current_pos == 0 {
        if allow_loop {
            return items.last().cloned();
        }
        return None;
    }

    items.get(current_pos - 1).cloned()
}
