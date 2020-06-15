use std::collections::HashSet;
use crate::X64Reg;
use crate::JITValue;
use crate::Transfer;
use crate::Direction;
use crate::alloc::Allocator;

impl Allocator {
  fn free_regs(&self) -> Vec<X64Reg> {
    let all_regs = X64Reg::free_regs().into_iter().collect::<HashSet<_>>();
    let used_regs = self.mappings.left_values().cloned().collect::<HashSet<_>>();
    all_regs.difference(&used_regs).cloned().collect::<Vec<_>>()
  }
  pub fn value_to_reg(&self, value: &JITValue) -> Option<&X64Reg> {
    self.mappings.get_by_right(value)
  }
  pub fn bind_value(&mut self, value: JITValue) -> Vec<Transfer> {
    if !self.mappings.contains_right(&value) {
      match self.free_regs().pop() {
        Some(free_reg) => {
          self.mappings.insert(free_reg, value);
          vec![Transfer {
            reg: free_reg,
            value,
            dir: Direction::LoadValue,
          }]
        },
        None => {
          //TODO: choose a better register replacement strategy
          let replace_reg = X64Reg::RAX;
          let old_value = *self.mappings.get_by_left(&replace_reg).expect("");
          self.mappings.insert(replace_reg, value);
          vec![
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
          ]
        },
      }
    } else {
      Vec::new()
    }
  }
  pub fn unbind_regs(&mut self) -> Vec<Transfer> {
    self.mappings
        .iter()
        .filter(|(_, &v)| {
          match v {
            JITValue::EmuReg(_) => true,
            _ => false,
          }
        })
        .map(|(&x64_reg, &emu_reg)| {
          Transfer {
            reg: x64_reg,
            value: emu_reg,
            dir: Direction::StoreValue,
          }
        })
        .collect()
  }
}
