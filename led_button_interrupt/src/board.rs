use crate::mcu::*;

// Fastbit nano board specific definitions
pub const BLUE_LED_PIN: u32 = GPIO_PIN_0;
pub const BLUE_LED_PORT: u32 = GPIOA_BASE;

pub const GREEN_LED_PIN: u32 = GPIO_PIN_2;
pub const GREEN_LED_PORT: u32 = GPIOB_BASE;

pub const RED_LED_PIN: u32 = GPIO_PIN_3;
pub const RED_LED_PORT: u32 = GPIOC_BASE;