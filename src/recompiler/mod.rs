use std::io;
use crate::alloc::Allocator;
use crate::asm::Assembler;
use crate::jit_fn::JITFn;
use crate::StackOffset;
use crate::StackOffsetType;
use crate::X64Reg;
use crate::EmuReg;
use crate::EmuRegName;
use crate::EmuRegNameType;
use crate::PtrType;
use crate::JITValue;
use crate::Variable;

mod abi;
mod ui;

pub struct Recompiler {
  alloc: Allocator,
  asm: Assembler,
}

impl Recompiler {
  pub fn new(inputs: &[EmuRegNameType], pointers: &[PtrType]) -> Self {
    let alloc = Allocator::new();
    let asm = Assembler::new();
    let mut recompiler = Recompiler { alloc, asm };
    recompiler.sysv_prologue();
    recompiler.load_pointers(pointers);
    recompiler.load_emu_regs(inputs);
    recompiler
  }
  pub fn compile(mut self) -> io::Result<JITFn> {
    self.free_variables();
    self.save_emu_regs();
    self.free_pointers();
    self.sysv_epilogue();
    *self.alloc.native_ptrs_mut() += self.asm.emit_retq();
    #[cfg(debug_assertions)]
    assert_eq!(self.alloc.full_stack(), StackOffset(0));
    self.asm.assemble()
  }
  fn new_variable(&mut self, size: StackOffset) -> JITValue {
    let offset = self.asm.emit_addq_ir(-size.0, X64Reg::RSP);
    *self.alloc.stack_mut() += offset;
    let position = self.alloc.full_stack();
    JITValue::Variable(Variable { position, size })
  }
  fn load_value(&mut self, value: JITValue) {
    let transfers = self.alloc.load_value(value);
    transfers.map(|t| self.asm.emit_transfers(t, self.alloc.full_stack()));
  }
  fn loadq_stack(&mut self, dest: JITValue, offset: StackOffset) {
    self.movq_mv_offset(X64Reg::RSP, dest, offset);
  }
  fn movq_mv_offset(&mut self, src: X64Reg, dest: JITValue, offset: StackOffset) {
    self.load_value(dest);
    let dest_reg = self.alloc.value_to_reg(&dest).expect("");
    self.asm.emit_movq_mr_offset(src, *dest_reg, offset.0);
  }
  fn free_variables(&mut self) {
    let offset = self.asm.emit_addq_ir(self.alloc.stack().0, X64Reg::RSP);
    *self.alloc.stack_mut() += offset;
  }
}

