use crate::gpio::*;

pub fn led_init(port: u32, pin: u32) {
  //1. enable the GPIO port clock (peripheral clock for the GPIO port)
  enable_gpio_clock(port);
  
  // 2. set the gpio pin mode -> output mode
  set_gpio_mode_output(port, pin);

  // 3. set the output type -> push-pull
  set_gpio_output_type_push_pull(port, pin);

  // 4. set the output speed -> (optional)

}

pub fn led_on(port: u32, pin: u32) {
  set_gpio_pin_state(port, pin, PinState::High);
}

pub fn led_off(port: u32, pin: u32) {
  set_gpio_pin_state(port, pin, PinState::Low);
}

pub fn led_toggle(port: u32, pin: u32) {
  set_gpio_pin_state(port, pin, PinState::Toggle);

}