use std::collections::HashSet;
use std::collections::VecDeque;
use crate::X64Reg;
use crate::GenericValue;
use crate::JITValue;
use crate::Transfer;
use crate::MultiTransfer;
use crate::Direction;
use crate::alloc::Allocator;

impl Allocator {
  pub fn debug(&self) {
    for i in self.mappings.iter() {
      println!("{:?}", i);
    }
  }
  fn available_regs(&self) -> VecDeque<X64Reg> {
    let used_regs = self.mappings
                        .left_values()
                        .collect::<HashSet<_>>();
    X64Reg::free_regs().iter()
                       .filter(|r| !used_regs.contains(r))
                       .map(|&r| r)
                       .collect::<VecDeque<_>>()
  }
  fn prioritized_regs(&self) -> VecDeque<X64Reg> {
    let available_regs = self.available_regs().into_iter().collect::<HashSet<_>>();
    let mut unavailable_regs = X64Reg::free_regs().iter()
                                                  .filter(|r| !available_regs.contains(r))
                                                  .filter(|r| {
                                                    match self.mappings.get_by_left(r) {
                                                      Some(JITValue::Flags) => false,
                                                      _ => true,
                                                    }
                                                  })
                                                  .map(|&r| r)
                                                  .collect::<VecDeque<_>>();
    let mut prioritized_regs = self.available_regs();
    prioritized_regs.append(&mut unavailable_regs);
    prioritized_regs
  }
  pub fn value_to_reg(&self, value: &JITValue) -> Option<&X64Reg> {
    self.mappings.get_by_right(value)
  }
  pub fn bind_specific_reg(&mut self, value: JITValue, reg: X64Reg) -> MultiTransfer {
    let mut transfers = Vec::new();
    self.mappings
        .get_by_left(&reg)
        .map(|&prev_value| {
          transfers.push(Transfer {
            reg: reg,
            other: GenericValue::JITValue(prev_value),
            dir: Direction::FromReg,
          });
        });
    self.mappings.insert(reg, value);
    transfers.push(Transfer {
      reg: reg,
      other: GenericValue::JITValue(value),
      dir: Direction::ToReg,
    });
    MultiTransfer(transfers)
  }
  pub fn bind_value(&mut self, value: JITValue) -> MultiTransfer {
    self.bind_multivalue(&vec![value])
  }
  pub fn bind_multivalue(&mut self, values: &Vec<JITValue>) -> MultiTransfer {
    let mut transfers = Vec::new();
    let reserved_regs = values.iter()
                              .map(|v| self.mappings.get_by_right(v))
                              .filter(|r| r.is_some())
                              .map(|r| r.expect(""))
                              .collect::<HashSet<_>>();
    let mut replacement_regs = self.prioritized_regs()
                               .into_iter()
                               .filter(|r| !reserved_regs.contains(r))
                               .collect::<VecDeque<_>>();
    let unbound_values = values.iter()
                               .filter(|v| self.mappings.get_by_right(v).is_none())
                               .collect::<Vec<_>>();
    for &v in unbound_values {
      match replacement_regs.pop_front() {
        Some(replacement_reg) => {
          self.mappings
              .get_by_left(&replacement_reg)
              .map(|&prev_value| {
                transfers.push(Transfer {
                  reg: replacement_reg,
                  other: GenericValue::JITValue(prev_value),
                  dir: Direction::FromReg,
                });
              });
          self.mappings.insert(replacement_reg, v);
          transfers.push(Transfer {
            reg: replacement_reg,
            other: GenericValue::JITValue(v),
            dir: Direction::ToReg,
          });
        },
        None => panic!("Can't bind the values {:?} simultaneously", values),
      }
    }
    MultiTransfer(transfers)
  }
  pub fn unbind_emu_regs(&mut self) -> MultiTransfer {
    let transfers = self.mappings
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
                            other: GenericValue::JITValue(emu_reg),
                            dir: Direction::FromReg,
                          }
                        })
                        .collect();
    self.mappings
        .retain(|_, &v| {
          match v {
            JITValue::EmuReg(_) => false,
            _ => true,
          }
        });
    MultiTransfer(transfers)
  }
  pub fn unbind_variables(&mut self) -> MultiTransfer {
    let transfers = self.mappings
                        .iter()
                        .filter(|(_, &v)| {
                          match v {
                            JITValue::Variable(_) => true,
                            _ => false,
                          }
                        })
                        .map(|(&x64_reg, &variable)| {
                          Transfer {
                            reg: x64_reg,
                            other: GenericValue::JITValue(variable),
                            dir: Direction::FromReg,
                          }
                        })
                        .collect();
    self.mappings
        .retain(|_, &v| {
          match v {
            JITValue::Variable(_) => false,
            _ => true,
          }
        });
    MultiTransfer(transfers)
  }
  pub fn unbind_x64_reg(&mut self, reg: X64Reg) -> MultiTransfer {
    MultiTransfer(match self.mappings.get_by_left(&reg) {
      Some(&value) => {
        let transfer = Transfer {
          reg: reg,
          other: GenericValue::JITValue(value),
          dir: Direction::FromReg,
        };
        self.mappings.remove_by_left(&reg);
        vec![transfer]
      },
      None => vec![],
    })
  }
  pub fn swap_bindings(&mut self, reg1: X64Reg, reg2: X64Reg) -> MultiTransfer {
    if reg1 != reg2 {
      let val1 = self.mappings.get_by_left(&reg1).map(|&v| v);
      let val2 = self.mappings.get_by_left(&reg2).map(|&v| v);
      match val1 {
        Some(val1) => {
          self.mappings.insert(reg2, val1);
        },
        None => {
          self.mappings.remove_by_left(&reg2);
        },
      }
      match val2 {
        Some(val2) => {
          self.mappings.insert(reg1, val2);
        },
        None => {
          self.mappings.remove_by_left(&reg1);
        },
      }
      MultiTransfer(vec![Transfer {
        reg: reg1,
        other: GenericValue::X64Reg(reg2),
        dir: Direction::FromReg,
      }])
    } else {
      MultiTransfer(vec![])
    }
  }
  pub fn bind_flags(&mut self) -> MultiTransfer {
    MultiTransfer(match self.mappings.get_by_right(&JITValue::Flags) {
      Some(_) => vec![],
      _ => {
        let reg = self.prioritized_regs().pop_front().expect("");
        let mut transfers = Vec::new();
        self.mappings
            .get_by_left(&reg)
            .map(|&prev_value| {
              transfers.push(Transfer {
                reg: reg,
                other: GenericValue::JITValue(prev_value),
                dir: Direction::FromReg,
              });
            });
        self.mappings.insert(reg, JITValue::Flags);
        transfers.push(Transfer {
          reg: reg,
          other: GenericValue::JITValue(JITValue::Flags),
          dir: Direction::ToReg,
        });
        transfers
      },
    })
  }
  pub fn unbind_flags(&mut self) -> MultiTransfer {
    MultiTransfer(match self.mappings.get_by_right(&JITValue::Flags) {
      Some(&reg) => {
        let transfers = vec![Transfer {
          reg: reg,
          other: GenericValue::JITValue(JITValue::Flags),
          dir: Direction::FromReg,
        }];
        self.mappings.remove_by_left(&reg);
        transfers
      },
      None => vec![],
    })
  }
}
