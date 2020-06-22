use crate::JITValue;
use crate::X64Reg;
use crate::ArgNumber;
use crate::recompiler::Recompiler;

impl Recompiler {
  fn set_argn(&mut self, value: JITValue, n: ArgNumber) {
    let arg_reg = X64Reg::argn_reg(n);
    bind!(self, match self.alloc.value_to_reg(&value) {
      Some(&value_reg) => {
        self.alloc.swap_bindings(value_reg, arg_reg)
      },
      None => {
        self.alloc.bind_specific_reg(value, arg_reg)
      },
    })
  }
  pub fn zero_arg1(&mut self) {
    let arg1_reg = X64Reg::argn_reg(ArgNumber::Arg1);
    bind!(self, self.alloc.unbind_x64_reg(arg1_reg));
    self.asm.emit_xorl_rr(arg1_reg, arg1_reg);
  }
  pub fn zero_arg2(&mut self) {
    let arg2_reg = X64Reg::argn_reg(ArgNumber::Arg2);
    bind!(self, self.alloc.unbind_x64_reg(arg2_reg));
    self.asm.emit_xorl_rr(arg2_reg, arg2_reg);
  }
  pub fn zero_arg3(&mut self) {
    let arg3_reg = X64Reg::argn_reg(ArgNumber::Arg3);
    bind!(self, self.alloc.unbind_x64_reg(arg3_reg));
    self.asm.emit_xorl_rr(arg3_reg, arg3_reg);
  }
  pub fn zero_arg4(&mut self) {
    let arg4_reg = X64Reg::argn_reg(ArgNumber::Arg4);
    bind!(self, self.alloc.unbind_x64_reg(arg4_reg));
    self.asm.emit_xorl_rr(arg4_reg, arg4_reg);
  }
  pub fn zero_arg5(&mut self) {
    let arg5_reg = X64Reg::argn_reg(ArgNumber::Arg5);
    bind!(self, self.alloc.unbind_x64_reg(arg5_reg));
    self.asm.emit_xorl_rr(arg5_reg, arg5_reg);
  }
  pub fn zero_arg6(&mut self) {
    let arg6_reg = X64Reg::argn_reg(ArgNumber::Arg6);
    bind!(self, self.alloc.unbind_x64_reg(arg6_reg));
    self.asm.emit_xorl_rr(arg6_reg, arg6_reg);
  }
  pub fn set_arg1(&mut self, value: JITValue) {
    self.set_argn(value, ArgNumber::Arg1);
  }
  pub fn set_arg2(&mut self, value: JITValue) {
    self.set_argn(value, ArgNumber::Arg2);
  }
  pub fn set_arg3(&mut self, value: JITValue) {
    self.set_argn(value, ArgNumber::Arg3);
  }
  pub fn set_arg4(&mut self, value: JITValue) {
    self.set_argn(value, ArgNumber::Arg4);
  }
  pub fn set_arg5(&mut self, value: JITValue) {
    self.set_argn(value, ArgNumber::Arg5);
  }
  pub fn set_arg6(&mut self, value: JITValue) {
    self.set_argn(value, ArgNumber::Arg6);
  }
}

