```
rustc main.rs -l ../../native/basic/doubler.o -L .
```

```rust
// main.rs
extern {
    fn doubler(x: u32) -> u32;
}

fn main() {
    unsafe { println!("{}", doubler(1)); }
}

```