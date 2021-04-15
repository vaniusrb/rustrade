use super::script_state::ScriptState;
use std::sync::Arc;
use std::sync::RwLock;

/// Singleton for current state of script trend provider
#[derive(Default)]
pub struct ScriptStateSingleton {
    pub script_state_opt: Option<ScriptState>,
}

impl ScriptStateSingleton {
    pub fn current() -> Arc<ScriptStateSingleton> {
        CURRENT_SCRIPT_STATE.with(|c| c.read().unwrap().clone())
    }

    pub fn make_current(self) {
        CURRENT_SCRIPT_STATE.with(|c| *c.write().unwrap() = Arc::new(self))
    }

    pub fn set_current(script_state: ScriptState) {
        Self {
            script_state_opt: Some(script_state),
        }
        .make_current();
    }
}

thread_local! {
    static CURRENT_SCRIPT_STATE: RwLock<Arc<ScriptStateSingleton>> = RwLock::new(Default::default());
}
