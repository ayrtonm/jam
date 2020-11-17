use memmap::Mmap;

pub struct JITFn {
    function: fn(),
    mmap: Mmap,
}

impl JITFn {
    pub fn new(function: fn(), mmap: Mmap) -> Self {
        JITFn { function, mmap }
    }

    pub fn run(&self) {
        (self.function)();
    }

    pub fn size(&self) -> usize {
        self.mmap.len()
    }
}
