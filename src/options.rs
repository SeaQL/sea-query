static mut GLOBAL_OPTIONS: GlobalOptions = GlobalOptions {
    prefer_more_parentheses: false,
    prefer_more_parentheses_set: false,
};

#[derive(Default)]
struct GlobalOptions {
    prefer_more_parentheses: bool,
    prefer_more_parentheses_set: bool,
}

/// Safety: this can only be set once during process startup.
///
/// ### Panics
///
/// Setting twice would result in panic.
pub fn set_prefer_more_parentheses(v: bool) {
    unsafe {
        if GLOBAL_OPTIONS.prefer_more_parentheses_set {
            panic!("Can't set the same global option `prefer_more_parentheses` twice");
        }
        GLOBAL_OPTIONS.prefer_more_parentheses = v;
        GLOBAL_OPTIONS.prefer_more_parentheses_set = true;
    }
}

#[inline]
pub fn get_prefer_more_parentheses() -> bool {
    unsafe { GLOBAL_OPTIONS.prefer_more_parentheses }
}
