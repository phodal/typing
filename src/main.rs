use std::os::raw::c_void;
use std::process::Command;

#[link(name = "objbridge", kind = "static")]
extern {
    pub fn initialize(s: *const c_void);
    pub fn register_keypress_callback(
        cb: extern "C" fn(_self: *mut c_void, *const u8, i32, i32, i32),
    );
}

pub struct TypingContext {}

impl TypingContext {
    pub fn new() -> TypingContext {
        TypingContext {}
    }
}

fn main() {
    let res = Command::new("launchctl")
        .args(&["start", "com.phodal.typing"])
        .status();

    unsafe {
        register_keypress_callback(keypress_callback);
        let typing_ctx = Box::new(TypingContext::new());
        let context = &*typing_ctx as *const TypingContext as *const c_void;
        initialize(context);
    }
}

extern "C" fn keypress_callback(
    _self: *mut c_void,
    raw_buffer: *const u8,
    len: i32,
    event_type: i32,
    key_code: i32,
) {
    unsafe {}
}