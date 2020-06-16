use crate::asm::Assembler;
use crate::Label;
use crate::StackOffset;
use crate::StackOffsetType;
use crate::X64Reg;

impl Assembler {
  pub fn emit_jmp_rel8(&mut self, offset: i8) {
    self.emit_u8(Assembler::JMP);
    self.emit_u8(offset as u8);
  }
  pub fn emit_jmp_label(&mut self, label: Label) {
    self.emit_u8(Assembler::JMP);
    self.emit_label(label);
  }
  pub fn emit_label(&mut self, label: Label) {
    let location = StackOffset(self.buffer.len() as StackOffsetType);
    self.labels_used.insert(location, label);
    self.emit_u8(Assembler::LABEL_PLACEHOLDER);
  }
}
