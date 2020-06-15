use std::collections::HashSet;
use bimap::BiHashMap;
use crate::X64Reg;
use crate::EmuReg;
use crate::JITValue;
use crate::StackOffset;
use crate::StackOffsetType;
use crate::Transfer;
use crate::Direction;

pub(super) struct Allocator {
  mappings: BiHashMap<X64Reg, JITValue>,
  stack: StackOffset,
  emulator_regs: Vec<EmuReg>,
  emulator_ptrs: StackOffset,
  native_ptrs: StackOffset,
}

impl Allocator {
  pub fn new() -> Self {
    let mappings = BiHashMap::<X64Reg, JITValue>::new();
    let stack = StackOffset(0);
    let emulator_regs = Vec::new();
    let emulator_ptrs = StackOffset(0);
    let native_ptrs = StackOffset(8);
    Allocator {
      mappings,
      stack,
      emulator_regs,
      emulator_ptrs,
      native_ptrs,
    }
  }
  pub fn value_to_reg(&self, value: &JITValue) -> Option<&X64Reg> {
    self.mappings.get_by_right(value)
  }
  fn free_regs(&self) -> Vec<X64Reg> {
    let all_regs = X64Reg::free_regs().into_iter().collect::<HashSet<_>>();
    let used_regs = self.mappings.left_values().cloned().collect::<HashSet<_>>();
    all_regs.difference(&used_regs).cloned().collect::<Vec<_>>()
  }
  pub fn load_value(&mut self, value: JITValue) -> Option<Vec<Transfer>> {
    if !self.mappings.contains_right(&value) {
      match self.free_regs().pop() {
        Some(free_reg) => {
          self.mappings.insert(free_reg, value);
          Some(vec![Transfer {
            reg: free_reg,
            value,
            dir: Direction::LoadValue,
          }])
        },
        None => {
          //TODO: choose a better register replacement strategy
          let replace_reg = X64Reg::RAX;
          let old_value = *self.mappings.get_by_left(&replace_reg).expect("");
          self.mappings.insert(replace_reg, value);
          let transfers = vec![
            Transfer {
              reg: replace_reg,
              value: old_value,
              dir: Direction::StoreValue,
            },
            Transfer {
              reg: replace_reg,
              value,
              dir: Direction::LoadValue,
            },
          ];
          Some(transfers)
        },
      }
    } else {
      None
    }
  }
  pub fn reg_position(&self, reg: &EmuReg) -> StackOffset {
    self.full_stack() - reg.position
  }
  pub fn value_position(&self, value: &JITValue) -> StackOffset {
    match value {
      JITValue::EmuReg(reg) => self.full_stack() - reg.position,
      JITValue::Variable(var) => self.full_stack() - var.position,
    }
  }
  pub fn ptr_position(&self, idx: usize) -> StackOffset {
    self.full_stack() - self.native_ptrs - self.emulator_ptrs + StackOffset(idx as StackOffsetType * 8)
  }
  pub fn full_stack(&self) -> StackOffset {
    self.stack +
    self.emulator_regs.iter().map(|r| r.size).sum() +
    self.emulator_ptrs +
    self.native_ptrs
  }
  pub fn stack(&self) -> StackOffset {
    self.stack
  }
  pub fn stack_mut(&mut self) -> &mut StackOffset {
    &mut self.stack
  }
  pub fn emulator_regs(&self) -> &Vec<EmuReg> {
    &self.emulator_regs
  }
  pub fn emulator_regs_mut(&mut self) -> &mut Vec<EmuReg> {
    &mut self.emulator_regs
  }
  pub fn emulator_ptrs(&self) -> StackOffset {
    self.emulator_ptrs
  }
  pub fn emulator_ptrs_mut(&mut self) -> &mut StackOffset {
    &mut self.emulator_ptrs
  }
  pub fn native_ptrs(&self) -> StackOffset {
    self.native_ptrs
  }
  pub fn native_ptrs_mut(&mut self) -> &mut StackOffset {
    &mut self.native_ptrs
  }
}
