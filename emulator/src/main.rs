use jam::recompiler::Recompiler;

fn main() {
  let mut mips_registers: [u32; 31] = [0; 31];
  let aux_value: u32 = 0xbfc0_0000;
  mips_registers[1] = 0xdead_beef;
  mips_registers[8] = 0xffff_0000;
  let ptrs = [&mips_registers[0] as *const u32 as u64,
              &aux_value as *const u32 as u64];
  let inputs = [1, 8];
  let mut cc = Recompiler::new(&inputs, &ptrs);
  let x = cc.new_u32();
  cc.load_ptr(x, 1);
  let jitfn = cc.compile().unwrap();
  jitfn.run();
  println!("{:#x?}", mips_registers);
}
