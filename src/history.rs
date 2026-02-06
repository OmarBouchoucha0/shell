use std::collections::VecDeque;

const HISTORY_MAX: u32 = 1000;

pub struct History {
    buffer: VecDeque<String>,
    size: u32,
    capacity: u32,
}

impl History {
    pub fn new() -> Self {
        History {
            buffer: VecDeque::new(),
            size: 0,
            capacity: HISTORY_MAX,
        }
    }

    pub fn push(&mut self, cmd: String) {
        if self.size >= self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(format!("{0} {cmd}", self.size));
        self.size += 1;
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> u32 {
        self.size
    }

    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    pub fn iter(&self) -> std::collections::vec_deque::Iter<'_, String> {
        self.buffer.iter()
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}
