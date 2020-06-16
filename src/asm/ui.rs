use std::io;
use std::mem;
use memmap::MmapMut;
use crate::jit_fn::JITFn;
use crate::Direction;
use crate::StackOffset;
use crate::Transfer;
use crate::X64Reg;
use crate::asm::Assembler;

impl Assembler {
  pub fn new() -> Self {
    let buffer = Vec::new();
    Assembler { buffer }
  }
  pub fn emit_transfers(&mut self, transfers: Vec<Transfer>, stack: StackOffset) {
    for t in transfers {
      let size = t.value.size();
      let offset = stack - t.value.position();
      match (t.dir, size) {
        (Direction::LoadValue, StackOffset(4)) => {
          self.emit_movl_mr_offset(X64Reg::RSP, t.reg, offset);
        },
        (Direction::StoreValue, StackOffset(4)) => {
          self.emit_movl_rm_offset(t.reg, X64Reg::RSP, offset);
        },
        (Direction::LoadValue, StackOffset(8)) => {
          self.emit_movq_mr_offset(X64Reg::RSP, t.reg, offset);
        },
        _ => todo!("{:?} {:?}", t.dir, size),
      }
    }
  }
  pub fn assemble(self) -> io::Result<JITFn> {
    let mut mmap = MmapMut::map_anon(self.buffer.len())?;
    mmap.copy_from_slice(&self.buffer);
    let exec_mmap = mmap.make_exec()?;
    let address = exec_mmap.as_ptr();
    unsafe {
      let function = mem::transmute::<*const u8, fn()>(address);
      Ok(JITFn::new(function, exec_mmap))
    }
  }
}
