use crate::asm::Assembler;
use crate::X64Reg;

impl Assembler {
    pub fn emit_xchgq_rm(&mut self, reg: X64Reg, ptr: X64Reg) {
        self.emit_rexwrb(reg, ptr);
        self.emit_u8(Assembler::XCHG);
        if ptr.low() == 5 as u8 {
            self.emit_u8(0x45 | reg.low() << 3);
            self.emit_u8(0x00);
        } else {
            self.emit_u8(reg.low() << 3 | ptr.low());
            if ptr.low() == 4 as u8 {
                self.emit_u8(0x24);
            }
        }
    }

    pub fn emit_xchgq_rr(&mut self, reg1: X64Reg, reg2: X64Reg) {
        if reg1 == X64Reg::RAX || reg2 == X64Reg::RAX {
            let rexb = Assembler::rexb(reg1) | Assembler::rexb(reg2);
            let prefix = Assembler::REX | Assembler::REXW | rexb;
            self.emit_u8(prefix);
            self.emit_u8(0x90 | reg1.low() | reg2.low());
        } else {
            let prefix =
                Assembler::REX | Assembler::REXW | Assembler::rexb(reg1) | Assembler::rexr(reg2);
            self.emit_u8(prefix);
            self.emit_u8(0x87);
            self.emit_u8(Assembler::MOD11 | reg1.low() | reg2.low() << 3);
        }
    }
}
