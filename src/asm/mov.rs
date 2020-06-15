use crate::asm::Assembler;
use crate::X64Reg;
use crate::StackOffset;

impl Assembler {
  pub fn emit_movq_ir(&mut self, src: u64, dest: X64Reg) {
    let prefix = Assembler::REX | Assembler::REXW | Assembler::rexb(dest);
    self.emit_u8(prefix);
    self.emit_u8(0xb8 | dest.low());
    self.emit_u64(src);
  }
  pub fn emit_movl_mr_offset(&mut self, ptr: X64Reg, dest: X64Reg, offset: StackOffset) {
    let offset = offset.0;
    self.emit_cond_rexrb(dest, ptr);
    self.emit_u8(Assembler::MOV);
    match offset {
      -0x80..=0x7f => {
        self.emit_u8(0x40 | dest.low() << 3 | ptr.low());
        if ptr.low() == 4 {
          self.emit_u8(0x24);
        };
        self.emit_u8(offset as u8)
      },
      _ => {
        todo!("");
      },
    }
  }
  pub fn emit_movl_rm_offset(&mut self, src: X64Reg, ptr: X64Reg, offset: StackOffset) {
    let offset = offset.0;
    self.emit_cond_rexrb(src, ptr);
    self.emit_u8(Assembler::MOV2);
    match offset {
      -0x80..=0x7f => {
        self.emit_u8(0x40 | src.low() << 3 | ptr.low());
        if ptr.low() == 4 {
          self.emit_u8(0x24);
        };
        self.emit_u8(offset as u8)
      },
      _ => {
        todo!("");
      },
    }
  }
  pub fn emit_movq_mr(&mut self, ptr: X64Reg, dest: X64Reg) {
    let prefix = Assembler::REX | Assembler::REXW | Assembler::rexr(dest) | Assembler::rexb(ptr);
    self.emit_u8(prefix);
    self.emit_u8(Assembler::MOV);
    if ptr.low() == 5 {
      self.emit_u8(0x45 | dest.low() << 3);
      self.emit_u8(0x00);
    } else {
      self.emit_u8(dest.low() << 3 | ptr.low());
      if ptr.low() == 4 {
        self.emit_u8(0x24);
      }
    }
  }
  pub fn emit_movq_mr_offset(&mut self, ptr: X64Reg, dest: X64Reg, offset: StackOffset) {
    let offset = offset.0;
    let prefix = Assembler::REX | Assembler::REXW | Assembler::rexr(dest) | Assembler::rexb(ptr);
    self.emit_u8(prefix);
    self.emit_u8(Assembler::MOV);
    self.emit_u8(0x40 | dest.low() << 3 | ptr.low());
    if ptr.low() == 4 {
      self.emit_u8(0x24);
    };
    match offset {
      -128..=127 => self.emit_u8(offset as u8),
      _ => self.emit_u32(offset as u32),
    }
  }
}