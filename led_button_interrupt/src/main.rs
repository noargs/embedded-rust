#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]


use board::*;
use button::*;
use led::*;
use core::panic::PanicInfo;

mod board;
mod button;
mod gpio;
mod led;
mod mcu;
mod reg;
mod startup_stm32f303;

#[unsafe(no_mangle)]
fn main() {
    led_init(BLUE_LED_PORT, BLUE_LED_PIN);
    led_on(BLUE_LED_PORT, BLUE_LED_PIN);
    // button_init(BUTTON_PIN);
    // button_configure_interrupt(BUTTON_PIN);

    loop {}
}

#[panic_handler]
fn panic_handler(_info: & PanicInfo) -> ! {
    loop {}
}

fn EXTI0_Handler() {
    led_toggle(BLUE_LED_PORT, BLUE_LED_PIN);
}
