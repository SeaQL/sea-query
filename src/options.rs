use std::sync::atomic::{AtomicBool, Ordering};

static PREFER_MORE_PARENTHESES: AtomicBool = AtomicBool::new(false);
static PREFER_MORE_PARENTHESES_SET: AtomicBool = AtomicBool::new(false);

/// ### Panics
///
/// This can only be set once during process startup. Setting twice would result in panic.
pub fn set_prefer_more_parentheses(v: bool) {
    if PREFER_MORE_PARENTHESES_SET.load(Ordering::SeqCst) {
        panic!("Can't set the same global option `PREFER_MORE_PARENTHESES` twice");
    }
    PREFER_MORE_PARENTHESES_SET.store(true, Ordering::SeqCst);
    PREFER_MORE_PARENTHESES.store(v, Ordering::SeqCst);
}

#[inline]
pub fn get_prefer_more_parentheses() -> bool {
    PREFER_MORE_PARENTHESES.load(Ordering::SeqCst)
}
