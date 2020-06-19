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

macro_rules! trash {
  ($offset:expr) => {
    let _ = $offset;
  }
}

macro_rules! bind {
  ($self:expr, $transfers:expr) => {
    {
      $self.asm.emit_transfers($transfers, $self.alloc.full_stack());
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
  fn bind_multivalue(&mut self, values: Vec<JITValue>) -> Vec<X64Reg> {
    bind!(self, self.alloc.bind_multivalue(&values));
    let mut bound_regs = Vec::new();
    for v in values {
      bound_regs.push(*self.alloc.value_to_reg(&v).expect(""));
    }
    bound_regs
  }
  fn bind_value(&mut self, value: JITValue) -> X64Reg {
    bind!(self, self.alloc.bind_value(value));
    *self.alloc.value_to_reg(&value).expect("")
  }
  fn sysv_caller_prologue(&mut self) {
    stack!(self, X64Reg::caller_saved_regs().into_iter()
                                            .map(|r| self.asm.emit_pushq_r(r))
                                            .sum::<StackOffset>());
  }
  fn sysv_caller_epilogue(&mut self) {
    stack!(self, X64Reg::caller_saved_regs().into_iter()
                                            .rev()
                                            .map(|r| self.asm.emit_popq_r(r))
                                            .sum::<StackOffset>());
  }
}
