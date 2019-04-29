//! app1.rs
//!
//! Example of utilizing pend, the minimal RTFM example!

#![no_main]
#![no_std]
#![feature(asm, const_fn, core_intrinsics, naked_functions)]

use core::sync::atomic::{self, Ordering};
use cortex_m::asm;


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

// xPSR
// PC
// LR
// R12
// R3
// R2
// R1
// R0

// #[repr(C, align(16))]
// #[derive(Copy, Clone)]
// pub struct stack_frame {
//     align: u32,
//     //align1: u32,
//     xPSR: u32,
//     PC: u32,
//     LR: u32,
//     R12: u32,
//     R3: u32,
//     R2: u32,
//     R1: u32,
//     // aligned to 16
//     R0: u32,
// }

// impl stack_frame {
//     const fn new() -> stack_frame {
//         stack_frame {
//             align: 0,
//             // align1: 0,
//             xPSR: 0,
//             PC: 0,

//             LR: 0,
//             R12: 0,
//             R3: 0,
//             R2: 0,
//             R1: 0,
//             R0: 0,
//         }
//     }
// }

// fn switch_to_user(
//     mut user_stack: *const usize,
//     process_regs: &mut [usize; 8],
// ) -> *const usize {
//     asm!("
//     /* Load bottom of stack into Process Stack Pointer */
//     msr psp, $0

//     /* Load non-hardware-stacked registers from Process stack */
//     /* Ensure that $2 is stored in a callee saved register */
//     ldmia $2, {r4-r11}

//     /* SWITCH */
//     svc 0xff /* It doesn't matter which SVC number we use here */
//     /* Push non-hardware-stacked registers into Process struct's */
//     /* regs field */
//     stmia $2, {r4-r11}

//     mrs $0, PSP /* PSP into r0 */"
//     : "={r0}"(user_stack)
//     : "{r0}"(user_stack), "{r1}"(process_regs)
//     : "r4","r5","r6","r7","r8","r9","r10","r11" : "volatile" );
//     user_stack
// }

// 0x0003002c      0x20002000      0x00002000      0x20002c00
// 0x00000000      0x00000001      0x00030055      0x01000000

#[app(device = crate::hal::target)]
const APP: () = {
    static mut TOCKRAM: [u32; 1024] = [0; 1024];

    #[init]
    fn init() {
        hprintln!("init").unwrap();
        rtfm::pend(interrupt::SWI0_EGU0);
    }
    #[idle(resources = [TOCKRAM])]
    fn idle() -> ! {
        hprintln!("idle").unwrap();

        resources.TOCKRAM[1024-8] = 0x0003002c;
        resources.TOCKRAM[1024-7] = 0x20002000;
        resources.TOCKRAM[1024-6] = 0x00002000;
        resources.TOCKRAM[1024-5] = 0x20002c00;
        resources.TOCKRAM[1024-4] = 0x00000000;
        resources.TOCKRAM[1024-3] = 0x00000001;
        resources.TOCKRAM[1024-2] = 0x00030055;
        resources.TOCKRAM[1024-1] = 0x01000000;

        hprintln!("psp = {:x?}", (&resources.TOCKRAM[1024-8]) as *const u32);

        unsafe {
            asm!("
                /* Load new stack into PSP */
                msr psp, r0

                /* Jump to SVCHandler */
                svc #124"
                : : "{r0}" (&resources.TOCKRAM[1024-8]) : : "volatile");
        }

        loop {}
    }

    #[exception]
    #[naked]
    fn SVCall() {
        hprintln!("SVCALL");
        unsafe {
            asm!(
                "
                // Return to Thread mode, exception return uses non-floating-point state from
                // the PSP and execution uses PSP after return.
                movw lr, #0xfffd
                movt lr, #0xffff
                bx lr"
                    : : : : "volatile"
            );
        }

        loop {}
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
