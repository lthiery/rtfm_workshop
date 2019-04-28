//! app1.rs
//!
//! Example of utilizing pend, the minimal RTFM example!

#![no_main]
#![no_std]

// panic handler
extern crate panic_semihosting;

use cortex_m_semihosting::hprintln;
use dwm1001::nrf52832_hal as hal;
use hal::nrf52832_pac as pac;
use pac::interrupt;
use rtfm::app;

#[link_section = ".app_memory"]
// Give half of RAM to be dedicated APP memory
static mut APP_MEMORY: [u8; 0x10000] = [0; 0x10000];

#[app(device = crate::hal::target)]
const APP: () = {
    #[init]
    fn init() {
        hprintln!("init").unwrap();
        rtfm::pend(interrupt::SWI0_EGU0);
    }
    #[idle]
    fn idle() -> ! {
        hprintln!("idle").unwrap();
        rtfm::pend(interrupt::SWI0_EGU0);
        loop {}
    }

    #[interrupt]
    fn SWI0_EGU0() {
        static mut TIMES: u32 = 0;
        *TIMES += 1;
        hprintln!("SWIO_EGU0 {}", TIMES).unwrap();
    }
};
