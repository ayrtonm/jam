use crate::recompiler::Recompiler;
use crate::JITValue;
use crate::Label;
use crate::StackOffset;
use crate::X64Reg;

impl Recompiler {
    pub fn call_label(&mut self, label: Label) {
        stack!(self, self.asm.emit_call_label(label));
    }

    pub fn call_ptr_with_ret(&mut self, ptr_idx: usize) {
        self.sysv_caller_prologue_with_ret();
        let misalignment = self.alloc.full_stack().0 % 16;
        let align = 16 - misalignment;
        stack!(self, self.asm.emit_addq_ir(-align, X64Reg::RSP));
        let offset = self.alloc.ptr_position(ptr_idx);
        trash!(self.asm.emit_callq_m_offset(X64Reg::RSP, offset));
        stack!(self, self.asm.emit_addq_ir(align, X64Reg::RSP));
        self.sysv_caller_epilogue_with_ret();
    }

    pub fn call_ptr(&mut self, ptr_idx: usize) {
        self.sysv_caller_prologue();
        let misalignment = self.alloc.full_stack().0 % 16;
        let align = 16 - misalignment;
        stack!(self, self.asm.emit_addq_ir(-align, X64Reg::RSP));
        let offset = self.alloc.ptr_position(ptr_idx);
        trash!(self.asm.emit_callq_m_offset(X64Reg::RSP, offset));
        stack!(self, self.asm.emit_addq_ir(align, X64Reg::RSP));
        self.sysv_caller_epilogue();
    }

    pub fn call(&mut self, value: JITValue) {
        let reg = self.bind_value(value);
        self.sysv_caller_prologue();
        let misalignment = self.alloc.full_stack().0 % 16;
        let align = 16 - misalignment;
        stack!(self, self.asm.emit_addq_ir(-align, X64Reg::RSP));
        trash!(self.asm.emit_callq_r(reg));
        stack!(self, self.asm.emit_addq_ir(align, X64Reg::RSP));
        self.sysv_caller_epilogue();
    }

    pub fn new_label(&mut self) -> Label {
        self.asm.new_label()
    }

    pub fn new_long_label(&mut self) -> Label {
        self.asm.new_long_label()
    }

    pub fn define_label(&mut self, label: Label) {
        self.asm.define_label(label);
    }

    pub fn jump(&mut self, label: Label) {
        self.asm.emit_jmp_label(label);
    }

    pub fn jump_if_carry(&mut self, label: Label) {
        match label.size {
            StackOffset(1) => self.asm.emit_jc_label(label),
            StackOffset(4) => self.asm.emit_jc_long_label(label),
            _ => unreachable!("Unknown label size"),
        }
    }

    pub fn jump_if_not_carry(&mut self, label: Label) {
        self.asm.emit_jnc_label(label);
    }

    pub fn jump_if_zero(&mut self, label: Label) {
        self.asm.emit_je_label(label);
    }

    pub fn jump_if_not_zero(&mut self, label: Label) {
        self.asm.emit_jne_label(label);
    }

    pub fn jump_if_signed(&mut self, label: Label) {
        self.asm.emit_js_label(label);
    }

    pub fn jump_if_not_signed(&mut self, label: Label) {
        self.asm.emit_jns_label(label);
    }

    pub fn jump_if_not_less(&mut self, label: Label) {
        self.asm.emit_jnl_label(label);
    }

    pub fn ret(&mut self) {
        stack!(self, self.asm.emit_retq());
    }
}
