use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppEventLoopContext {
    pub event_looping: Arc<RwLock<bool>>,
    pub screen_event_looping: Arc<RwLock<bool>>,
    pub connect_event_looping: Arc<RwLock<bool>>,
}
impl AppEventLoopContext {
    pub fn init() -> Self {
        Self {
            event_looping: Arc::new(RwLock::new(true)),
            screen_event_looping: Arc::new(RwLock::new(true)),
            connect_event_looping: Arc::new(RwLock::new(true)),
        }
    }
    pub async fn close(&self) {
        *self.event_looping.write().await = false;
        *self.screen_event_looping.write().await = false;
        *self.connect_event_looping.write().await = false;
    }
}
