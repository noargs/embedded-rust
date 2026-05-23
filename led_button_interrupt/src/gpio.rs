use crate::{mcu::*, reg::*};

pub fn enable_gpio_clock(port: u32) {
    let rcc_ahbenr_addr = (RCC_BASE + 0x14) as *mut u32;

  match port {
    GPIOA_BASE => {
      // enable 17th bit of RCC_AHBENR to enable clock for GPIOA
      reg_set_bit(rcc_ahbenr_addr, 17, true);
    },
    GPIOB_BASE => {
      reg_set_bit(rcc_ahbenr_addr, 18, true);
    },
    GPIOC_BASE => {
      reg_set_bit(rcc_ahbenr_addr, 19, true);
    },
    // Add more cases for other GPIO ports if needed
    _ => {}
  }
}

pub fn set_gpio_mode_output(port: u32, pin: u32) {
  let gpio_mode_reg_addr = (port + 0x00) as *mut u32;
  let bit_position = pin * 2;
  let mode_value = 0x1;

  reg_set_bits(gpio_mode_reg_addr, mode_value, bit_position, 2);
}

pub fn set_gpio_output_type_push_pull(port: u32, pin: u32) {
  let gpio_op_type_reg_addr = (port + 0x04) as *mut u32;
  let bit_position = pin;
  let bit_value = 0;

  reg_set_bits(gpio_op_type_reg_addr, bit_value, bit_position, 1);
}

pub enum PinState {
  High,
  Low,
  Toggle,
}

pub fn set_gpio_pin_state(port: u32, pin: u32, pin_state: PinState) {
  let gpio_bsrr_addr = (port + 0x18) as *mut u32;
  
  match pin_state {
    PinState::High => {
      reg_set_val(gpio_bsrr_addr, 1 << pin);
    },
    PinState::Low => {
      reg_set_val(gpio_bsrr_addr, 1 << (pin + 16));
    },
    PinState::Toggle => {
      //1. Read the GPIO pin state using ODR (BSRR is write-only, so we need to read from ODR)
      let gpio_odr_addr = (port + 0x14) as *mut u32;
      //2. Toggle the pin state by writing to BSRR, according to current state (read from ODR)
      if reg_read_bit(gpio_odr_addr, pin) {
        // Pin is high, reset it to low
        reg_set_val(gpio_bsrr_addr, 1 << (pin + 16));
      } else {
        // Pin is low, set it to high
        reg_set_val(gpio_bsrr_addr, 1 << pin);
      }
    }
  }
}