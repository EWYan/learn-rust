#![feature(asm_const)]
#![feature(format_args_nl)]
#![no_main]
#![no_std]

mod bsp;
mod cpu;
mod panic_wait;
mod print;
mod console;

unsafe fn kernel_init() -> ! {
    println!("Hello from Rust!");

    loop {
        use aarch64_cpu::asm;
        asm::wfe()
    }
}