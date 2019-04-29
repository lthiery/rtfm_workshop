#![no_main]
#![feature(asm, const_fn, lang_items)]
#![no_std]

// panic handler
extern crate panic_semihosting;

use cortex_m_semihosting::hprintln;
use dwm1001::nrf52832_hal as hal;
use rtfm::app;

static mut APP_MEMORY: [u8; 0x1000] = [0; 0x1000];

#[link_section = "APP"]
// Give half of RAM to be dedicated APP memory
static mut APP_MEMORY: [u8; 0x10000] = [0; 0x10000];

/// Dummy buffer that causes the linker to reserve enough space for the stack.
pub static mut STACK_MEMORY: [u8; 0x1000] = [0; 0x1000];

// enter thread mode with process stack pointer (for both floating point state and execution)
const fn enter_thread_mode_with_psp() {
    // an LDR instruction with PC as the destination
    // triggers exception return (EXC_RETURN)

    // 0xFFFFFFED == Return to Thread mode, exception return uses floating-point state from PSP
    // and execution uses PSP after return.
    unsafe {
         asm!(
            "ldr sp, #0xFFFFFFED"
        : : : : "volatile" );
    }
}


#[app(device = crate::hal::target)]
const APP: () = {
    #[init]
    fn init() {
        hprintln!("init").unwrap();
    }

    #[idle]
    fn idle() -> ! {
        hprintln!("idle").unwrap();
        enter_thread_mode_with_psp();
        loop {}
    }

    //fn int_1 ->! {
        //enter_thread_mode();
    //}
};
