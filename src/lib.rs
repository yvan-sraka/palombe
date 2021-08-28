//! # 🕊️ Palombe [![cargo version](https://img.shields.io/crates/v/palombe.svg)](https://crates.io/crates/palombe)
//! 
//! Palombe lets you send and receive key/value messages synchronously through
//! different processes using named pipes.
//! 
//! ## Quick example
//! 
//! ```rust
//! extern create palombe;
//! 
//! fn main() {
//!     std::thread::spawn(|| send("foo", "bar"));
//!     assert_eq!(receive("foo"), "bar");
//! }
//! ```
//! 
//! Acknowledgments
//! ---------------
//! 
//! :warning: This tool is not suited for building software, it is intended to
//! be used only in rapid prototyping and first product development steps!
//! 
//! C-bindings that expose Palombe have no UTF8 support (because it uses
//! `CString` that are FFI-Safe), so `base64` could be a good encoding for
//! sharing complex datatypes ...
//! 
//! If you looking for a better / faster / safer way to share typed (yes
//! you want that) data across different processes, take a look at
//! [GoogleProtocal Buffer](https://developers.google.com/protocol-buffers/) or
//! even better at [Cap’n Proto](https://capnproto.org/) (which is
//! infinitely faster).
//! 
//! Supported environments
//! ----------------------
//! 
//! The tool is embed into modules targeting several environments:
//! 
//! -   ECMAScript: [npm](https://www.npmjs.com/package/palombe) \|
//!     [Yarn](https://yarnpkg.com/fr/package/palombe)
//!     ([Sources](https://github.com/yvan-sraka/palombe-node))
//! -   Python: [PyPI](https://pypi.org/project/palombe/)
//!     ([Sources](https://github.com/yvan-sraka/palombe-python))
//! -   Ruby: [RubyGem.org](https://rubygems.org/gems/palombe)
//!     ([Sources](https://github.com/yvan-sraka/palombe-ruby))
//! -   Rust: [Crates.io](https://crates.io/crates/palombe)
//!     ([Sources](https://github.com/yvan-sraka/palombe-rust))
//! 
//! Contributing
//! ------------
//! 
//! Please read
//! [CONTRIBUTING.md](https://github.com/yvan-sraka/Palombe/blob/master/CONTRIBUTING.md)
//! for details on our code of conduct, and the process for submitting a pull
//! requests to us.
//! 
//! Authors
//! -------
//! 
//! -   [Yvan Sraka](https://github.com/yvan-sraka)
//! 
//! See also the list of
//! [contributors](https://github.com/yvan-sraka/Palombe/graphs/contributors)
//! who participated in this project.
//! 
//! License
//! -------
//! 
//! This project is licensed under the 3rd version of the GPL License - see the
//! [LICENSE](https://github.com/yvan-sraka/Palombe/blob/master/LICENSE)

extern crate libc;
use std::ffi::CString;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

/// Wrapper around `libc::mkfifo` that crate a named pipe in `/tmp/palombe/`
/// 
/// **N.B.** This function has no purpose to be exposed, it's a lib internal.
fn mkfifo(name: &str) -> PathBuf {
    let prefix = Path::new("/tmp/palombe/");
    let path = prefix.join(name);
    std::fs::create_dir_all(prefix)
        .unwrap_or_else(|_| panic!("Error: couldn't create the folder {}", prefix.display()));
    let filename = CString::new(path.to_str().unwrap()).unwrap();
    unsafe {
        libc::mkfifo(filename.as_ptr(), 0o600);
    }
    path
}

/// Send a `value` associated with a `key` to another thread/program launched by the same user
///
/// # Example
///
/// ```rust
/// extern create palombe;
/// 
/// fn main() {
///     palombe.send("foo", "bar");
/// }
/// ```
pub fn send(name: &str, value: &str) {
    let path = mkfifo(&name);
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .open(path)
        .expect("Error: couldn't open the named pipe");
    file.write_all(value.as_bytes())
        .expect("Error: couldn't write the named pipe");
}

/// Receive the `value` associated with a `key` to another thread/program launched by the same user
///
/// # Example
///
/// ```rust
/// extern create palombe;
/// 
/// fn main() {
///     println!("{}", palombe.receive("foo"));
/// }
/// ```
pub fn receive(name: &str) -> String {
    let path = mkfifo(&name);
    let file = std::fs::File::open(path.clone())
        .unwrap_or_else(|_| panic!("Error: couldn't open: {}", path.display()));
    let mut reader = std::io::BufReader::new(file);
    let mut buffer = String::new();
    loop {
        let len = reader
            .read_line(&mut buffer)
            .expect("Error: couldn't read the input file");
        if len == 0 {
            std::fs::remove_file(&path)
                .unwrap_or_else(|_| panic!("Error: couldn't remove the file {}", path.display()));
            return buffer;
        }
    }
}

/// Same as `send` function, but fulfilled for C compatibility
///
/// # Example
///
/// ```rust
/// extern create palombe;
/// use std::ffi::CString;
/// 
/// fn main() {
///     let key = CString::new("foo").unwrap();
///     let value = CString::new("bar").unwrap();
///     palombe.send(&key, &value);
/// }
/// ```
#[no_mangle]
pub extern "C" fn c_send(key: &CString, value: &CString) {
    let path = mkfifo(&key.to_str().unwrap());
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .open(path)
        .expect("Error: couldn't open the named pipe");
    file.write_all(value.as_bytes())
        .expect("Error: couldn't write the named pipe");
}

/// Same as `receive` function, but fulfilled for C compatibility
///
/// # Example
///
/// ```rust
/// extern create palombe;
/// use std::ffi::CString;
/// 
/// fn main() {
///     let key = CString::new("foo").unwrap();
///     let value: CString = palombe.receive(&key);
/// }
/// ```
#[no_mangle]
pub extern "C" fn c_receive(key: &CString) -> CString {
    let path = mkfifo(&key.to_str().unwrap());
    let file = std::fs::File::open(path.clone())
        .unwrap_or_else(|_| panic!("Error: couldn't open: {}", path.display()));
    let mut reader = std::io::BufReader::new(file);
    let mut buffer = String::new();
    loop {
        let len = reader
            .read_line(&mut buffer)
            .expect("Error: couldn't read the input file");
        if len == 0 {
            std::fs::remove_file(&path)
                .unwrap_or_else(|_| panic!("Error: couldn't remove the file {}", path.display()));
            return CString::new(buffer).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string() {
        std::thread::spawn(|| send("foo", "bar"));
        assert_eq!(receive("foo"), "bar");
    }

    #[test]
    fn c_string() {
        let key = CString::new("bip").unwrap();
        let value = CString::new("boop").unwrap();
        let key_ = key.clone();
        let value_ = value.clone();
        std::thread::spawn(move || c_send(&key_, &value_));
        assert_eq!(c_receive(&key), value);
    }
}
