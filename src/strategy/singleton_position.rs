use super::trader_register::Position;
use std::sync::{Arc, RwLock};

/// Singleton for current position
#[derive(Default)]
pub struct PositionSingleton {
    pub position_opt: Option<Position>,
}

impl PositionSingleton {
    pub fn current() -> Arc<PositionSingleton> {
        CURRENT_POSITION.with(|c| c.read().unwrap().clone())
    }
    pub fn make_current(self) {
        CURRENT_POSITION.with(|c| *c.write().unwrap() = Arc::new(self))
    }
    pub fn set_current(position: Position) {
        Self {
            position_opt: Some(position),
        }
        .make_current();
    }
}

thread_local! {
    static CURRENT_POSITION: RwLock<Arc<PositionSingleton>> = RwLock::new(Default::default());
}
