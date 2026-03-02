//! Traits and helpers for navigating, focusing, and toggling collections of UI items.
//!
//! These abstractions power the keyboard navigation logic shared across accordion,
//! menu, and menubar components.

use std::collections::HashMap;

/// Filter a collection down to its *active* (non-disabled) items.
pub trait FilterActiveItems<T> {
    /// Return a `Vec` of active items sorted by index.
    fn filter_active_items(&self) -> Vec<T>;
}

/// Keyboard-style sequential navigation over an ordered list of items.
pub trait NavigateItems<T> {
    /// Return the first active item.
    fn navigate_first_item(&self) -> Option<T>;
    /// Return the last active item.
    fn navigate_last_item(&self) -> Option<T>;
    /// Return the item after the currently focused one, optionally wrapping around.
    fn navigate_next_item(&self) -> Option<T>;
    /// Return the item before the currently focused one, optionally wrapping around.
    fn navigate_previous_item(&self) -> Option<T>;
}

/// Open / close toggle behaviour for an item or panel.
pub trait Toggle {
    /// Flip between open and closed.
    fn toggle(&self);
    /// Set the state to open.
    fn open(&self);
    /// Set the state to closed.
    fn close(&self);
}

/// Move browser focus to the element backing this item.
pub trait Focus {
    /// Attempt to focus the element. Returns `true` if the element was found and focused.
    fn focus(&self) -> bool;
}

/// Retrieve the positional index of an item.
pub trait GetIndex<T> {
    /// Return the item's index.
    fn get_index(&self) -> T;
}

/// Track which item in a list currently has focus.
pub trait ManageFocus {
    /// Set the currently focused index, or `None` to clear focus.
    fn set_focus(&self, index: Option<usize>);

    /// Check whether the item at `index` is the currently focused one.
    fn item_in_focus(&self, index: usize) -> bool;
}

/// Whether an item should participate in navigation (i.e. is not disabled).
pub trait IsActive {
    /// Return `true` if the item is active / enabled.
    fn is_active(&self) -> bool;
}

/// Collect all active items from a `HashMap`, sorted by index.
///
/// Disabled items (where [`IsActive::is_active`] returns `false`) are excluded.
pub fn filter_active<T>(items: HashMap<usize, T>) -> Vec<T>
where
    T: GetIndex<usize> + IsActive + Copy,
{
    let mut items = items
        .values()
        .filter(|item| item.is_active())
        .copied()
        .collect::<Vec<T>>();

    items.sort_by(|a, b| a.get_index().cmp(&b.get_index()));

    items
}

/// Return the next item after the currently focused one.
///
/// * If `current_focus` is `None`, the first item is returned.
/// * If the current focus is the last item and `allow_loop` is `true`, wraps to the first item.
/// * Returns `None` when navigation is not possible.
pub fn next_item<T>(items: Vec<T>, current_focus: Option<usize>, allow_loop: bool) -> Option<T>
where
    T: GetIndex<usize> + Copy,
{
    let Some(item_focus) = current_focus else {
        return items.first().copied();
    };

    let Some(current_pos) = items.iter().position(|item| item.get_index() == item_focus) else {
        return None;
    };

    items.get(current_pos + 1).copied().or_else(|| {
        if allow_loop {
            items.first().copied()
        } else {
            None
        }
    })
}

/// Return the previous item before the currently focused one.
///
/// * If `current_focus` is `None`, the last item is returned.
/// * If the current focus is the first item and `allow_loop` is `true`, wraps to the last item.
/// * Returns `None` when navigation is not possible.
pub fn previous_item<T>(items: Vec<T>, current_focus: Option<usize>, allow_loop: bool) -> Option<T>
where
    T: GetIndex<usize> + Copy,
{
    let Some(item_focus) = current_focus else {
        return items.last().copied();
    };

    let Some(current_pos) = items.iter().position(|item| item.get_index() == item_focus) else {
        return None;
    };

    if current_pos == 0 {
        if allow_loop {
            return items.last().copied();
        }
        return None;
    }

    items.get(current_pos - 1).copied()
}
