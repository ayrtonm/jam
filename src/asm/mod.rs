use crate::Label;
use crate::StackOffset;
use crate::StackOffsetType;
use crate::X64Reg;
use std::collections::HashMap;

mod add;
mod and;
mod bt;
mod call;
mod cmp;
mod div;
mod flags;
mod jmp;
mod mov;
mod or;
mod shift;
mod stack;
mod sub;
mod test;
mod ui;
mod xchg;
mod xor;

pub(super) struct Assembler {
    buffer: Vec<u8>,
    label_counter: usize,
    labels_used: HashMap<StackOffset, Label>,
    labels_defined: HashMap<Label, StackOffset>,
}

impl Assembler {
    const ADD_EAX: u8 = 0x05;
    const ADD_I32: u8 = 0x81;
    const ADD_I8: u8 = 0x83;
    const ADD_R: u8 = 0x01;
    const AND_EAX: u8 = 0x25;
    const AND_EXT: u8 = (4 << 3);
    const AND_I32: u8 = 0x81;
    const AND_R: u8 = 0x21;
    const CALL: u8 = 0xe8;
    const CLC: u8 = 0xf8;
    const CMP: u8 = 0x39;
    const DIV: u8 = 0xf7;
    const DIV_EXT: u8 = (6 << 3);
    const IDIV_EXT: u8 = (7 << 3);
    const JC: u8 = 0x72;
    const JC_LONG: u8 = 0x82;
    const JE: u8 = 0x74;
    const JMP: u8 = 0xeb;
    const JNC: u8 = 0x73;
    const JNE: u8 = 0x75;
    const JNL: u8 = 0x7D;
    const JNS: u8 = 0x79;
    const JS: u8 = 0x78;
    const LABEL_PLACEHOLDER: u8 = 0xff;
    const MOD11: u8 = 0xc0;
    //FIXME: use more descriptive names for mov
    const MOV: u8 = 0x8b;
    const MOV2: u8 = 0x89;
    const OR_EAX: u8 = 0x0d;
    const OR_I32: u8 = 0x81;
    const OR_R: u8 = 0x09;
    const POP: u8 = 0x58;
    const PUSH: u8 = 0x50;
    const REX: u8 = 0x40;
    const REXB: u8 = 0x01;
    const REXR: u8 = 0x04;
    const REXW: u8 = 0x08;
    const SAR: u8 = 0xf8;
    const SHIFT: u8 = 0xc1;
    const SHL: u8 = 0xe0;
    const SHR: u8 = 0xe8;
    const STC: u8 = 0xf9;
    const SUB_EAX: u8 = 0x2D;
    const SUB_EXT: u8 = (5 << 3);
    const SUB_I32: u8 = 0x81;
    const SUB_I8: u8 = 0x83;
    const SUB_R: u8 = 0x29;
    const TEST: u8 = 0x85;
    const XCHG: u8 = 0x87;
    const XOR: u8 = 0x31;

    fn emit_label(&mut self, label: Label) {
        let location = StackOffset(self.buffer.len() as StackOffsetType);
        match label.size {
            StackOffset(1) => {
                self.labels_used.insert(location, label);
                self.emit_u8(Assembler::LABEL_PLACEHOLDER);
            },
            StackOffset(4) => {
                self.labels_used.insert(location, label);
                self.emit_u8(Assembler::LABEL_PLACEHOLDER);
                self.emit_u8(Assembler::LABEL_PLACEHOLDER);
                self.emit_u8(Assembler::LABEL_PLACEHOLDER);
                self.emit_u8(Assembler::LABEL_PLACEHOLDER);
            },
            _ => unreachable!(""),
        }
    }

    fn rexb(reg: X64Reg) -> u8 {
        match reg.is_extended() {
            true => Assembler::REXB,
            false => 0,
        }
    }

    fn rexr(reg: X64Reg) -> u8 {
        match reg.is_extended() {
            true => Assembler::REXR,
            false => 0,
        }
    }

    fn emit_cond_rexb(&mut self, reg: X64Reg) {
        if reg.is_extended() {
            self.emit_u8(Assembler::REX | Assembler::REXB);
        };
    }

    fn emit_cond_rexrb(&mut self, reg1: X64Reg, reg2: X64Reg) {
        if reg1.is_extended() || reg2.is_extended() {
            self.emit_u8(Assembler::REX | Assembler::rexr(reg1) | Assembler::rexb(reg2));
        }
    }

    fn emit_rexwrb(&mut self, reg1: X64Reg, reg2: X64Reg) {
        self.emit_u8(
            Assembler::REX | Assembler::REXW | Assembler::rexr(reg1) | Assembler::rexb(reg2),
        );
    }

    fn emit_u8(&mut self, imm8: u8) {
        self.buffer.push(imm8);
    }

    fn emit_u16(&mut self, imm16: u16) {
        imm16.to_ne_bytes().iter().for_each(|&b| {
            self.emit_u8(b);
        });
    }

    fn emit_u32(&mut self, imm32: u32) {
        imm32.to_ne_bytes().iter().for_each(|&b| {
            self.emit_u8(b);
        });
    }

    fn emit_u64(&mut self, imm64: u64) {
        imm64.to_ne_bytes().iter().for_each(|&b| {
            self.emit_u8(b);
        });
    }
}
