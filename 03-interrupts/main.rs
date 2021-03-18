#![feature(lang_items, asm)]
#![crate_type = "staticlib"]
#![no_std]
mod timer;

/* 
Static memory locations and offsets
*/
const CM_PER: u32 = 0x44E0_0000; // Base addr for clock
const DMTIMER2: u32 = 0x4804_0000; // Base addr for timer2
const INTCPS: u32 = 0x4820_0000; // Base addr for interrupt
const CM_PER_L4LS_CLKSTCTRL: u32 = 0x0; // CM_PER offset
const CM_PER_TIMER2_CLKCTRL: u32 = 0x80; // CM_PER offset
const INTC_CONTROL: u32 = 0x48;
const INTC_MIR_CLEAR2: u32 = 0xC8;

/* Helper constants */
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
        *((CM_PER + CM_PER_L4LS_CLKSTCTRL) as *mut u32) = 0x2;
        *((CM_PER + CM_PER_TIMER2_CLKCTRL) as *mut u32) = 0x2;
    }
}

fn initialize_interrupts() {
    // Enable DMTimer2
    sysclock.stop();
    sysclock.set_load_value(CLOCK_RELOAD_VALUE);
    sysclock.set_value(CLOCK_RELOAD_VALUE);
    sysclock.configure(
        timer::ENABLE_AUTO_RELOAD | timer::IRQ_OVERFLOW_MODE
    );
    sysclock.irq_enable();

    // Wire up register
    unsafe {
        // Unmask INT2 interrupt (DMTIMER2) which allows
        // interrupts generated from the timer to
        // propagate.
        *((INTCPS + INTC_MIR_CLEAR2) as *mut u32) = 0x4;
    }

    // Start the clock
    sysclock.start();
}

#[no_mangle]
fn handle_irq_rust() {
    // Increment timer and reload value
    sysclock.irq_disable();
    sysclock.stop();
    sysclock.irq_acknowledge();
    sysclock.irq_clear();
    sysclock.set_value(CLOCK_RELOAD_VALUE);
    sysclock.incr();
    sysclock.irq_enable();
    sysclock.start();
    
    // Clear interrupts
    unsafe {
        *((INTCPS + INTC_CONTROL) as *mut u32) = 0x1;
    }
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