use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
#[derive(Clone)]
pub struct PrinterCtx {
    write_buffer: Arc<Mutex<String>>,
    screen_buffer: Arc<Mutex<VecDeque<String>>>,
}
impl PrinterCtx {
    pub fn new() -> PrinterCtx {
        PrinterCtx {
            write_buffer: Arc::new(Mutex::new(String::new())),
            screen_buffer: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
}