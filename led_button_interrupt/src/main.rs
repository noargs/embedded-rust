#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]
#![allow(dead_code)]


use board::*;
// use button::*;
use led::*;
use core::panic::PanicInfo;

mod board;
// mod button;
mod gpio;
mod led;
mod mcu;
mod reg;
mod startup_stm32f303;

#[unsafe(no_mangle)]
fn main() {
    led_init(BLUE_LED_PORT, BLUE_LED_PIN);
    led_init(GREEN_LED_PORT, GREEN_LED_PIN);
    led_init(RED_LED_PORT, RED_LED_PIN);

    led_on(BLUE_LED_PORT, BLUE_LED_PIN);
    led_on(GREEN_LED_PORT, GREEN_LED_PIN);
    led_on(RED_LED_PORT, RED_LED_PIN);
    
    // button_init(BUTTON_PIN);
    // button_configure_interrupt(BUTTON_PIN);

    loop {}
}

#[panic_handler]
fn panic_handler(_info: & PanicInfo) -> ! {
    loop {}
}

// button interrupt handler
#[allow(non_snake_case)]
fn EXTI0_Handler() {
    led_toggle(BLUE_LED_PORT, BLUE_LED_PIN);
}
