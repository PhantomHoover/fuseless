#![allow(unused_imports, unused_variables, dead_code)]

use ctor::*;
use std::fs;
use std::env;
use std::process;
use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::OsStrExt;
use sysinfo;

#[ctor]
unsafe fn fuseless_ctor() {
    let mut args: Vec<OsString> = fs::read("/proc/self/cmdline").unwrap()
        .split(|x| *x == 0u8).skip(1)
        .map(|x| OsStr::from_bytes(x).to_owned()).collect();
    args.pop();

    let pid = sysinfo::get_current_pid().unwrap();
    let wd = std::env::current_dir().unwrap();
    let tmpdir = format!("/tmp/fuseless.{}", pid);
    std::fs::create_dir(&tmpdir).unwrap();
    std::env::set_current_dir(&tmpdir).unwrap();

    let mut child = std::process::Command::new("/proc/self/exe")
        .arg("--appimage-extract")
        .spawn().unwrap();
    child.wait().unwrap();

    std::env::set_current_dir(wd).unwrap();
    let app_path = std::path::Path::new(&tmpdir).join("squashfs-root").join("AppRun");
    let mut child = std::process::Command::new(app_path).args(args).spawn().unwrap();
    let status = child.wait().unwrap();

    std::fs::remove_dir_all(&tmpdir).unwrap();
    std::process::exit(status.code().unwrap_or(1));
}
