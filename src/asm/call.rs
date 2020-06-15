use crate::StackOffset;
use crate::X64Reg;
use crate::asm::Assembler;

impl Assembler {
  pub fn emit_callq_r(&mut self, reg: X64Reg) -> StackOffset {
    self.emit_cond_rexb(reg);
    self.emit_u8(0xff);
    self.emit_u8(0xd0 | reg.low());
    StackOffset(8)
  }
  pub fn emit_callq_m(&mut self, reg: X64Reg) -> StackOffset {
    self.emit_cond_rexb(reg);
    self.emit_u8(0xff);
    if reg.low() == 5 {
      self.emit_u8(0x55);
      self.emit_u8(0x00);
    } else {
      self.emit_u8(0x10 | reg.low());
      if reg.low() == 4 {
        self.emit_u8(0x24);
      }
    }
    StackOffset(8)
  }
  pub fn emit_callq_m_offset(&mut self, reg: X64Reg, offset: StackOffset) -> StackOffset {
    let offset = offset.0;
    if offset == 0 {
      self.emit_callq_m(reg)
    } else {
      self.emit_cond_rexb(reg);
      self.emit_u8(0xff);
      match offset {
        -0x80..=0x7f => {
          self.emit_u8(0x50 | reg.low());
          if reg.low() == 4 {
            self.emit_u8(0x24);
          }
          self.emit_u8(offset as u8);
        },
        _ => {
          self.emit_u8(0x90 | reg.low());
          if reg.low() == 4 {
            self.emit_u8(0x24);
          }
          self.emit_u32(offset as u32);
        },
      }
      StackOffset(8)
    }
  }
  pub fn emit_retq(&mut self) -> StackOffset {
    self.emit_u8(0xc3);
    StackOffset(-8)
  }
}
