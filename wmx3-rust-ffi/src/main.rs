use std::{thread::sleep, time::Duration};

// lib.rsでビルドしたFFIコードを参照
use wmx3_rust_ffi::*;

fn main() {
    let ret = unsafe { open_wmx() };
    println!("open_wmx = {}", ret);

    for _ in 0..10 {
        let pos = unsafe { get_pos(0) };
        println!("Axis0 Pos = {}", pos);
        sleep(Duration::from_secs(1));
    }

    let ret = unsafe { close_wmx() };
    println!("close_wmx = {}", ret);
}
