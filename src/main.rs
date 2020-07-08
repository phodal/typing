use std::os::raw::{c_void, c_char};
use std::process::{Command, exit};
use log::{error, info, warn};
use std::ffi::CStr;

#[link(name = "objbridge", kind = "static")]
extern {
    pub fn initialize(s: *const c_void);
    pub fn eventloop();
    pub fn prompt_accessibility() -> i32;
    pub fn check_accessibility() -> i32;
    pub fn get_secure_input_process(pid: *mut i64) -> i32;
    pub fn register_keypress_callback(
        cb: extern "C" fn(_self: *mut c_void, *const u8, i32, i32, i32),
    );
}

pub struct TypingContext {

}

impl TypingContext {
    pub fn new() -> TypingContext {
        unsafe {
            let has_accessibility = check_accessibility();
            println!("has_accessibility: {}", has_accessibility == 0);
            if has_accessibility == 0 {
                let res = prompt_accessibility();

                if res == 0 {
                    error!("Accessibility");
                    exit(1);
                }
            }
        }

        unsafe {
            register_keypress_callback(keypress_callback);
            let typing_ctx = Box::new(TypingContext{

            });
            let context = &*typing_ctx as *const TypingContext as *const c_void;
            initialize(context);
        }


        TypingContext {

        }
    }

    fn start_secure_input_watcher(&self) {
        let mut pid: i64 = -1;
        unsafe {
            get_secure_input_process(&mut pid as *mut i64)
        };
    }

    fn eventloop(&self) {
        self.start_secure_input_watcher();

        unsafe {
            eventloop();
        }
    }
}

#[cfg(target_os = "macos")]
const MAC_PLIST_CONTENT: &str = include_str!("res/mac/com.phodal.typing.plist");
#[cfg(target_os = "macos")]
const MAC_PLIST_FILENAME: &str = "com.phodal.typing.plist";

fn register() {
    use std::fs::create_dir_all;
    use std::process::{Command, ExitStatus};

    let home_dir = dirs::home_dir().expect("Could not get user home directory");
    let library_dir = home_dir.join("Library");
    let agents_dir = library_dir.join("LaunchAgents");

    if !agents_dir.exists() {
        create_dir_all(agents_dir.clone()).expect("Could not create LaunchAgents directory");
    }

    let plist_file = agents_dir.join(MAC_PLIST_FILENAME);
    if !plist_file.exists() {
        let cmd_path = std::env::current_exe().expect("Could not get espanso executable path");
        let plist_content = String::from(MAC_PLIST_CONTENT).replace(
            "{{{typing_path}}}",
            cmd_path.to_str().unwrap_or_default(),
        );

        let user_path = std::env::var("PATH").unwrap_or("".to_owned());
        let plist_content = plist_content.replace("{{{PATH}}}", &user_path);

        std::fs::write(plist_file.clone(), plist_content).expect("Unable to write plist file");

        println!("Entry created correctly!");
    }

    let res = Command::new("launchctl")
        .args(&["load", "-w", plist_file.to_str().unwrap_or_default()])
        .status();

    if let Ok(status) = res {
        if status.success() {
            println!("Entry loaded correctly!")
        }
    } else {
        println!("Error loading new entry");
    }
}

fn main() {
    register();

    let res = Command::new("launchctl")
        .args(&["start", "com.phodal.typing"])
        .status();

    if let Ok(status) = res {
        if status.success() {
            println!("Daemon started correctly!")
        } else {
            eprintln!("Error starting launchd daemon with status: {}", status);
        }
    } else {
        eprintln!("Error starting launchd daemon: {}", res.unwrap_err());
    }

    unsafe {
        let typing = TypingContext::new();
        typing.eventloop();
    }
}

extern "C" fn keypress_callback(
    _self: *mut c_void,
    raw_buffer: *const u8,
    len: i32,
    event_type: i32,
    key_code: i32,
) {
    unsafe {
        let c_str = CStr::from_ptr(raw_buffer as (*const c_char));
        let char_str = c_str.to_str();

        // Send the char through the channel
        match char_str {
            Ok(char_str) => {
                print!("{}", char_str);
            }
            Err(e) => {
                error!("Unable to receive char: {}", e);
            }
        }
    }
}