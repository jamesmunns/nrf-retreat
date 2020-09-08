#![no_std]
#![no_main]

use cortex_m_rt::entry;
use esb_lr_ptx as _; // memory layout + panic handler

#[entry]
fn main() -> ! {
    assert!(false, "TODO: Write actual tests");

    esb_lr_ptx::exit();
}
