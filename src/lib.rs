#![deny(unused_must_use)]

pub mod recompiler;
pub mod jit_fn;
mod alloc;
mod asm;

//obtained by casting *const fn() as u64
type PtrType = u64;
//added/subtracted to/from %rsp
type StackOffsetType = i32;
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
#[must_use]
struct StackOffset(StackOffsetType);

//used to build x86-64 machine code buffer in Assembler
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum X64Reg {
  RAX, RCX, RDX, RBX,
  RSP, RBP, RSI, RDI,
  R8, R9, R10, R11,
  R12, R13, R14, R15,
}

//typically obtained from emulator opcodes
type EmuRegNameType = u32;
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct EmuRegName(EmuRegNameType);

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct EmuReg {
  name: EmuRegName,
  position: StackOffset,
  size: StackOffset,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Variable {
  position: StackOffset,
  size: StackOffset,
}

#[derive(Debug)]
enum GenericValue {
  JITValue(JITValue),
  X64Reg(X64Reg),
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
#[must_use]
pub enum JITValue {
  EmuReg(EmuReg),
  Variable(Variable),
  Flags,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
  ToReg,
  FromReg,
}

#[derive(Debug)]
#[must_use]
struct Transfer {
  reg: X64Reg,
  other: GenericValue,
  dir: Direction,
}

#[derive(Debug)]
#[must_use]
struct MultiTransfer(Vec<Transfer>);

enum ArgNumber {
  Arg1,
  Arg2,
  Arg3,
  Arg4,
  Arg5,
  Arg6,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
#[must_use]
pub struct Label {
  id: usize,
  size: StackOffset,
}

impl std::ops::Add for StackOffset {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    StackOffset(self.0 + other.0)
  }
}

impl std::ops::AddAssign for StackOffset {
  fn add_assign(&mut self, other: Self) {
    self.0 += other.0;
  }
}

impl std::iter::Sum for StackOffset {
  fn sum<I: Iterator<Item = StackOffset>>(iter: I) -> Self {
    iter.fold(StackOffset(0), |x,y| x + y)
  }
}

impl std::ops::Neg for StackOffset {
  type Output = Self;
  fn neg(self) -> Self {
    StackOffset(-self.0)
  }
}

impl std::ops::Sub for StackOffset {
  type Output = Self;
  fn sub(self, other: Self) -> Self {
    StackOffset(self.0 - other.0)
  }
}

impl X64Reg {
  pub fn low(&self) -> u8 {
    (*self as u8) & 0b0111
  }
  pub fn high(&self) -> u8 {
    (*self as u8) & 0b1000
  }
  pub fn is_extended(&self) -> bool {
    self.high() != 0
  }
  pub fn all_regs() -> Vec<X64Reg> {
    vec![
         X64Reg::RAX, X64Reg::RCX, X64Reg::RBP, X64Reg::RBX,
         X64Reg::R8,  X64Reg::R9,  X64Reg::R10, X64Reg::R11,
         X64Reg::R12, X64Reg::R13, X64Reg::R14, X64Reg::R15,
         X64Reg::RSP, X64Reg::RDX, X64Reg::RSI, X64Reg::RDI,
       ]
  }
  pub fn free_regs() -> Vec<X64Reg> {
    X64Reg::all_regs().into_iter().filter(|&r| r != X64Reg::RSP).collect()
  }
  pub fn callee_saved_regs() -> Vec<X64Reg> {
    vec![X64Reg::RBX, X64Reg::RSP, X64Reg::RBP,
         X64Reg::R12, X64Reg::R13, X64Reg::R14, X64Reg::R15]
  }
  pub fn caller_saved_regs() -> Vec<X64Reg> {
    vec![X64Reg::RAX, X64Reg::RCX, X64Reg::RDX, X64Reg::RSI, X64Reg::RDI,
         X64Reg::R8, X64Reg::R9, X64Reg::R10, X64Reg::R11]
  }
  pub fn argn_reg(n: ArgNumber) -> X64Reg {
    match n {
      ArgNumber::Arg1 => X64Reg::RDI,
      ArgNumber::Arg2 => X64Reg::RSI,
      ArgNumber::Arg3 => X64Reg::RDX,
      ArgNumber::Arg4 => X64Reg::RCX,
      ArgNumber::Arg5 => X64Reg::R8,
      ArgNumber::Arg6 => X64Reg::R9,
    }
  }
}

impl JITValue {
  fn position(&self) -> StackOffset {
    match self {
      JITValue::Variable(var) => var.position,
      JITValue::EmuReg(reg) => reg.position,
      JITValue::Flags => unreachable!(""),
    }
  }
  fn size(&self) -> StackOffset {
    match self {
      JITValue::Variable(var) => var.size,
      JITValue::EmuReg(reg) => reg.size,
      JITValue::Flags => unreachable!(""),
    }
  }
}
