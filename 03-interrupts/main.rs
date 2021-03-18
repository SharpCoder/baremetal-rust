#![feature(lang_items, asm)]
#![crate_type = "staticlib"]
#![no_std]
mod clocks;
mod interrupts;
mod platform;
mod timer;

const CLOCK_RELOAD_VALUE: u32 = 0xFFFF_FFDF;
static mut sysclock: timer::Timer = timer::Timer::new(platform::DMTIMER2);

#[no_mangle]
pub fn kmain() {
    initialize_platform();
    initialize_interrupts();

    loop {
        unsafe {
            asm!("nop");
        }
    }    
}

fn initialize_platform() {
    unsafe {
        *((platform::CM_PER + clocks::CM_PER_L4LS_CLKSTCTRL) as *mut u32) = 0x2;
        *((platform::CM_PER + clocks::CM_PER_TIMER2_CLKCTRL) as *mut u32) = 0x2;
    }
}

fn initialize_interrupts() {
    // Enable DMTimer2
    sysclock.stop();
    sysclock.set_load_value(CLOCK_RELOAD_VALUE);
    sysclock.set_value(CLOCK_RELOAD_VALUE);
    sysclock.configure(timer::ENABLE_AUTO_RELOAD | timer::IRQ_OVERFLOW_MODE);
    sysclock.irq_enable();

    // Wire up register
    interrupts::register_handler(interrupts::INT_DMTIMER2, handle_timer_irq);
    interrupts::unmask_interrupt(interrupts::INT_DMTIMER2);

    // Start the clock
    sysclock.start();
}

fn handle_timer_irq() {
    sysclock.irq_disable();
    sysclock.stop();
    sysclock.irq_acknowledge();
    sysclock.irq_clear();
    sysclock.set_value(CLOCK_RELOAD_VALUE);
    sysclock.incr();
    sysclock.irq_enable();
    sysclock.start();
}

#[no_mangle]
fn handle_irq_rust() {
    let int_number = interrupts::get_active_irq_number();
    interrupts::service_handler(int_number);
    interrupts::clear_interrupts();
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() { }

#[panic_handler]
#[no_mangle]
pub extern fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop { }
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() { }