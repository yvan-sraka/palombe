<!-- cargo-sync-readme start -->

# 🕊️ Palombe [![cargo version](https://img.shields.io/crates/v/palombe.svg)](https://crates.io/crates/palombe)

Palombe lets you send and receive messages synchronously through different processes using named pipes.

## Quick example

```rust
extern create palombe;
use std::ffi::CString;

fn main() {
    let key = CString::new("foo").unwrap();
    let value = CString::new("bar").unwrap();
    let key_ = key.clone();
    let value_ = value.clone();
    std::thread::spawn(move || palombe.send(&key_, &value_));
    assert_eq!(palombe.receive(&key), value);
}
```

<!-- cargo-sync-readme end -->