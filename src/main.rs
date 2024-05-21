#![no_std]
#![no_main]

use core::arch::global_asm;

mod semantics;
mod system_call;

global_asm!(include_str!("asm/entry.asm"));

fn _main() {}
