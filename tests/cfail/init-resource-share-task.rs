#![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

extern crate cortex_m_rtfm as rtfm;
extern crate stm32f103xx;

use rtfm::app;

app! { //~ proc macro panicked
    device: stm32f103xx,

    resources: {
        static BUFFER: [u8; 16] = [0; 16];
    },

    init: {
        resources: [BUFFER],
    },

    tasks: {
        SYS_TICK: {
            path: sys_tick,
            // ERROR resources assigned to `init` can't be shared with tasks
            resources: [BUFFER],
        },
    },
}

fn init(_p: init::Peripherals) {}

fn idle() -> ! {
    loop {}
}

fn sys_tick() {}
