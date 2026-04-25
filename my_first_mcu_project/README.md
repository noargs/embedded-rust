
    
### Creare project     
`cargo new my_first_mcu_project`    
   

### Build project (native compilation, same host+target) 
`cargo build`      
     

### Build project (cross compilation, for different target)           
- find the target first with `rustup target list`     

| ARM Cortex Mx Processor  | Architecture | `rustup target list` |  
|--------------------|--------|-----------|
| Arm Cortex-M0 | Armv6-M | `thumbv6m-none-eabi` |
| Arm Cortex-M0+ | Armv6-M | `thumbv6m-none-eabi` | 
| Arm Cortex-M3  | Armv7-M | `thumbv7m-none-eabi` | 
| Arm Cortex-M4     | Armv7E-M | `thumbv7em-none-eabi`, or `-eabihf` |
| Arm Cortex-M7     | Armv7E-M | `thumbv7em-none-eabi`, or `-eabihf` | 

`cargo build --target thumbv7em-none-eabihf`    
   
- **thumbv7em**: rustc generates code for target architecture: ARM Cortext-M (Thumb instruction set, ARMv7-M architecutre).  
- **none**: without any operating system.    
- **eabihf**: code will adhere to the ARM Embedded Application Interface (EABI) with hardware floating-point support.   
   
> [!IMPORTANT]  
> If error **error[E0463]: can't find crate for `std`**.    
> **note: the `thumbv7em-none-eabihf` target may not be installed**.    
> install it with `rustup target add thumbv7em-none-eabihf`     
    
> [!NOTE]  
> If error **error[E0463]: can't find crate for `std`**.    
> **note: the `thumbv7em-none-eabihf` target may not support the standard library**.    
> In bare-metal systems, **no_std** means the rust standard library (std) is not available because there is no operating system to provide features like file I/O, threads, or heap memory. You must use the core library for basic functionality and handle everything directly with the hardware.       

### **loop {...}**      
will run forever unless explicitly terminated by a **break**, **return** statement, or an **error/panic**     
    
### Attributes (like `#![no_std]`)      
Attributes provide instructions and metadata to the compiler, affecting code interpretation. Something similar in C as *compiler directives* to change the compiler behaviour.     
   
Attributes can be used to,    
1) Control conditional compilation    
2) Derive traits automatically.    
3) Modify function behavior.   
4) Manage test cases.  
5) Indicate deprecation, etc.   
6) Errors and Warning Management.    
7) Manage the Memory layout of a struct or union.   
8) Many more.     
    
**Syntax of attributes**.      
The basic form of an attribute with no parameters `#[attribute]`.     
Where *attribute* is the name of the attribute      
    
Example:
`#[test]`, `#[ignore]`, `#[inline]`
An 'attribute' can be,
1) Built-in attributes (Standardized by the rust lang)
  • https://doc.rust-lang.org/reference/attributes.html
2) Custom attributes (implemented using procedural macros)  
    
### Attribute related to lint checks      
• What are lints?
  • "lints" are checks performed by the compiler.
  • For example detecting for `unused_variable` in a program is a lint check.
  • For this lint check the compiler may issue warnings or errors. Depends on the default behavour of the compiler.
• Developers can control the behavior of these lint checks using attributes like `#[allow(…)]`, `#[warn(…)]`, `#[deny(...)]`, and `#[forbid(...)]`       
    
### Inner and outer attributes    
- **Inner attributes**: denoted by `#![attribute]`, apply to the entire scope in which they are placed, which can be either the whole crate or a specific module.   
- **Outer attribute**: denoted by `#[attribute]`, apply to the specific item that immediately follows the attribute in the source code.      
    
### `#[forbid(lint)]`
- forbid attribute used to enforce strict compliance with specific lint checks,     
- forbid attribute takes precedence over allow, warn, and deny. Once a lint rule has been set to forbid, you cannot downgrade its severity to warn or allow within the scope where forbid has been applied.      
    

### Why `#[panic_handler]`       
running following code will give us **error: `#[panic_handler]` function required, but not found**     

```rust
#![no_std]
fn main() {
    
    loop{

    }
}
```    
    
- If an out-of-bounds access is attempted on array or slices, Rust will trigger a panic.     
- Using **unwrap()** on **Option** or **Result** types performs a runtime check to see if the value is **None** or **Err**. If it is, Rust will panic.     
- In debug builds, Rust performs checks for integer overflow. If an overflow occurs, it triggers a panic.    
- Programmer can manually trigger a panic using the **panic!** macro or assertions such as `assert!`, `assert_eq!`, and `assert_ne!` even in a `no_std` environment. 
     

### #![no_main]
• Crate level attribute
• Disables the default Rust **main** entry point, which is typically used in applications that rely on the standard library (std)
• By using this attribute, you can define your own custom entry point, which is particularly useful for embedded systems or bare-metal applications.     
     
### #[no_mangle]
• Item level attribute
• you ensure that the function name remains as you define it, without Rust's typical name mangling. This is important for interoperability with other code and for ensuring that the linker and startup routines correctly recognize the function name.     
    
## Final working code       
```rust
#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
fn main() {
    loop{}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
```    
    
> [!IMPORTANT]  
> to skip typing **--target** flag i.e. `cargo build --target thumbv7em-none-eabihf` you can do the following     
```bash
$ mkdir .cargo && cd .cargo && touch config.toml 
$ sudo nano config.toml 
[build]
target = "thumbv7em-none-eabihf"   
```    
> Now you can run `cargo build` **without the target option**     
    
### Executable location      
- You can find the executable at:   
`my_first_mcu_project/target/thumbv7em-none-eabihf/debug/my_first_mcu_project`   
- running file command on executable will give you following:    
`target/thumbv7em-none-eabihf/debug/my_first_mcu_project: ELF 32-bit LSB executable, ARM, EABI5 version 1 (SYSV), statically linked, with debug_info, not stripped`  
    

## Why '!' type?
• When we use the **never type (!)** to mark a return value or a type in Rust, it provides information to the compiler that helps it make smarter decisions regarding
1. Type inference and coercion
2. Control flow analysis
3. Code optimization.      
   
> [!NOTE]    
> The Rust compiler can coerce the ! type into other types (like i32) when needed to satisfy type requirements. This is because the ! type is considered a "bottom type," meaning it can be coerced into any other type since it will never produce a value.      
    
### What does it mean to say a function returns the 'never' type?
A function that returns the **never type(!)** is one that **never returns control to its caller**     
It either:     
• Loops infinitely, or   
• Terminates the program (e.g., by panicking or exiting), or  
• Transfers control elsewhere (e.g., via break or a system call that halts execution).     
    
## ELF file inspection
![ELF](../imgs/01.png)     
   
Make sure you have **cargo-binutils** installed and the necessary components.     
`cargo install cargo-binutils`      
`rustup component add llvm-tools-preview`.      
    
- **cargo objdump -- -h <elf file>** Provides a high-level overview of the sections in the ELF file, including their sizes and addresses.     
    
- **cargo readobj -- -S/-h <elf file>** Provides detailed information about each section in the ELF file.      
   
# Startup code typically includes      
1. **Vector table**
  • Defines the initial stack pointer and the addresses of interrupt and exception handlers
2. **Reset Handler**
  • This is the entry point to our program which initializes the hardware and sets up the runtime environment
3. **Exception handlers**     
   
### Importance of start-up file     
• The startup file is responsible for setting up the right environment for the main user code to run
• Code written in startup file runs before main(). So, you can say startup file calls main()
• Some part of the startup code file is the target (Processor) dependent
• Startup code takes care of vector table placement in code memory as required by the ARM cortex Mx processor
• Startup code may also take care of stack reinitialization
• Startup code is responsible of .data, .bss section initialization in main memory      
     
> [!INFO].     
> startup file in this project `startup_stm32f303.rs`.    
    
**To complete the `reset_handler()`, we need section information. To get section information we need to write linker script.**      
     
### What is a linker script?
A linker script is a text file used by the linker to control the layout of the output executable file. It provides detailed instructions on how to map the input obiect files and their sections into the final executable file and how these sections should be placed in memory.     
    
• You can create custom sections that are not typically generated by the compiler. For example, you might want to create a section for specific types of data or code.     
• Control the exact placement of sections in memory, specifying which memory addresses to use for different sections.     
• Set attributes for sections, such as whether they are read-only, executable, or writable.      
    
## Linker     
**LLD (the LLVM linker)**:    
  This is the default linker Rust uses for the `{arm,thumb}*-none-eabi(hf)` targets and other similar embedded targets
**External Linker**:
  You can also configure Rust to use an external linker like *arm-none-eabi-ld*, which is part of the GNU Arm Embedded Toolchain.     
  add the following to your **.cargo/config.toml** `[target.<your-target>]`     
  linker = "arm-none-eabi-ld"

## Linker flags.      
Add this entry to **.cargo/config.toml** file of your project.     
**Uses default linker (rust-lld)**    
```toml
[target.thumbv7m-none-eabi]
rustflags = [
  "-C", "link-arg=-Tmemory.ld"
]
```     
* `memory.ld` is your linker script file.      
    
**Uses external linker**.      
```toml
[target.thumbv7m-none-eabi]
linker = "arm-none-eabi-ld"
rustflags = [
  "-C", "link-arg=-Tmemory.ld"
]
```      
- [More info on LLD of LLVM](https://lld.llvm.org/ELF/linker_script.html)     
- LLD implements a large subset of the GNU ld linker script notation as they are documented in ld [manual](https://sourceware.org/binutils/docs/ld/Scripts.html).     
- You can also refer to GNU linker [documentation](https://ftp.gni.org/old-gnu/Manuals/ld-2.9.1/html_chapter/ld_toc.html#TOC5) as it is compatible with LLVM linker     

## Important Linker scripts commands           
| Command  | Meaning |   
|----------|---------|  
| MEMORY | Defines memory regions available on the target device. |     
| SECTIONS | Tells the linker how input sections are mapped to output sections and placed in memory |     
| ENTRY | Specifies the entry point of the program |     
| OUTPUT | Specifies the name of the output file |     
| PROVIDE | Defines a symbol if it is not already defined |     
| ASSERT | Tests an assertion and stops linking if false |     
| KEEP | Ensures that the linker retains the specified sections |     
| AT | Specifies a different load address for a section |     
| ALIGN | Aligns the current location to a specified boundary |     
| LOADADDR(section) | Returns the absolute load address of the section |     
| SIZEOF | Returns the size of a section |     
| ORIGIN | Returns the origin address of a memory region |     
| LENGTH | Returns the length of a memory region |      

- For more info refer: https://ftp.gnu.org/old-gnu/Manuals/ld-2.9.1/html_chapter/ld_3.html#SEC6     

![System and memory overview of STM32F303xB/C](../imgs/02.png)       
   

## Memory layout attributes        
```ld
MEMORY
{
  FLASH (rx)    : ORIGIN = 0x08000000, LENGTH = 256K
  RAM (rwx)     : ORIGIN = Ox20000000, LENGTH = 64K
  CCMRAM (rwx)  : ORIGIN = 0x10000000, LENGTH = 64K

  /*non volatile data even powered off i.e. calibration, constants, config settings*/
  /* EEPROM (rwx)  : ORIGIN = 0x08080000, LENGTH = 4K */ 
  /* Battery backed RAM*/
  /* BATTRAM (rw)  : ORIGIN = 0x40024000, LENGTH = 4k */
}
```          
   
• Attributes i.e. (rwx) in a linker script specify the intended use of memory regions (e.g., read-only, writable, executable). They guide the linker in placing sections appropriately, especially for sections not explicitly listed in the script.   
     
• Attributes ensure that sections are placed in memory regions that match their required characteristics, helping to avoid incorrect memory usage and potential runtime errors.    
    
## Different types of data of a program      
A program contains various kinds of data, which can be categorized based on their storage location, mutability, and initialization state      
- Read-only data        
- Initialized data      
- Uninitialized data     
- Stack and heap data      

### Read-only data (.rodata)       
**Do not consume RAM space**   
- String literals      
- Constant variables (const)     
- Static immutable variables      
- Stored in ROM or Flash memory    
    
```rust
const PI: f64 = 3.141592653589793;         // constant variable
const NUMBERS: [i32; 5] = [1, 2, 3, 4, 5]; // constant array
static SCORES_GLOBAL: [i32; 5] = [1, 2, 3, 4, 5]; // static immutable array 

fn main() {
  let message = "This is a string literal."  // string literal
}
```      
    
```ld
.rodata :
{

} > FLASH
```     
    
**You can mention `AT` here i.e. `> FLASH AT> FLASH`. However redundant here as you use `AT` only when load address different than execution address**
      
      
### Initialised data (.data)     
**Rust does not have a separate concept of global variables as seen in some other languages. Global variables are handled through static variables**        
**Consume both ROM and RAM**       
- Initiliased static mut variables (global variables) .data      
- Initialised local variables (stack)      
- .data section is always part of the ROM and at runtime, are copied from ROM to RAM, where the program can modify them.     
      
```rust
// (Initiailzed global variable, data section)
static mut GLOBAL_COUNTER: 132 = 1;

fn main() {
  let local_var: 132 = 42; // Initialized local variable (stack)

  unsafe {
  // modifying global variable is unsafe in rust
  GLOBAL_COUNTER += 1;
  printin! ("Global counter: {}", GLOBAL_COUNTER);
}
```     

> [!NOTE]      
> Here, RAM address is also called as **Virtual Memory Address** (VMA). This is the address where the section is loaded in RAM at runtime      
> FLASH address is also called as **Load Memory Address(LMA)**. This is the address where the section is stored in FLASH initially before being copied to RAM during startup       

```ld
.data :
{

} > RAM AT> FLASH
```    
**Use `AT` only when load address is different than executiona address**
       
     
### Unintialised data (.bss)        
**ROM just stores size information of the .bss section RAM stores entire .bss section**
• Uninitialized global variables.
• Uninitialized static variables.
• Uninitialized static mut variables (typically zero-initialized).
• These data consume no significance space in the ROM, only metadata indicating the size of the .bs section. During runtime the startup code zeros out the .bass section in RAM      
     
```rust     
// This would go into the BSS section
static mut UNINITIALIZED_ARRAY: [u8; 1024] = [0; 1024];
fn main() -> ! {
  unsafe {
    UNINITIALIZED_ARRAY [0] = 1;
  }   
}
```          
      
### Stack:
• Local variables.
• Function call management (return addresses parameters).        
     
### Heap:
• Dynamically allocated memory (using Box, Vec, etc., in Rust).      
      
```rust
fn main() {
  // 42 is stored in heap during runtime
  let heap_allocated_data = Box::new (42);
  
  // [1, 2, 3, 4, 5] is stored in heap during runtime
  let vec_of_numbers = vec![1, 2, 3, 4, 5];
}     
```        
     
![Merging sections](../imgs/03.png)          
      

## Location counter ('.')      
• The location counter (.) represents the current memory address within the section being processed.    
• As the linker processes the sections defined in the SECTIONS command, it automatically updates the location counter to reflect the current position in memory.      
• You can use the location counter to define the start or end of sections or to create gaps between sections by manipulating its value.     
    
- Below `_sdata` or `_sbss` (start data/bss) are linker symbols not variables    
    
```ld
  .data :
  {
  _sidata = LOADADDR(.data);   /* This returns the FLAS (LMA) of the .data section */
    _sdata = .;        /* start of data section in VMA(RAM) */
    *(.data)
    *(.data*)
    _edata = .;
  } > RAM AT> FLASH

  /* uninitialised data will be RAM */
  .bss : 
  {
    _sbss = .;
    *(.bss)
    *(.bss*)  
    _ebss = .;  
  } > RAM
```       
     
![Code memory(FLASH), Data memory(SRAM)](../imgs/04.png) 

### ALIGN(n)        
    
- Aligns the location counter to the next multiple of n bytes
  Example:
  ALIGN(4) => location counter is adjusted to the next address that is a multiple of 4. If the current address is already aligned to 4 bytes, it remains unchanged.       
  
- ALIGN ensures that your code and data sections are correctly placed at their natural and appropriate memory boundaries, preventing misalignment and potential faults      
      
```
SECTIONS
{
  .text :
  {
    /* . = 0x0800 0000 */
    . = ALIGN(4);
    /* here you should collect all executable code */
    *(.text)
    *(.text*)
    . = ALIGN(4);
  } > FLASH

  .rodata :
  {
    . = ALIGN(4);
    ...
    . = ALIGN(4);

  } > FLASH

  .data :
  {
    _sidata = LOADADDR(.data);   /* This returns the FLASH (LMA) of the .data section */
    . = ALIGN(4);
    _sdata = .;        /* start of data section in VMA(RAM) */
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
    ...
    . = ALIGN(4);
    _ebss = .;  
  } > RAM
}
```
      
### RAM usage check with the help of linker       
We reserver 1Kb for Stack and 1kb for heap. If the location counter goes beyond the maximum RAM size, the linker could throw error.    
```ld
  .ram_usage_check :
  {
    . = ALIGN(8);
    . = . + _min_stack_size;   /* 1kb = 1024bytes = 0x400 */
    . = . + _min_heap_size;
    . = ALIGN(8);
  } > RAM   
```

![Initialise the Initial Stack pointer](../imgs/05.png)     
     
> [!NOTE]       
> In ARM Cortex Mx processor, the stack pointer is aligned to 8 byte boundary. Because 8 byte boundary alignment is important during push and pop operations.    
> For example, consider a situation of pushing and popping a 64 bit data like double or long long. So there are 64 bit data types, and these data types should be stored at addresses that are multiples of 8 bytes for efficient access. And also 8 byte alignment inherently includes 8 byte alignment as well. 
> We have used `ALIGN(8)` in `.ram_usage_check` section       
      
## Test our program     
```bash
$ cargo clean
$ cargo build
$ cargo objdump -- -h target/thumbv7em-none-eabhif/debug/my_first_mcu_project

# ## ELF file inspection ##
# print all details of the ELF file
$ cargo readobj -- -all <elf file>

# print the contents of each section in an ELF file
$ cargo readobj -- -x .data|.rodata|.text <elf file>

# display the Symbol table
$ cargo readobj -- -s <elf file>
```