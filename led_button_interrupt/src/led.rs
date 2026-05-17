use core::ptr;



fn clear_bits(value: u32, mask: u32) -> u32 {
  value & !mask
}

fn set_bits(value: u32, mask: u32) -> u32 {
  value | mask
}

pub fn led_init(port: u32, pin: u32) {
  // 1. set the gpio pin mode -> output mode
  let offset = 0;
  let gpio_mode_reg_addr = (port + offset) as *mut u32;

  // calculate the bit position for the given pin
  let bit_position = pin * 2;
  let mode_mask = 0x3 << bit_position;
  let mode_value = 0x1 << bit_position;

  unsafe {
    let mut gpio_mode_reg_value = read_register(gpio_mode_reg_addr);
    gpio_mode_reg_value = clear_bits(gpio_mode_reg_value, mode_mask); // clear the bits for the pin
    gpio_mode_reg_value = set_bits(gpio_mode_reg_value, mode_value);  // set the bits for output mode
    write_register(gpio_mode_reg_addr, gpio_mode_reg_value);
  }
  // 2. set the output type -> push-pull
  // 3. set the output speed -> (optional)

}

pub fn led_on(port: u32, pin: u32) {

}

pub fn led_off(port: u32, pin: u32) {

}

pub fn led_toggle(port: u32, pin: u32) {

}