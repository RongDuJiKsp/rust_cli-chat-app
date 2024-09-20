use std::sync::Arc;
use tokio::sync::RwLock;
use crate::entity::alias::sync::{PtrFac, SharedRWPtr};

#[derive(Clone)]
pub struct AppEventLoopContext {
    pub event_looping: SharedRWPtr<bool>,
    pub screen_event_looping: SharedRWPtr<bool>,
    pub connect_event_looping: SharedRWPtr<bool>,
}
impl AppEventLoopContext {
    pub fn init() -> Self {
        Self {
            event_looping: PtrFac::shared_rw_ptr(true),
            screen_event_looping: PtrFac::shared_rw_ptr(true),
            connect_event_looping: PtrFac::shared_rw_ptr(true),
        }
    }
    pub async fn close(&self) {
        *self.event_looping.write().await = false;
        *self.screen_event_looping.write().await = false;
        *self.connect_event_looping.write().await = false;
    }
}
