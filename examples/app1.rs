//! app1.rs
//!
//! Example of utilizing pend, the minimal RTFM example!

#![no_main]
#![no_std]
#![feature(asm, const_fn, core_intrinsics, naked_functions)]
#![feature(crate_visibility_modifier)]

use core::sync::atomic::{self, Ordering};
use cortex_m::asm;

mod tock;

use tock::*;


// pub use cortexm::nvic;
// pub use cortexm::scb;
// pub use cortexm::syscall;
// pub use cortexm::systick;

// panic handler
extern crate panic_semihosting;

use cortex_m_semihosting::hprintln;
use dwm1001::nrf52832_hal as hal;
use hal::nrf52832_pac as pac;
use pac::interrupt;
use rtfm::app;

// Return to Handler mode, exception return uses non-floating-point state
// from the MSP and execution uses MSP after return.
const RETURN_TO_HANDER_MODE_NO_FP_MSP: u32 = 0xFFFFFFF1;

// Return to Thread mode, exception return uses non-floating-point state from
// MSP and execution uses MSP after return.
const RETURN_TO_HANDER_MODE_FP_MSP: u32 = 0xFFFFFFF9;

// Return to Thread mode, exception return uses non-floating-point state from
// the PSP and execution uses PSP after return.
const RETURN_TO_THREAD_MODE_NO_FP_PSP: u32 = 0xFFFFFFFD;

// Return to Handler mode, exception return uses floating-point-state from
// MSP and execution uses MSP after return.
const RETURN_TO_THREAD_MODE_FP_MSP: u32 = 0xFFFFFFE1;

// Return to Thread mode, exception return uses floating-point state from
// MSP and execution uses MSP after return.
const RETURN_TO_THREAD_MODE_NO_FP_MSP: u32 = 0xFFFFFFE9;

// Return to Thread mode, exception return uses floating-point state from PSP
// and execution uses PSP after return.
const RETURN_TO_THREAD_MODE_FP_PSP: u32 = 0xFFFFFFED;



// #[link_section = ".app"]
// // Give half of RAM to be dedicated APP memory
static mut APP_MEMORY_BASE: * const u8 = 0x00030000 as * const u8;


/// This is used in the syscall handler. When set to 1 this means the
/// svc_handler was called. Marked `pub` because it is used in the cortex-m*
/// specific handler.
#[no_mangle]
#[used]
pub static mut SYSCALL_FIRED: usize = 0;

#[app(device = crate::hal::target)]
const APP: () = {
    static mut TOCKRAM: [usize; 1024] = [0; 1024];

    #[init]
    fn init() {
        hprintln!("init").unwrap();

        

        rtfm::pend(interrupt::SWI0_EGU0);


    }
    #[idle(resources = [TOCKRAM])]
    fn idle() -> ! {

        let mut init = 0;
        loop {
            hprintln!("idle").unwrap();

            let app_header = unsafe {tbf_header::parse_and_validate(APP_MEMORY_BASE)};

            if let Some(app) = app_header {

                let entry_function;
                unsafe {
                    entry_function = process::FunctionCall {
                        pc: (APP_MEMORY_BASE as usize) + app.get_init_function_offset() as usize,
                        argument0: (APP_MEMORY_BASE as usize) + app.get_protected_size() as usize,
                        argument1: resources.TOCKRAM.as_ptr() as usize,
                        argument2: resources.TOCKRAM.len(),
                        argument3: resources.TOCKRAM.as_ptr() as usize,
                    };
                }

                hprintln!("entry_function {:?}", entry_function);

                resources.TOCKRAM[1024-8] = entry_function.argument0;
                resources.TOCKRAM[1024-7] = entry_function.argument1;
                resources.TOCKRAM[1024-6] = entry_function.argument2;
                resources.TOCKRAM[1024-5] = entry_function.argument3;
                resources.TOCKRAM[1024-4] = 0x00000000;
                resources.TOCKRAM[1024-3] = 0x00000001;
                resources.TOCKRAM[1024-2] = entry_function.pc;
                resources.TOCKRAM[1024-1] = 0x01000000;

                unsafe {
                    asm!("
                        /* Load new stack into PSP */
                        msr psp, r0

                        /* Jump to SVCHandler */
                        svc #124"
                        : : "{r0}" (&resources.TOCKRAM[1024-8]) : : "volatile");
                }
            }
        }
        

        //loop {}
    }

    #[exception]
    #[naked]
    fn SVCall() {
        hprintln!("SVCALL");
        unsafe {
            // asm!(
            //     "
            //     // Return to Thread mode, exception return uses non-floating-point state from
            //     // the PSP and execution uses PSP after return.
            //     movw lr, #0xfffd
            //     movt lr, #0xffff
            //     bx lr"
            //         : : : : "volatile"
            // );
        

             asm!(
        "
            // if coming from kernel
            cmp lr, #0xfffffffd
            bne to_kernel

            movw lr, #0xfffd
            movt lr, #0xffff
            bx lr
          to_kernel:

            ldr r0, =SYSCALL_FIRED
            mov r1, #1
            str r1, [r0, #0]

            movw lr, #0xfffd
            movt lr, #0xffff
            bx lr"
            : : : : "volatile" );
         };
    }

    #[interrupt]
    fn SWI0_EGU0() {
        static mut TIMES: u32 = 0;
        *TIMES += 1;
        hprintln!("SWIO_EGU0 {}", TIMES).unwrap();
    }
};

#[naked]
fn tock_fn() {
//    hprintln!("tock");
    asm::bkpt();
    loop {
  //       hprintln!("tock");
         atomic::compiler_fence(Ordering::SeqCst);
    }
}
