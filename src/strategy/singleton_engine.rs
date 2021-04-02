use std::sync::{Arc, RwLock};

use rhai::{Engine, Scope, AST};

/// Singleton for engine script
#[derive(Default)]
pub struct EngineSingleton {
    pub engine_scope: Option<(Engine, Scope<'static>, AST)>,
}

impl EngineSingleton {
    pub fn current() -> Arc<EngineSingleton> {
        CURRENT_ENGINE.with(|c| c.read().unwrap().clone())
    }
    pub fn make_current(self) {
        CURRENT_ENGINE.with(|c| *c.write().unwrap() = Arc::new(self))
    }
    pub fn set_current(engine_scope: (Engine, Scope<'static>, AST)) {
        Self {
            engine_scope: Some(engine_scope),
        }
        .make_current();
    }
}

thread_local! {
    static CURRENT_ENGINE: RwLock<Arc<EngineSingleton>> = RwLock::new(Default::default());
}
