// Event emission helpers for Tauri events
// This module provides utilities for emitting events from backend to frontend

use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Call state changed event payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallStateChangedPayload {
    /// Call identifier
    pub call_id: String,
    /// New call state (as string: "idle", "ringing", "connecting", "active", "onHold", "ended")
    pub state: String,
    /// Call start time as Unix timestamp (milliseconds), None if not started
    pub start_time: Option<u64>,
}

/// Event emitter for Tauri events
/// Wraps AppHandle to allow event emission from services
/// Uses a trait object to support both real and mock AppHandles
#[derive(Clone)]
pub struct EventEmitter {
    app: Arc<dyn EmitEvent + Send + Sync>,
}

/// Trait for emitting events (allows both real and mock AppHandles)
trait EmitEvent {
    fn emit(&self, event: &str, payload: &CallStateChangedPayload) -> Result<(), String>;
}

// Implement for real AppHandle
impl<R: tauri::Runtime> EmitEvent for tauri::AppHandle<R> {
    fn emit(&self, event: &str, payload: &CallStateChangedPayload) -> Result<(), String> {
        tauri::Emitter::emit(self, event, payload).map_err(|e| e.to_string())
    }
}

impl EventEmitter {
    /// Create a new EventEmitter from an AppHandle
    pub fn new<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> Self {
        Self {
            app: Arc::new(app) as Arc<dyn EmitEvent + Send + Sync>,
        }
    }

    /// Emit a call_state_changed event to the frontend
    ///
    /// # Arguments
    /// * `call_id` - Call identifier
    /// * `state` - New call state as string
    /// * `start_time` - Optional start time as Unix timestamp (milliseconds)
    pub fn emit_call_state_changed(&self, call_id: String, state: String, start_time: Option<u64>) {
        let payload = CallStateChangedPayload {
            call_id,
            state,
            start_time,
        };

        eprintln!(
            "DEBUG:[EVENTS/CALL_STATE] Emitting call_state_changed: call_id={}, state={}, start_time={:?}",
            payload.call_id, payload.state, payload.start_time
        );

        if let Err(e) = self.app.emit("call_state_changed", &payload) {
            eprintln!(
                "DEBUG:[EVENTS/CALL_STATE] Failed to emit call_state_changed event: {}",
                e
            );
        }
    }
}
