extern crate libc;
use std::ffi::{CStr, CString};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

fn __mkfifo(name: &CStr) -> PathBuf {
    let prefix = Path::new("/tmp/palombe/");
    let path = prefix.join(name.to_str().unwrap());
    std::fs::create_dir_all(prefix)
        .unwrap_or_else(|_| panic!("Error: couldn't create the folder {}", prefix.display()));
    let filename = CString::new(path.to_str().unwrap()).unwrap();
    unsafe {
        libc::mkfifo(filename.as_ptr(), 0o600);
    }
    path
}

#[no_mangle]
pub extern "C" fn send(name: &CString, value: &CString) {
    let path = __mkfifo(&name);
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .open(path)
        .expect("Error: couldn't open the named pipe");
    file.write_all(value.as_bytes())
        .expect("Error: couldn't write the named pipe");
}

#[no_mangle]
pub extern "C" fn receive(name: &CString) -> CString {
    let path = __mkfifo(&name);
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
