ENTRY(reset_handler)

MEMORY
{
  FLASH (rx)    : ORIGIN = 0x08000000, LENGTH = 256K
  RAM (rwx)     : ORIGIN = 0x20000000, LENGTH = 40K
  CCMRAM (rwx)  : ORIGIN = 0x10000000, LENGTH = 8K

  /* non volatile data even powered off i.e. calibration, constants, config settings */
  /* EEPROM (rwx)  : ORIGIN = 0x08000000, LENGTH = 4K */
  /* battery backed RAM */
  /* BATTRAM (rw)  : ORIGIN = 0x40024000, LENGTH = 4K */
}

_start_of_stack = ORIGIN(RAM) + LENGTH(RAM);
_min_stack_size = 0x400;       /* 1kb = 1024 bytes = 0x400 */
_min_heap_size  = 0x400;

/*
 * SECTIONS command in a linker script specifies where each
 * section (e.g, .text, .data, .bss) should be placed in 
 * in memory
 */

SECTIONS
{
  /*
   * 1. defines where each program section is placed in the 
   *    microcontroller's memory
   *
   * 2. linker assigns addresses based on the SECTIONS commands,
   *    creating an ELF file wwith exact memory locations.
   *    For example, if FLASH starts at 0x08000000 and the text
   *    section is placed in the FLASH, the linker will assign
   *    address starting from 0x08000000 for the .text section
   *
   * 3. the ELF file guides the programming tool to correctly
   *    place the section in the microcontroller ensuring proper
   *    execution of the code
   */

   .text :
   {
    /* . = 0x08000000 */
    . = ALIGN(4);
    LONG(_start_of_stack);
    KEEP(*(.isr_vector));

    /* here you should collect all executable code */
    *(.text)
    *(.text*)
    . = ALIGN(4);
   } > FLASH

   .rodata :
   {
    . = ALIGN(4);
    *(.rodata)
    *(.rodata*)
    . = ALIGN(4);
   } > FLASH

  /*
   * initialised data will be in FLASH and it will also be in
   * in RAM during runtime
   */
   .data :
   {
    /* this returns the FLASH (LMA) of the .data section */
    _sidata = LOADADDR(.data); 
    . = ALIGN(4);
    
    /* start of data section in VMA(RAM) */
    _sdata = .; 
    *(.data)
    *(.data*)
    . = ALIGN(4);
    _edata = .;
   } > RAM AT> FLASH

   /* uninitialised data will be RAM */
   .bss :
   {
    . = ALIGN(4);
    _sbss = .;
    *(.bss)
    *(.bss*)
    . = ALIGN(4);
    _ebss = .;
   } > RAM

   .ram_usage_check :
   {
    . = . + _min_stack_size;
    . = . + _min_heap_size;
   } > RAM
}

INCLUDE "device_STM32F303.x"