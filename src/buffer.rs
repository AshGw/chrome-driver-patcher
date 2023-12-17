use memfd::{MemfdOptions,Memfd};
use nix::unistd::fexecve;
use std::{ffi::CString,io::Write};
use std::{os::unix::io::AsRawFd,env};

// this executes the binary directly into memory so a mf avoids elevated privileges
pub fn execute(buffer: &[u8], args: impl Iterator<Item = String>){
    let memfd: Memfd = MemfdOptions::default()
    .close_on_exec(true)
    .create("chromedriver")
    .expect("Failed to create mem fd");

    memfd.as_file().write_all(buffer).expect("Error writing to the file");

    let cargs: Vec<CString> = args.map(|s: String| CString::new(s).unwrap()).collect();
    let cvars: Vec<CString> = env::vars()
        .map(|(key, val)| CString::new(format!("{}={}", key, val)).unwrap())
        .collect();
   fexecve(memfd.as_raw_fd(), &cargs, &cvars)
        .expect("Failed to Execute the patched binary");
}

