// main.rs
extern {
    fn doubler(x: u32) -> u32;
}

fn main() {
    unsafe { println!("{}", doubler(1)); }
}
