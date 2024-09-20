use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

pub type SharedPtr<T> = Arc<Mutex<T>>;
pub type SharedRWPtr<T> = Arc<RwLock<T>>;
pub struct SharePtrFactory;
impl SharePtrFactory {
    pub fn new_shared_ptr<T>(data: T) -> SharedPtr<T> {
        Arc::new(Mutex::new(data))
    }
    pub fn new_shared_rw_ptr<T>(data: T) -> SharedRWPtr<T> {
        Arc::new(RwLock::new(data))
    }
}