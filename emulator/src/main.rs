use jam::recompiler::Recompiler;

fn main() {
  let mut mips_registers: [u32; 31] = [0; 31];
  let aux_value: u64 = 0xbfc0_0000;
  mips_registers[1] = 0xdead_beef;
  mips_registers[8] = 0xffff_0000;
  let ptrs = [&mips_registers[0] as *const u32 as u64,
              &aux_value as *const u64 as u64];
  let inputs = [1, 8];
  let mut rc = Recompiler::new(&inputs, &ptrs);
  let r1 = rc.reg(1).expect("");
  rc.load_ptr(r1, 1);
  rc.deref_u64(r1);
  let jitfn = rc.compile().unwrap();
  assert_eq!(mips_registers[1], 0xdead_beef);
  jitfn.run();
  assert_eq!(mips_registers[1], aux_value as u32);
  println!("{:x?}", mips_registers);
  println!("{:x?}", aux_value);
}
