/// Singleton registry that ensures at most one tooltip is open at a time.
///
/// WASM is single-threaded, so `thread_local!` storage is safe and requires no
/// locking. The pattern mirrors what Radix UI's `TooltipProvider` and melt-ui's
/// `createTooltipGroup()` do at the framework level, but transparently — callers
/// do not need to add any provider to their tree.
///
/// Lifecycle:
/// 1. `Root` calls `register(id, close_fn)` on mount and `unregister(id)` on cleanup.
/// 2. `TooltipContext::open()` calls `activate(id)` before opening, which immediately
///    closes the previously active tooltip (if any).
/// 3. `TooltipContext::close()` calls `deactivate(id)` to clear the active slot.
use std::cell::{Cell, RefCell};
use std::collections::HashMap;

thread_local! {
    /// The numeric ID of the currently-open tooltip, or `None`.
    static ACTIVE: Cell<Option<usize>> = Cell::new(None);
    /// Maps numeric tooltip ID → a closure that closes that tooltip immediately.
    static REGISTRY: RefCell<HashMap<usize, Box<dyn Fn()>>> =
        RefCell::new(HashMap::new());
}

/// Register a close callback for tooltip `id`. Called from `Root` on mount.
pub fn register(id: usize, close_fn: impl Fn() + 'static) {
    REGISTRY.with(|r| r.borrow_mut().insert(id, Box::new(close_fn)));
}

/// Unregister tooltip `id`. Called from `Root` on cleanup.
pub fn unregister(id: usize) {
    REGISTRY.with(|r| r.borrow_mut().remove(&id));
    ACTIVE.with(|a| {
        if a.get() == Some(id) {
            a.set(None);
        }
    });
}

/// Called when tooltip `id` is about to open.
/// Immediately closes any other tooltip that is currently open.
pub fn activate(id: usize) {
    let prev = ACTIVE.with(|a| a.replace(Some(id)));
    if let Some(prev_id) = prev {
        if prev_id != id {
            REGISTRY.with(|r| {
                if let Some(f) = r.borrow().get(&prev_id) {
                    f();
                }
            });
        }
    }
}

/// Called when tooltip `id` closes. Clears the active slot so the next
/// tooltip that opens does not try to close an already-gone tooltip.
pub fn deactivate(id: usize) {
    ACTIVE.with(|a| {
        if a.get() == Some(id) {
            a.set(None);
        }
    });
}
