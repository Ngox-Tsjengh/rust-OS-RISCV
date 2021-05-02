#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]

#![macro_use]
mod lang_item;
mod console;
mod sbi;

global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| 
        {
           unsafe { (a as *mut u8).write_volatile(0)}
        }
    );
}

#[no_mangle]
extern "C" fn rust_main(){
    println!("Hello, world!");
    panic!("test");
    loop {}
}

