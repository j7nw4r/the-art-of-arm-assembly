use std::arch::global_asm;

global_asm!(
    "
// Listing1-3.s
    .text

    .global _asmMain, asmMain
    .align 2

_asmMain:
asmMain:

// Empty function just to return the Rust Code.

    ret
    "
);

unsafe extern "C" {
    fn asmMain();
}

fn main() {
    println!("Calling asmMain");
    unsafe {
        println!("{:?}", asmMain());
    }
    println!("Returned from asmMain");
}
