use crate::asm::Assembler;
use crate::StackOffset;
use crate::X64Reg;

impl Assembler {
    pub fn emit_addl_rr(&mut self, src: X64Reg, dest: X64Reg) {
        //let's avoid this since it would give a StackOffset that varies at JIT-runtime
        #[cfg(debug_assertions)]
        assert!(src != X64Reg::RSP);
        self.emit_cond_rexrb(src, dest);
        self.emit_add_rr(src, dest);
    }

    pub fn emit_add_rr(&mut self, src: X64Reg, dest: X64Reg) {
        self.buffer.push(Assembler::ADD_R);
        self.buffer
            .push(Assembler::MOD11 | src.low() << 3 | dest.low());
    }

    pub fn emit_addl_ir(&mut self, imm32: i32, reg: X64Reg) -> StackOffset {
        self.emit_cond_rexb(reg);
        self.emit_add_ir(imm32, reg)
    }

    pub fn emit_addq_ir(&mut self, imm32: i32, reg: X64Reg) -> StackOffset {
        let prefix = Assembler::REX | Assembler::REXW | Assembler::rexb(reg);
        self.emit_u8(prefix);
        self.emit_add_ir(imm32, reg)
    }

    fn emit_add_ir(&mut self, imm32: i32, reg: X64Reg) -> StackOffset {
        match imm32 {
            -0x80..=0x7f => {
                self.emit_u8(Assembler::ADD_I8);
                self.emit_u8(Assembler::MOD11 | reg.low());
                self.emit_u8(imm32 as u8);
            },
            _ => {
                if reg == X64Reg::RAX {
                    self.emit_u8(Assembler::ADD_EAX);
                } else {
                    self.emit_u8(Assembler::ADD_I32);
                    self.emit_u8(Assembler::MOD11 | reg.low());
                }
                self.emit_u32(imm32 as u32);
            },
        };
        if reg == X64Reg::RSP {
            -StackOffset(imm32)
        } else {
            StackOffset(0)
        }
    }
}
