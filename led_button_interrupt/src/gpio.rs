use crate::reg::*;

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