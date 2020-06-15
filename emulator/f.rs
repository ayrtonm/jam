#![feature(llvm_asm)]
fn main() {
  let out: u64;
  unsafe {
  llvm_asm!("
    xorq %rax, %rax
    addq %rsp, %rax
    pushq %rcx
    subq %rsp, %rax
    popq %rcx":"={rax}"(out));
  }
  println!("{}", out);
}
