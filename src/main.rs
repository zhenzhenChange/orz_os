#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod kits;
mod semantics;
mod system_call;

core::arch::global_asm!(include_str!("asm/entry.asm"));

#[no_mangle]
fn orz_os_main() -> ! {
    kits::ffi::clear_bss();
    println!("RustOrzOS .");
    panic!("shutdown.");
}
