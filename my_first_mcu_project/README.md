
    
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
```ld
[target.thumbv7m-none-eabi]
rustflags = [
  "-C", "link-arg=-Tmemory.ld"
]
```     
* `memory.ld` is your linker script file.      
    
**Uses external linker**.      
```ld
[target.thumbv7m-none-eabi]
linker = "arm-none-eabi-ld"
rustflags = [
  "-C", "link-arg=-Tmemory.ld"
]
```      
- [More info on LLD of LLVM](https://lld.llvm.org/ELF/linker_script.html)     
- LLD implements a large subset of the GNU ld linker script notation as they are documented in ld [manual](https://sourceware.org/binutils/docs/ld/Scripts.html).     
- You can also refer to GNU linker [documentation](https://ftp.gni.org/old-gnu/Manuals/ld-2.9.1/html_chapter/ld_toc.html#TOC5) as it is compatible with LLVM linker       
    



 

