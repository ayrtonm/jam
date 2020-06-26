use crate::EmuReg;
use crate::GenericValue;
use crate::JITValue;
use crate::Transfer;
use crate::MultiTransfer;
use crate::Direction;
use crate::Idx;
use crate::IdxType;
use crate::alloc::Allocator;

impl Allocator {
  fn max_flag(&self) -> IdxType {
    self.mappings
        .right_values()
        .filter(|&v| {
          match v {
            JITValue::Flags(_) => true,
            _ => false,
          }
        })
        .map(|&v| {
          match v {
            JITValue::Flags(x) => x.0 + 1,
            _ => unreachable!(""),
          }
        })
        .max()
        .unwrap_or(0)
  }
  fn min_flag(&self) -> IdxType {
    self.mappings
        .right_values()
        .filter(|&v| {
          match v {
            JITValue::Flags(_) => true,
            _ => false,
          }
        })
        .map(|&v| {
          match v {
            JITValue::Flags(x) => x.0,
            _ => unreachable!(""),
          }
        })
        .min()
        .unwrap_or(0)
  }
  fn max_write(&self) -> IdxType {
    self.mappings
        .right_values()
        .filter(|&v| {
          match v {
            JITValue::DelayedWrite(_,_) => true,
            _ => false,
          }
        })
        .map(|&v| {
          match v {
            JITValue::DelayedWrite(_,x) => x.0 + 1,
            _ => unreachable!(""),
          }
        })
        .min()
        .unwrap_or(0)
  }
  fn min_write(&self) -> IdxType {
    self.mappings
        .right_values()
        .filter(|&v| {
          match v {
            JITValue::DelayedWrite(_,_) => true,
            _ => false,
          }
        })
        .map(|&v| {
          match v {
            JITValue::DelayedWrite(_,x) => x.0,
            _ => unreachable!(""),
          }
        })
        .min()
        .unwrap_or(0)
  }
  pub fn bind_flags(&mut self) -> MultiTransfer {
    let flag_idx = Idx(self.max_flag());
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
    self.mappings.insert(reg, JITValue::Flags(flag_idx));
    transfers.push(Transfer {
      reg: reg,
      other: GenericValue::JITValue(JITValue::Flags(flag_idx)),
      dir: Direction::ToReg,
    });
    MultiTransfer(transfers)
  }
  pub fn unbind_flags(&mut self) -> MultiTransfer {
    let flag_idx = Idx(self.min_flag());
    MultiTransfer(match self.mappings.get_by_right(&JITValue::Flags(flag_idx)) {
      Some(&reg) => {
        let transfers = vec![Transfer {
          reg: reg,
          other: GenericValue::JITValue(JITValue::Flags(flag_idx)),
          dir: Direction::FromReg,
        }];
        self.mappings.remove_by_left(&reg);
        transfers
      },
      None => vec![],
    })
  }
  fn write_count(&self) -> IdxType {
    self.mappings
        .right_values()
        .filter(|&v| {
          match v {
            JITValue::DelayedWrite(_,_) => true,
            _ => false,
          }
        })
        .count()
  }
  pub fn bind_delayed_write(&mut self, emu_reg: EmuReg) -> MultiTransfer {
    let idx = Idx(self.max_write());
    let reg = self.prioritized_regs().pop_front().expect("");
    let mut transfers = Vec::new();
    self.mappings
        .get_by_left(&reg)
        .map(|&prev_value| {
          transfers.push(Transfer {
            reg: reg,
            other: GenericValue::JITValue(prev_value),
            dir: Direction::FromReg,
          })
        });
    self.mappings.insert(reg, JITValue::DelayedWrite(emu_reg, idx));
    MultiTransfer(transfers)
  }
  pub fn get_delayed_write(&self, emu_reg: EmuReg) -> Option<&JITValue> {
    self.mappings
        .right_values()
        .filter(|&v| {
          match v {
            JITValue::DelayedWrite(other_emu_reg,_) => emu_reg == *other_emu_reg,
            _ => false,
          }
        })
        .next()
  }
  pub fn process_delayed_write(&mut self) -> MultiTransfer {
    let mut transfers = Vec::new();
    if self.write_count() != 0 {
      let idx = Idx(self.min_write());
      match self.mappings
          .iter()
          .filter(|(_, &v)| {
            match v {
              JITValue::DelayedWrite(_, n) => n == idx,
              _ => false,
            }
          })
          .next() {
        Some((&x64_reg, &v)) => {
          let other_reg = *match v {
            JITValue::DelayedWrite(r,_) => {
              match self.value_to_reg(&JITValue::EmuReg(r)) {
                Some(mapped_reg) => mapped_reg,
                None => {
                  transfers.append(&mut self.bind_value(JITValue::EmuReg(r)).0);
                  self.value_to_reg(&JITValue::EmuReg(r)).expect("")
                },
              }
            }
            _ => unreachable!(""),
          };
          self.mappings.remove_by_left(&x64_reg);
          transfers.push(Transfer {
            reg: x64_reg,
            other: GenericValue::X64Reg(other_reg),
            dir: Direction::FromReg,
          });
        },
        None => (),
      }
    };
    MultiTransfer(transfers)
  }
}
