use crate::EmuReg;
use crate::JITValue;
use crate::StackOffset;
use crate::StackOffsetType;
use crate::X64Reg;
use bimap::BiHashMap;

mod queues;
mod registers;

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

    pub fn reg_position(&self, reg: &EmuReg) -> StackOffset {
        self.full_stack() - reg.position
    }

    pub fn value_position(&self, value: &JITValue) -> StackOffset {
        match value {
            JITValue::EmuReg(reg) => self.full_stack() - reg.position,
            JITValue::Variable(var) => self.full_stack() - var.position,
            JITValue::Flags(_) => unreachable!(""),
            JITValue::DelayedWrite(..) => unreachable!(""),
        }
    }

    pub fn ptr_position(&self, idx: usize) -> StackOffset {
        self.full_stack() - self.native_ptrs() - self.emulator_ptrs() +
            StackOffset(idx as StackOffsetType * 8)
    }

    pub fn full_stack(&self) -> StackOffset {
        self.stack() +
            self.emulator_regs.iter().map(|r| r.size).sum() +
            self.emulator_ptrs() +
            self.native_ptrs()
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
