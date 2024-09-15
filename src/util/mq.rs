use tokio::sync::mpsc;
pub type MessageProvider = mpsc::Sender<ApplicationEvent>;
pub type MessageCustomer = mpsc::Receiver<ApplicationEvent>;
pub struct ApplicationEvent {}
pub struct MessageQueue;
impl MessageQueue {
    pub fn new(size: usize) -> (MessageProvider, MessageCustomer) {
        let (tx, rx) = mpsc::channel(size);
        (tx, rx)
    }
}