use crate::alloc::Allocator;
use crate::Direction;
use crate::GenericValue;
use crate::JITValue;
use crate::Move;
use crate::Transfer;
use crate::X64Reg;
use std::collections::HashSet;
use std::collections::VecDeque;

impl Allocator {
    pub fn debug(&self) {
        let mut sorted_keys = self.mappings.left_values().collect::<Vec<_>>();
        sorted_keys.sort_by(|&&a, &&b| (a as u8).cmp(&(b as u8)));
        for i in sorted_keys {
            let j = self.mappings.get_by_left(&i);
            println!("{:?} <=> {:?}", i, j.unwrap());
        }
    }

    pub fn contains_reg(&self, reg: &X64Reg) -> bool {
        self.mappings.get_by_left(&reg).is_some()
    }

    fn used_regs(&self) -> HashSet<&X64Reg> {
        self.mappings.left_values().collect::<HashSet<_>>()
    }

    fn available_regs(&self) -> VecDeque<X64Reg> {
        let used_regs = self.used_regs();
        X64Reg::free_regs()
            .iter()
            .filter(|r| !used_regs.contains(r))
            .map(|&r| r)
            .collect::<VecDeque<_>>()
    }

    pub fn prioritized_regs(&self) -> VecDeque<X64Reg> {
        let available_regs = self.available_regs().into_iter().collect::<HashSet<_>>();
        let mut unavailable_regs = X64Reg::free_regs()
            .iter()
            .filter(|r| !available_regs.contains(r))
            .filter(|r| match self.mappings.get_by_left(r) {
                Some(JITValue::Flags(_)) | Some(JITValue::DelayedWrite(..)) => false,
                _ => true,
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

    pub fn bind_specific_reg(&mut self, value: JITValue, reg: X64Reg) -> Transfer {
        let mut transfers = Vec::new();
        self.mappings.get_by_left(&reg).map(|&prev_value| {
            transfers.push(Move {
                reg,
                other: GenericValue::JITValue(prev_value),
                dir: Direction::FromReg,
            });
        });
        self.mappings.insert(reg, value);
        transfers.push(Move {
            reg,
            other: GenericValue::JITValue(value),
            dir: Direction::ToReg,
        });
        Transfer(transfers)
    }

    pub fn bind_value(&mut self, value: JITValue) -> Transfer {
        self.bind_multivalue(&vec![value])
    }

    pub fn bind_multivalue(&mut self, values: &Vec<JITValue>) -> Transfer {
        let mut transfers = Vec::new();
        let reserved_regs = values
            .iter()
            .map(|v| self.mappings.get_by_right(v))
            .filter(|r| r.is_some())
            .map(|r| r.expect(""))
            .collect::<HashSet<_>>();
        let mut replacement_regs = self
            .prioritized_regs()
            .into_iter()
            .filter(|r| !reserved_regs.contains(r))
            .collect::<VecDeque<_>>();
        let unbound_values = values
            .iter()
            .filter(|v| self.mappings.get_by_right(v).is_none())
            .collect::<Vec<_>>();
        for &v in unbound_values {
            match replacement_regs.pop_front() {
                Some(replacement_reg) => {
                    self.mappings
                        .get_by_left(&replacement_reg)
                        .map(|&prev_value| {
                            transfers.push(Move {
                                reg: replacement_reg,
                                other: GenericValue::JITValue(prev_value),
                                dir: Direction::FromReg,
                            });
                        });
                    self.mappings.insert(replacement_reg, v);
                    transfers.push(Move {
                        reg: replacement_reg,
                        other: GenericValue::JITValue(v),
                        dir: Direction::ToReg,
                    });
                },
                None => panic!("Can't bind the values {:?} simultaneously", values),
            }
        }
        Transfer(transfers)
    }

    pub fn unbind_emu_regs(&mut self) -> Transfer {
        let transfers = self
            .mappings
            .iter()
            .filter(|(_, &v)| match v {
                JITValue::EmuReg(_) => true,
                _ => false,
            })
            .map(|(&x64_reg, &emu_reg)| Move {
                reg: x64_reg,
                other: GenericValue::JITValue(emu_reg),
                dir: Direction::FromReg,
            })
            .collect();
        self.mappings.retain(|_, &v| match v {
            JITValue::EmuReg(_) | JITValue::Variable(_) => false,
            _ => true,
        });
        Transfer(transfers)
    }

    pub fn swap_binding(&mut self, reg: X64Reg) -> Transfer {
        Transfer(match self.mappings.get_by_left(&reg) {
            Some(&value) => {
                let other_reg = self.prioritized_regs().pop_front().expect("");
                let transfers = vec![Move {
                    reg,
                    other: GenericValue::X64Reg(other_reg),
                    dir: Direction::FromReg,
                }];
                let other_val = self.mappings.get_by_left(&other_reg).map(|&v| v);
                self.mappings.insert(other_reg, value);
                match other_val {
                    Some(other_val) => {
                        self.mappings.insert(reg, other_val);
                    },
                    None => {
                        self.mappings.remove_by_left(&reg);
                    },
                };
                transfers
            },
            None => vec![],
        })
    }

    pub fn swap_specific_bindings(&mut self, reg1: X64Reg, reg2: X64Reg) -> Transfer {
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
            Transfer(vec![Move {
                reg: reg1,
                other: GenericValue::X64Reg(reg2),
                dir: Direction::FromReg,
            }])
        } else {
            Transfer(vec![])
        }
    }
}
