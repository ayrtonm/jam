use crate::asm::Assembler;
use crate::X64Reg;

impl Assembler {
  pub fn emit_btl_ir(&mut self, imm5: u32, reg: X64Reg) {
    #[cfg(debug_assertions)]
    assert!(imm5 < 32);
    self.emit_cond_rexb(reg);
    self.emit_u8(0x0f);
    self.emit_u8(0xba);
    self.emit_u8(0xe0 | reg.low());
    self.emit_u8(imm5 as u8);
  }
}
