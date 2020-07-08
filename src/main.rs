#[link(name="objbridge", kind="static")]
extern {
    fn open_settings_panel();
}

fn main() {
    unsafe {
        open_settings_panel();
    }
}
