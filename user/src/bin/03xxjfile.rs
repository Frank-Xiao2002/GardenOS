#![no_std]
#![no_main]

use core::arch::asm;

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("Hello, world!");
    println!("Xiao Xiaojin HERE!!!!!");
    unsafe {
        asm!("sret");
    }
    0
}

