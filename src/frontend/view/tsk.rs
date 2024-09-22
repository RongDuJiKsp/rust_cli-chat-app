use crate::frontend::view::ctx::PrinterCtx;

pub struct WriteToBufTask<'a> {
    to: Vec<String>,
    cls: bool,
    this: &'a PrinterCtx,
}
impl<'a> WriteToBufTask<'a> {
    pub(super) fn new(this: &'a PrinterCtx) -> WriteToBufTask {
        Self {
            to: Vec::new(),
            cls: false,
            this,
        }
    }
    pub fn with_cls(mut self) -> Self {
        self.cls = true;
        self
    }
    pub fn with_output(mut self, s: String) -> Self {
        self.to.push(s);
        self
    }
    pub fn with_many(mut self, mut s: Vec<String>) -> Self {
        self.to.append(&mut s);
        self
    }
    pub async fn run(self) {
        let mut io = self.this.screen_buffer.write().await;
        if self.cls {
            io.clear();
        }
        for s in self.to {
            io.push_back(s);
        }
    }
}
