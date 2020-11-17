use crate::asm::Assembler;
use crate::X64Reg;

impl Assembler {
    pub fn emit_cmpl_rr(&mut self, reg1: X64Reg, reg2: X64Reg) {
        self.emit_cond_rexrb(reg1, reg2);
        self.emit_u8(Assembler::CMP);
        self.emit_u8(Assembler::MOD11 | reg1.low() << 3 | reg2.low());
    }
}
