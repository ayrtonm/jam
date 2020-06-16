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
    self.emit_use_label(label);
  }
  pub fn emit_use_label(&mut self, label: Label) {
    let location = self.buffer.len();
    self.buffer.push(Assembler::LABEL_PLACEHOLDER);
    self.labels_used.insert(StackOffset(location as StackOffsetType), label);
  }
}
