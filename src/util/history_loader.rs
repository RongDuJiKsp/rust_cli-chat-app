use crate::config::buffer_size::COMMAND_HISTORY_BUFFER_SIZE;
use std::collections::VecDeque;

pub struct HistoryLoader<T: Clone> {
    history_buffer: VecDeque<T>,
    history_index: usize,
}
impl<T: Clone> HistoryLoader<T> {
    pub fn new() -> HistoryLoader<T> {
        HistoryLoader {
            history_buffer: VecDeque::with_capacity(COMMAND_HISTORY_BUFFER_SIZE),
            history_index: 0,
        }
    }
    pub fn add(&mut self, data: T) {
        self.history_buffer.push_back(data);
        self.history_index = self.history_buffer.len() - 1;
    }
    pub fn easily(&mut self) -> Option<T> {
        let t = self.history_buffer.get(self.history_index).cloned();
        if self.history_index > 0 {
            self.history_index -= 1;
        }
        t
    }
    pub fn later(&mut self) -> Option<T> {
        let t = self.history_buffer.get(self.history_index).cloned();
        if self.history_index +1< self.history_buffer.len() {
            self.history_index += 1;
        }
        t
    }
}