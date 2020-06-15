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
use crate::recompiler::Recompiler;

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
    let transfers = self.alloc.unbind_regs();
    self.asm.emit_transfers(transfers, self.alloc.full_stack());
    self.save_emu_regs();
    self.free_pointers();
    self.sysv_epilogue();
    *self.alloc.native_ptrs_mut() += self.asm.emit_retq();
    #[cfg(debug_assertions)]
    assert_eq!(self.alloc.full_stack(), StackOffset(0));
    self.asm.assemble()
  }
  fn free_variables(&mut self) {
    let offset = self.asm.emit_addq_ir(self.alloc.stack().0, X64Reg::RSP);
    *self.alloc.stack_mut() += offset;
  }
  pub(super) fn sysv_prologue(&mut self) {
    let offset = X64Reg::callee_saved_regs().into_iter()
                                            .map(|r| self.asm.emit_pushq_r(r))
                                            .sum::<StackOffset>();
    *self.alloc.native_ptrs_mut() += offset;
  }
  pub(super) fn load_pointers(&mut self, pointers: &[PtrType]) {
    let offset = pointers.iter()
                         .rev()
                         .map(|&p| self.asm.emit_pushq_i(p))
                         .sum();
    *self.alloc.emulator_ptrs_mut() += offset;
  }
  pub(super) fn load_emu_regs(&mut self, inputs: &[EmuRegNameType]) {
    self.asm.emit_movq_mr(X64Reg::RSP, X64Reg::RAX);
    let offset = self.alloc.full_stack();
    let _ = self.asm.emit_addq_ir(-(inputs.len() as StackOffsetType) * 4, X64Reg::RSP);
    let regs = inputs.iter()
                     .enumerate()
                     .map(|(n, &i)| {
                       let reg_offset = StackOffset(4 * (n + 1) as StackOffsetType);
                       let emu_reg = EmuReg {
                         name: EmuRegName(i),
                         position: offset + reg_offset,
                         size: StackOffset(4),
                       };
                       self.alloc.emulator_regs_mut().push(emu_reg);
                       emu_reg
                     })
                     .collect::<Vec<_>>();
    let regs = regs.iter()
                   .map(|reg| {
                     (reg.name, self.alloc.reg_position(&reg))
                   })
                   .collect::<Vec<_>>();
    regs.iter()
        .for_each(|(name, pos)| {
          let src_offset = StackOffset(4 * name.0 as StackOffsetType);
          self.asm.emit_movl_mr_offset(X64Reg::RAX, X64Reg::RCX, src_offset);
          self.asm.emit_movl_rm_offset(X64Reg::RCX, X64Reg::RSP, *pos);
        });
  }
  pub(super) fn save_emu_regs(&mut self) {
    self.asm.emit_movq_mr_offset(X64Reg::RSP, X64Reg::RAX, self.alloc.ptr_position(0));
    let regs = self.alloc
                   .emulator_regs()
                   .iter()
                   .map(|&r| {
                     (r.name, self.alloc.reg_position(&r), r.size)
                   })
                   .collect::<Vec<_>>();
    let offset = regs.into_iter()
                     .map(|(name, pos, size)| {
                       let dest_offset = StackOffset(4 * name.0 as StackOffsetType);
                       self.asm.emit_movl_mr_offset(X64Reg::RSP, X64Reg::RCX, pos);
                       self.asm.emit_movl_rm_offset(X64Reg::RCX, X64Reg::RAX, dest_offset);
                       size
                     })
                     .sum::<StackOffset>();
    let _ = self.asm.emit_addq_ir(offset.0, X64Reg::RSP);
    self.alloc.emulator_regs_mut().clear();
  }
  pub(super) fn free_pointers(&mut self) {
    let offset = self.asm.emit_addq_ir(self.alloc.emulator_ptrs().0, X64Reg::RSP);
    *self.alloc.emulator_ptrs_mut() += offset;
  }
  pub(super) fn sysv_epilogue(&mut self) {
    let offset = X64Reg::callee_saved_regs().into_iter()
                                            .rev()
                                            .map(|r| self.asm.emit_popq_r(r))
                                            .sum::<StackOffset>();
    *self.alloc.native_ptrs_mut() += offset;
  }
}