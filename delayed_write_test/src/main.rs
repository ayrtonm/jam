use jam::recompiler::Recompiler;

fn print_fn(x: u32, y: u32) {
  println!("-------------------");
  println!("R0: {:x}\nDelayed write: {:x}", x, y);
}
fn main() {
  let mut mips_registers: [u32; 32] = [4; 32];
  let ptrs = [&mips_registers[0] as *const u32 as u64,
              print_fn as *const fn(u32, u32) as u64];
  let print_idx = 1;
  let inputs = [0];
  let mut rc = Recompiler::new(&inputs, &ptrs);
  let r0 = rc.reg(0).unwrap();
  let delay = rc.new_delayed_write(r0);
  rc.set_arg1(r0);
  rc.set_arg2(delay);
  rc.call_ptr(print_idx);

  rc.seti_u32(delay, 5);
  rc.set_arg1(r0);
  rc.set_arg2(delay);
  rc.call_ptr(print_idx);

  rc.process_delayed_write();
  rc.set_arg1(r0);
  rc.call_ptr(print_idx);

  let jitfn = rc.compile().unwrap();
  jitfn.run();
}
