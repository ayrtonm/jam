use std::collections::HashSet;
use crate::X64Reg;
use crate::JITValue;
use crate::Transfer;
use crate::Direction;
use crate::alloc::Allocator;

//TODO: create a method to bind multiple values simultaneously
// right now I might do something like the following before executing each instruction
//   self.bind_value(Rs)
//   self.bind_value(Rt)
//   self.bind_value(Rd)
// but Rt may unbind Rs and Rd may unbind Rs or Rt if all other registers are bound
// with the current replacement strategy (always replace rax) this will always happen
// I could try solving this by improving the replacement strategy to make sure that
// the last n bound values remain bound, but then this turns into one of those details
// you have to know to use jam, so from usability perspective it's probably better to
// make a separate method
//TODO: also I should definitely pick a better replacement strategy
impl Allocator {
  pub fn debug(&self) {
    for i in self.mappings.iter() {
      println!("{:?}", i);
    }
  }
  //TODO: prioritize registers to improve the replacement strategy
  //e.g. leave rdi, rsi, rdx for last since they function arguments
  //also leave r8-r15 for last since they may require instruction prefixes
  fn free_regs(&self) -> Vec<X64Reg> {
    let all_regs = X64Reg::free_regs().into_iter().collect::<HashSet<_>>();
    let used_regs = self.mappings.left_values().cloned().collect::<HashSet<_>>();
    all_regs.difference(&used_regs).cloned().collect::<Vec<_>>()
  }
  pub fn value_to_reg(&self, value: &JITValue) -> Option<&X64Reg> {
    self.mappings.get_by_right(value)
  }
  pub fn bind(&mut self, value: JITValue, reg: X64Reg) -> Vec<Transfer> {
    match self.mappings.get_by_left(&reg) {
      Some(&prev_value) => {
        self.mappings.insert(reg, value);
        self.bind_value(prev_value)
      },
      None => {
        self.mappings.insert(reg, value);
        Vec::new()
      },
    }
  }
  pub fn bind_value(&mut self, value: JITValue) -> Vec<Transfer> {
    self.bind_multivalue(&vec![value])
  }
  //TODO: see todo on Allocator::free_regs
  pub fn bind_multivalue(&mut self, values: &Vec<JITValue>) -> Vec<Transfer> {
    let preserve_regs = values.iter()
                              .map(|v| self.mappings.get_by_right(v))
                              .filter(|r| r.is_some())
                              .map(|o| o.expect(""))
                              .cloned()
                              .collect::<HashSet<_>>();
    let all_regs = X64Reg::free_regs().into_iter()
                                      .collect::<HashSet<_>>();
    let mut replacement_regs = all_regs.difference(&preserve_regs)
                                       .cloned()
                                       .collect::<Vec<_>>();
    let mut transfers = Vec::new();
    for &v in values {
      if !self.mappings.contains_right(&v) {
        match self.free_regs().pop() {
          Some(free_reg) => {
            self.mappings.insert(free_reg, v);
            transfers.push(Transfer {
              reg: free_reg,
              value: v,
              dir: Direction::LoadValue,
            });
          },
          None => {
            let replace_reg = replacement_regs.pop().expect("");
            let old_value = *self.mappings.get_by_left(&replace_reg).expect("");
            self.mappings.insert(replace_reg, v);
            transfers.push(Transfer {
                reg: replace_reg,
                value: old_value,
                dir: Direction::StoreValue,
            });
            transfers.push(Transfer {
                reg: replace_reg,
                value: v,
                dir: Direction::LoadValue,
            });
          },
        }
      };
    };
    transfers
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
  pub fn swap_bindings(&mut self, reg1: X64Reg, reg2: X64Reg) {
    let val1 = self.mappings.get_by_left(&reg1).map(|&v| v);
    let val2 = self.mappings.get_by_left(&reg2).map(|&v| v);
    match val1 {
      Some(val) => {
        self.mappings.insert(reg2, val);
      },
      None => {
        self.mappings.remove_by_left(&reg2);
      },
    }
    match val2 {
      Some(val) => {
        self.mappings.insert(reg1, val);
      },
      None => {
        self.mappings.remove_by_left(&reg1);
      },
    }
  }
}
