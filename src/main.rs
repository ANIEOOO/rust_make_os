// src/main.rs

#![no_std] // 不链接Rust标准库
#![no_main] // 禁用所有Rust层级的入口点

use core::panic::PanicInfo;

mod vga_buffer;


// 因为编译器会寻找一个名为`_start`的函数，所以这个函数就是入口点
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world!");
    panic!("Hello world!");
    loop {}
}

/// 这个函数将在panic时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}