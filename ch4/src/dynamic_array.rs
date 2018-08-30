use std::boxed::Box;
use std::cmp;


const MIN_SIZE: usize = 10;
pub struct LogSaver {
    buf: Box<[Option<u64>]>,
    cap: usize,    
    pub length: usize,
}

impl LogSaver {
    pub fn new_empty() -> LogSaver {
        LogSaver{
            buf: Box::new([None; MIN_SIZE]), 
            length: 0,
            cap: MIN_SIZE
        }
    }

    fn grow(&mut self, min_cap: usize) {
        let old_cap = self.buf.len();
        let mut new_cap = old_cap + (old_cap >> 1);

        new_cap = cmp::max(new_cap, min_cap);
        new_cap = cmp::min(new_cap, usize::max_value());
        let current = self.buf.clone();
        self.cap = new_cap;

        self.buf = vec![None; new_cap].into_boxed_slice();
        self.buf[..current.len()].clone_from_slice(&current);
    }

    pub fn append(&mut self, value: u64)  {
        if self.length == self.cap {
            self.grow(self.length + 1);
        }
        self.buf[self.length] = Some(value);
        self.length += 1;
    }

    pub fn at(&mut self, index: usize) -> Option<u64> {
        if self.length > index {
            self.buf[index]
        } else {
            None
        }
    }
}