use super::position_register::PositionRegister;
use std::sync::{Arc, RwLock};

/// Singleton for current position
#[derive(Default)]
pub struct PositionRegisterSingleton {
    pub position_opt: Option<PositionRegister>,
}

impl PositionRegisterSingleton {
    pub fn current() -> Arc<PositionRegisterSingleton> {
        CURRENT_POSITION.with(|c| c.read().unwrap().clone())
    }
    pub fn make_current(self) {
        CURRENT_POSITION.with(|c| *c.write().unwrap() = Arc::new(self))
    }
    pub fn set_current(position: PositionRegister) {
        Self {
            position_opt: Some(position),
        }
        .make_current();
    }
}

thread_local! {
    static CURRENT_POSITION: RwLock<Arc<PositionRegisterSingleton>> = RwLock::new(Default::default());
}
