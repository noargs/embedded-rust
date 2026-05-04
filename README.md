## VS code extensions     
- rust     
- rust sytax (if doesn't come with rust extension)
- cortex-debug      

## MCU specific       
STM32CubeCLT(Tools for Third party IDEs) 
- In MacOSx you can find it in `/opt/ST/STM32CubeCLT_1.21.0` after download and installing `st-stm32cubeclt_1.21.0_27995_20260219_1804-macosx_x86_64.pkg` and `st-stlink-server.2.1.2-1.pkg`    

## Cortext-debug (VScode extension settings)      
You have to intergrate the **cortex-debug** VS code extension with **STM32CubeCLT**.    
- Go to VS code's Extension tab, click *cortex-debug* and click *settings* icon and go to *Settings*     

### Click **Cortex-Debug** and **External: GNU Tools** and write followings   
1- Click **Cortex-Debug: Arm Toolchain path** > **Edit in settings.json** as `"cortex-debug.armToolchainPath": "/opt/ST/STM32CubeCLT_1.21.0/GNU-tools-for-STM32/bin",`      
2- Click **Cortex-Debug: Arm Toolchain prefix** > **Edit in settings.json** as `"cortex-debug.armToolchainPrefix": "arm-none-eabi"`   

### Click **Cortex-Debug** and **External: GDB Servers** and write followings       
1- Click **Cortex-Debug: Stlink path > **Edit in settings.json** as `"cortex-debug.stlinkPath": "/opt/ST/STM32CubeCLT_1.21.0/STLink-gdb-server/bin/ST-LINK_gdbserver",`      
     
> [!IMPORTANT]     
> remove **.osx** at the end of `"cortex-debug.armToolchainPath"` and `"cortex-debug.stlinkPath"`    

### Go to Settings > Open Settings (JSON)     
Your settings json on MacOSX should look like following:      
```json
  "cortex-debug.armToolchainPath": "/opt/ST/STM32CubeCLT_1.21.0/GNU-tools-for-STM32/bin",
  "cortex-debug.stlinkPath": "/opt/ST/STM32CubeCLT_1.21.0/STLink-gdb-server/bin/ST-LINK_gdbserver",
  "cortex-debug.armToolchainPrefix": "arm-none-eabi",
```     
Your settings json on Windows should look like following:    
```json
  "cortex-debug.armToolchainPath": "C:\\ST\\STM32CubeCLT_1.21.0\\GNU-tools-for-STM32\\bin",
  "cortex-debug.stlinkPath": "C:\\ST\\STM32CubeCLT_1.21.0\\STLink-gdb-server\\bin\\ST-LINK_gdbserver.exe",
  "cortex-debug.armToolchainPrefix": "arm-none-eabi",
```     


    
## Configure Tasks      
In VS code go to menu **Terminal** > **Configure Task...** and type `cargo build` and hit enter. That will create **tasks.json** with the following content      
```json
{
	"version": "2.0.0",
	"tasks": [
		{
			"type": "cargo",
			"command": "build",
			"problemMatcher": [
				"$rustc"
			],
			"group": "build",
			"label": "rust: cargo build[MCU]"
		}
	]
}
```   
Change the **label** from `"rust: cargo build"` to `"rust: cargo build[MCU]"` to distinguish your created tasks from other     

> [!NOTE]     
> Shortcut to run the task **CMD + Shift + P** and click or type in **Tasks: Run task** and it will show the label of your task as **rust: cargo build[MCU]**       
    
## Create launch option      
Click play button in VS code and click **create a launch.json file** and select **Cortex Debug** which will create **launch.json** and that should look like following; you should include `"svdFile"` and `"device"` as well. And also add the **pre launch task**, a task you created previously.    
```json
{
  "version": "0.2.0",
  "configurations": [

    {
      "name": "Cortex Debug",
      "cwd": "${workspaceFolder}",
      "executable": "${workspaceFolder}/target/thumbv7em-none-eabihf/debug/my_first_mcu_project",
      "request": "launch",
      "type": "cortex-debug",
      "runToEntryPoint": "main",
      "servertype": "stlink",
      "svdFile" : "/opt/ST/STM32CubeCLT_1.21.0/STMicroelectronics_CMSIS_SVD/STM32F303.svd",
      "device" : "STM32F303",
      "preLaunchTask": "rust: cargo build[MCU]",
      "showDevDebugOutput": "raw"
    }
  ]
}
```  
   
## Run the program     
In VS code menu click **Run** and click **Start debugging**   

## View memory        
- `CMD + Shift + P` and write **>Cortex-Debug: View Memory** 
- Now mention the memory in the box (i.e. *Create new memory view*) `0x20000000`       

       
## To inspect binaries (ELF)           
**cargo-binutils**     
- `cargo install cargo-binutils`      
- `rustup component add llvm-tools-preview`          

## probe-rs           
- Previously we use Cortex-Debug VS Code extension to download and debug the ELF file onto the target hardware.      
- Similarlly probe-rs tool is an embedded debugging and target interaction toolkit. we will explore how to flash the ELF file onto the target hardware using this tool.    
- **install** it with `cargo install probe-rs-tools --locked`    
- to check **supported chips** run `probe-rs chip list`    
- to **flash** once you find out your chip being supported by probe-rs, run `cargo flash --chip STM32F303CC`    
- to know **more commands** with `cargo flash ...` run `cargo flash --help`      
- **another way of flashing** elf file using `cargo run` but you have to edit `.cargo/config.toml` mention the **runner** as follows      
```toml
[build]
target = "thumbv7em-none-eabihf"

[target.thumbv7em-none-eabihf]
rustflags = [
  "-C", "link-arg=-Tmemory.ld"
]
runner = 'probe-rs run --chip STM32F303CC'
```    
> [!IMPORTANT]    
> After mentioning **runner** in the `.cargo/config.toml` dont forget to do `cargo build` then you can do `cargo run` to flash the target     
> after flashing the debugging session remains on until you terminate with the ctrl+c to stop interacting with the board via the debugger.         