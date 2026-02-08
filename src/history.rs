use rustyline::Result;
use rustyline::history::{History, SearchDirection, SearchResult};
use std::borrow::Cow;
use std::collections::VecDeque;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
const HISTORY_MAX: usize = 1000;

pub struct ShellHistory {
    buffer: VecDeque<String>,
    size: usize,
    capacity: usize,
}

impl ShellHistory {
    pub fn new() -> Self {
        ShellHistory {
            buffer: VecDeque::new(),
            size: 0,
            capacity: HISTORY_MAX,
        }
    }

    pub fn push(&mut self, cmd: String) {
        if self.size >= self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(cmd);
        self.size += 1;
    }

    pub fn len(&self) -> usize {
        self.size
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn iter(&self) -> std::collections::vec_deque::Iter<'_, String> {
        self.buffer.iter()
    }
}

impl Default for ShellHistory {
    fn default() -> Self {
        Self::new()
    }
}

impl History for ShellHistory {
    fn get(&self, index: usize, _dir: SearchDirection) -> Result<Option<SearchResult<'_>>> {
        Ok(self.buffer.get(index).map(|entry| SearchResult {
            entry: Cow::Borrowed(entry),
            idx: index,
            pos: 0,
        }))
    }

    fn add(&mut self, line: &str) -> Result<bool> {
        if line.is_empty() {
            return Ok(false);
        }
        if self.size >= self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(line.to_string());
        self.size += 1;
        Ok(true)
    }

    fn add_owned(&mut self, line: String) -> Result<bool> {
        if line.is_empty() {
            return Ok(false);
        }
        if self.size >= self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(line);
        self.size += 1;
        Ok(true)
    }

    fn len(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn set_max_len(&mut self, max_len: usize) -> Result<()> {
        self.capacity = max_len;
        while self.buffer.len() > max_len {
            self.buffer.pop_front();
            self.size -= 1;
        }
        Ok(())
    }

    fn ignore_dups(&mut self, _yes: bool) -> Result<()> {
        Ok(())
    }

    fn ignore_space(&mut self, _yes: bool) {}

    fn save(&mut self, path: &Path) -> Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        for entry in &self.buffer {
            writeln!(writer, "{}", entry)?;
        }
        Ok(())
    }

    fn append(&mut self, path: &Path) -> Result<()> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        let mut writer = BufWriter::new(file);
        for entry in &self.buffer {
            writeln!(writer, "{}", entry)?;
        }
        Ok(())
    }

    fn load(&mut self, path: &Path) -> Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            self.add(&line)?;
        }
        Ok(())
    }

    fn clear(&mut self) -> Result<()> {
        self.buffer.clear();
        self.size = 0;
        Ok(())
    }

    fn search(
        &self,
        term: &str,
        start: usize,
        dir: SearchDirection,
    ) -> Result<Option<SearchResult<'_>>> {
        if term.is_empty() || start >= self.len() {
            return Ok(None);
        }

        match dir {
            SearchDirection::Reverse => {
                let skip_count = self.len().saturating_sub(1).saturating_sub(start);
                for (idx, entry) in self.buffer.iter().rev().skip(skip_count).enumerate() {
                    if let Some(pos) = entry.find(term) {
                        return Ok(Some(SearchResult {
                            idx: start.saturating_sub(idx),
                            entry: Cow::Borrowed(entry),
                            pos,
                        }));
                    }
                }
                Ok(None)
            }
            SearchDirection::Forward => {
                for (idx, entry) in self.buffer.iter().skip(start).enumerate() {
                    if let Some(pos) = entry.find(term) {
                        return Ok(Some(SearchResult {
                            idx: idx + start,
                            entry: Cow::Borrowed(entry),
                            pos,
                        }));
                    }
                }
                Ok(None)
            }
        }
    }

    fn starts_with(
        &self,
        term: &str,
        start: usize,
        dir: SearchDirection,
    ) -> Result<Option<SearchResult<'_>>> {
        if term.is_empty() || start >= self.len() {
            return Ok(None);
        }

        match dir {
            SearchDirection::Reverse => {
                let skip_count = self.len().saturating_sub(1).saturating_sub(start);
                for (idx, entry) in self.buffer.iter().rev().skip(skip_count).enumerate() {
                    if entry.starts_with(term) {
                        return Ok(Some(SearchResult {
                            idx: start.saturating_sub(idx),
                            entry: Cow::Borrowed(entry),
                            pos: term.len(),
                        }));
                    }
                }
                Ok(None)
            }
            SearchDirection::Forward => {
                for (idx, entry) in self.buffer.iter().skip(start).enumerate() {
                    if entry.starts_with(term) {
                        return Ok(Some(SearchResult {
                            idx: idx + start,
                            entry: Cow::Borrowed(entry),
                            pos: term.len(),
                        }));
                    }
                }
                Ok(None)
            }
        }
    }
}
