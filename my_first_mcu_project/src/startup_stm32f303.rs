// 1. define the vector table for the mcu
// 2. define the reset handler for the mcu
// 3. define the panic handler i.e. exception handler for the mcu

fn reset_handler() {
  // 1. copy the .data section from flash to ram

  // 2. zero out the .bss section in ram

  // 3. call the main function
}