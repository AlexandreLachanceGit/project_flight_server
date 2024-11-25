use std::sync::atomic::{AtomicUsize, Ordering};

static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Generate new client player ID
pub fn new_id() -> usize {
    ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}
