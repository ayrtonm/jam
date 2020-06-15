use crate::alloc::Allocator;
use crate::asm::Assembler;
use crate::StackOffset;
use crate::X64Reg;
use crate::JITValue;
use crate::Variable;

mod abi;
mod ui;

pub struct Recompiler {
  alloc: Allocator,
  asm: Assembler,
}

impl Recompiler {
  fn new_variable(&mut self, size: StackOffset) -> JITValue {
    let offset = self.asm.emit_addq_ir(-size.0, X64Reg::RSP);
    *self.alloc.stack_mut() += offset;
    let position = self.alloc.full_stack();
    JITValue::Variable(Variable { position, size })
  }
  fn bind_value(&mut self, value: JITValue) -> X64Reg {
    let transfers = self.alloc.bind_value(value);
    self.asm.emit_transfers(transfers, self.alloc.full_stack());
    *self.alloc.value_to_reg(&value).expect("")
  }
}

