#[macro_use]
extern crate lazy_static;

use std::ffi::CStr;
use std::os::raw::{c_char, c_void};
use std::process::{exit, Command};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Acquire;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};

use crate::ime::event::{Event, KeyEvent, KeyModifier};
use crate::ime::manager::{DefaultEventManager, EventManager};
use log::{debug, error, info};
use std::thread;

mod ime;

#[link(name = "objbridge", kind = "static")]
extern "C" {
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
    pub send_channel: Sender<Event>,
    is_injecting: Arc<AtomicBool>,
}

impl TypingContext {
    pub fn new(send_channel: Sender<Event>, is_injecting: Arc<AtomicBool>) -> Box<TypingContext> {
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

        let context = Box::new(TypingContext {
            is_injecting,
            send_channel,
        });

        unsafe {
            let context_ptr = &*context as *const TypingContext as *const c_void;

            register_keypress_callback(keypress_callback);

            initialize(context_ptr);
        }

        context
    }

    fn start_secure_input_watcher(&self) {
        let mut pid: i64 = -1;
        unsafe { get_secure_input_process(&mut pid as *mut i64) };
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
    use std::process::Command;

    let home_dir = dirs::home_dir().expect("Could not get user home directory");
    let library_dir = home_dir.join("Library");
    let agents_dir = library_dir.join("LaunchAgents");

    if !agents_dir.exists() {
        create_dir_all(agents_dir.clone()).expect("Could not create LaunchAgents directory");
    }

    let plist_file = agents_dir.join(MAC_PLIST_FILENAME);
    if !plist_file.exists() {
        let cmd_path = std::env::current_exe().expect("Could not get espanso executable path");
        let plist_content = String::from(MAC_PLIST_CONTENT)
            .replace("{{{typing_path}}}", cmd_path.to_str().unwrap_or_default());

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
        let (send_channel, receive_channel) = mpsc::channel();
        let is_injecting = Arc::new(std::sync::atomic::AtomicBool::new(false));

        let clone_injecting = is_injecting.clone();
        thread::Builder::new()
            .name("daemon_background".to_string())
            .spawn(move || {
                worker_background(receive_channel, clone_injecting);
            })
            .expect("Unable to spawn daemon background thread");

        let typing = TypingContext::new(send_channel, is_injecting);
        typing.eventloop();
    }
}

fn worker_background(receive_channel: Receiver<Event>, is_injecting: Arc<AtomicBool>) {
    let event_manager = DefaultEventManager::new(receive_channel);

    info!("worker is running!");
    event_manager.eventloop();
}

extern "C" fn keypress_callback(
    _self: *mut c_void,
    raw_buffer: *const u8,
    len: i32,
    event_type: i32,
    key_code: i32,
) {
    unsafe {
        let _self = _self as *mut TypingContext;

        if (*_self).is_injecting.load(Acquire) {
            debug!("Input ignored while espanso is injecting text...");
            return;
        }

        if event_type == 0 {
            let c_str = CStr::from_ptr(raw_buffer as (*const c_char));
            let char_str = c_str.to_str();

            match char_str {
                Ok(char_str) => {
                    let event = Event::Key(KeyEvent::Char(char_str.to_owned()));
                    (*_self).send_channel.send(event).unwrap();
                }
                Err(e) => {
                    error!("Unable to receive char: {}", e);
                }
            }
        } else if event_type == 1 {
            // Modifier event
            let modifier: Option<KeyModifier> = match key_code {
                0x37 => Some(KeyModifier::LEFT_META),
                0x36 => Some(KeyModifier::RIGHT_META),
                0x38 => Some(KeyModifier::LEFT_SHIFT),
                0x3C => Some(KeyModifier::RIGHT_SHIFT),
                0x3A => Some(KeyModifier::LEFT_ALT),
                0x3D => Some(KeyModifier::RIGHT_ALT),
                0x3B => Some(KeyModifier::LEFT_CTRL),
                0x3E => Some(KeyModifier::RIGHT_CTRL),
                0x33 => Some(KeyModifier::BACKSPACE),
                0x39 => Some(KeyModifier::CAPS_LOCK),
                _ => None,
            };

            if let Some(modifier) = modifier {
                let event = Event::Key(KeyEvent::Modifier(modifier));
                (*_self).send_channel.send(event).unwrap();
            } else {
                // Not one of the default modifiers, send an "other" event
                let event = Event::Key(KeyEvent::Other);
                (*_self).send_channel.send(event).unwrap();
            }
        } else {
            // Other type of event
            let event = Event::Key(KeyEvent::Other);
            (*_self).send_channel.send(event).unwrap();
        }
    }
}
