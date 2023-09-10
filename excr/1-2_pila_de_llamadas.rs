use std::arch::asm;

pub fn main() {
    println!("=== Stack trace from fp chain ===\n");
    
    let mut frame_pointer: usize;
    unsafe {
        asm!(
            "mov {}, fp",
            out(reg) frame_pointer,
        );

        while frame_pointer !=0 {
            println!("Frame Pointer: {:#x}", frame_pointer);

            frame_pointer = *(frame_pointer as *const usize).offset(-1);
        }
    }
}
