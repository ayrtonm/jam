use crate::alloc::Allocator;
use crate::asm::Assembler;
use crate::StackOffset;
use crate::X64Reg;
use crate::JITValue;
use crate::Variable;

macro_rules! stack {
  ($self:expr, $offset:expr) => {
    {
      let offset = $offset;
      *$self.alloc.stack_mut() += offset;
    }
  }
}

mod abi;
mod ui;

pub struct Recompiler {
  alloc: Allocator,
  asm: Assembler,
}

impl Recompiler {
  fn new_variable(&mut self, size: StackOffset) -> JITValue {
    stack!(self, self.asm.emit_addq_ir(-size.0, X64Reg::RSP));
    let position = self.alloc.full_stack();
    JITValue::Variable(Variable { position, size })
  }
  fn bind_value(&mut self, value: JITValue) -> X64Reg {
    let transfers = self.alloc.bind_value(value);
    self.asm.emit_transfers(transfers, self.alloc.full_stack());
    *self.alloc.value_to_reg(&value).expect("")
  }
  fn sysv_prologue(&mut self) {
    let offset = X64Reg::callee_saved_regs().into_iter()
                                            .map(|r| self.asm.emit_pushq_r(r))
                                            .sum::<StackOffset>();
    *self.alloc.stack_mut() += offset;
  }
  fn sysv_epilogue(&mut self) {
    let offset = X64Reg::callee_saved_regs().into_iter()
                                            .rev()
                                            .map(|r| self.asm.emit_popq_r(r))
                                            .sum::<StackOffset>();
    *self.alloc.stack_mut() += offset;
  }
}

