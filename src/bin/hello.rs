#![no_main]
#![no_std]

use esb_lr_ptx as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    esb_lr_ptx::exit()
}
