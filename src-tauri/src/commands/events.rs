// Event emission helpers for Tauri events
// This module provides utilities for emitting events from backend to frontend

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{AppHandle, Emitter};

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
#[derive(Clone)]
pub struct EventEmitter {
    app: Arc<AppHandle>,
}

impl EventEmitter {
    /// Create a new EventEmitter from an AppHandle
    pub fn new(app: AppHandle) -> Self {
        Self {
            app: Arc::new(app),
        }
    }

    /// Emit a call_state_changed event to the frontend
    ///
    /// # Arguments
    /// * `call_id` - Call identifier
    /// * `state` - New call state as string
    /// * `start_time` - Optional start time as Unix timestamp (milliseconds)
    pub fn emit_call_state_changed(
        &self,
        call_id: String,
        state: String,
        start_time: Option<u64>,
    ) {
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

