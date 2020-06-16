use jam::recompiler::Recompiler;
use jam::ArgNumber;

extern fn print(x: u32) {
  let y = x + 1;
  println!("{:x} + 1 = {:x}", x, y);
  return;
}

fn main() {
  let mut mips_registers: [u32; 32] = [0; 32];
  let aux_value: u32 = 0xbfc0_0000;
  let aux_value_2: u32 = 0xf0f0_abcd;
  mips_registers[1] = 0xdead_beef;
  mips_registers[8] = 0xffff_0000;
  let ptrs = [&mips_registers[0] as *const u32 as u64,
              &aux_value as *const u32 as u64,
              &aux_value_2 as *const u32 as u64,
              print as *const fn(u32) as u64];
  let inputs = (0..32).collect::<Vec<_>>();
  let mut rc = Recompiler::new(&inputs, &ptrs);
  for i in 0..32 {
    let r = rc.reg(i).unwrap();
    rc.load_ptr(r, 1);
    rc.deref_u32(r);
  }
  let r8 = rc.reg(8).unwrap();
  let r1 = rc.reg(1).unwrap();
  rc.set_argn(r1, ArgNumber::Arg1);
  rc.call_ptr(3);
  rc.load_ptr(r8, 2);
  rc.deref_u32(r8);
  let r2 = rc.reg(2).unwrap();
  let r3 = rc.reg(3).unwrap();
  rc.setv_u32(r2, r8);
  rc.seti_u32(r3, 0xf0f0_0f0f);
  let jitfn = rc.compile().unwrap();
  assert_eq!(mips_registers[1], 0xdead_beef);
  jitfn.run();
  println!("{:#x?}", mips_registers);
  println!("{} bytes", jitfn.size());
  assert_eq!(mips_registers[1], aux_value as u32);
}
