use std::io;
use std::mem;
use memmap::MmapMut;
use crate::jit_fn::JITFn;
use crate::Direction;
use crate::Label;
use crate::StackOffset;
use crate::StackOffsetType;
use crate::Transfer;
use crate::X64Reg;
use crate::asm::Assembler;

impl Assembler {
  pub fn new() -> Self {
    let buffer = Vec::new();
    let label_counter = 0;
    let labels_used = Default::default();
    let labels_defined = Default::default();
    Assembler {
      buffer,
      label_counter,
      labels_used,
      labels_defined,
    }
  }
  pub fn new_label(&mut self) -> Label {
    let label = Label {
      id: self.label_counter,
      size: StackOffset(1),
    };
    self.label_counter += 1;
    label
  }
  pub fn new_long_label(&mut self) -> Label {
    let label = Label {
      id: self.label_counter,
      size: StackOffset(4),
    };
    self.label_counter += 1;
    label
  }
  pub fn define_label(&mut self, label: Label) {
    self.labels_defined.insert(label, StackOffset(self.buffer.len() as StackOffsetType));
  }
  pub fn new_defined_label(&mut self) -> Label {
    let label = self.new_label();
    self.define_label(label);
    label
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
        (Direction::StoreValue, StackOffset(8)) => {
          self.emit_movq_rm_offset(t.reg, X64Reg::RSP, offset);
        },
        _ => todo!("{:?} {:?}", t.dir, size),
      }
    }
  }
  pub fn resolve_label_addresses(&mut self) {
    for (&loc, &label) in self.labels_used.iter() {
      match self.labels_defined.get(&label) {
        Some(&def) => {
          let rel_distance = def - loc - label.size;
          match label.size {
            StackOffset(1) => {
              self.buffer[loc.0 as usize] = rel_distance.0 as u8;
            },
            StackOffset(4) => {
              self.buffer[loc.0 as usize] = (rel_distance.0 & 0xff) as u8;
              self.buffer[loc.0 as usize + 1] = ((rel_distance.0 >> 8) & 0xff) as u8;
              self.buffer[loc.0 as usize + 2] = ((rel_distance.0 >> 16) & 0xff) as u8;
              self.buffer[loc.0 as usize + 3] = ((rel_distance.0 >> 24) & 0xff) as u8;
            },
            _ => todo!(""),
          }
        },
        None => panic!("used undefined label {:?} at {:?}", label, loc),
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
