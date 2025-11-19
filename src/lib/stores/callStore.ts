import { writable, derived, get, type Readable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { historyStore, type CallHistoryEntry, type CallDirection } from "./historyStore";
import { contactsStore } from "./contactsStore";

export type CallState = "idle" | "ringing" | "connecting" | "active" | "onHold" | "ended";

export type CallDirectionType = "incoming" | "outgoing";

export interface ActiveCall {
  id: string;
  number: string;
  name: string | null;
  direction: CallDirectionType;
  state: CallState;
  startTime: Date | null; // null until call is active
  duration: number; // seconds
  isMuted: boolean;
  isOnHold: boolean;
}

// Create writable store for current call (null when idle)
const { subscribe, set, update } = writable<ActiveCall | null>(null);

// Track pending timeouts by call ID to allow cleanup
const pendingTimeouts = new Map<string, ReturnType<typeof setTimeout>[]>();

// Helper to clear all timeouts for a call
function clearCallTimeouts(callId: string) {
  const timeouts = pendingTimeouts.get(callId);
  if (timeouts) {
    timeouts.forEach((timeout) => clearTimeout(timeout));
    pendingTimeouts.delete(callId);
  }
}

// Derived stores
export const currentCall = derived({ subscribe }, ($call) => $call);

export const callState = derived({ subscribe }, ($call) => $call?.state ?? "idle");

export const isCallActive = derived({ subscribe }, ($call) => $call?.state === "active");

export const isCallRinging = derived({ subscribe }, ($call) => $call?.state === "ringing");

// Helper to find contact name by number
function findContactName(number: string): string | null {
  // Use get() to read current value without subscribing
  const contacts = get(contactsStore);
  const normalizedNumber = number.replace(/\D/g, "");
  
  for (const contact of contacts) {
    for (const phoneNumber of contact.numbers) {
      // Simple matching - remove formatting
      const normalizedContactNumber = phoneNumber.number.replace(/\D/g, "");
      if (normalizedNumber === normalizedContactNumber || normalizedNumber.endsWith(normalizedContactNumber)) {
        return contact.name;
      }
    }
  }
  return null;
}

// Store methods
export const callStore = {
  subscribe,
  currentCall,
  callState,
  isCallActive,
  isCallRinging,

  // Initiate an outbound call
  initiateCall: async (number: string) => {
    console.log("DEBUG:[CALLSTORE/INITIATE] Initiating call to:", number);
    
    // Find contact name if available
    const name = findContactName(number);
    
    try {
      // Call Tauri command to initiate call
      const callId = await invoke<string>("initiate_call", { number });
      console.log("DEBUG:[CALLSTORE/INITIATE] Call initiated successfully, CallId:", callId);
      
      // Create call object with real CallId from backend
      const call: ActiveCall = {
        id: callId,
        number,
        name,
        direction: "outgoing",
        state: "ringing",
        startTime: null,
        duration: 0,
        isMuted: false,
        isOnHold: false,
      };

      set(call);

      // Clear any existing timeouts for this call ID (shouldn't happen, but safety check)
      clearCallTimeouts(call.id);

      // Note: State transitions (ringing → connecting → active) will be handled
      // by the backend CallService when SIP responses are received.
      // For now, we keep the call in "ringing" state until backend updates it.
    } catch (error) {
      console.error("DEBUG:[CALLSTORE/INITIATE] Failed to initiate call:", error);
      
      // Handle specific error cases
      if (error instanceof Error) {
        const errorMessage = error.message.toLowerCase();
        if (errorMessage.includes("not registered") || errorMessage.includes("registration")) {
          throw new Error("Cannot initiate call: Account is not registered. Please register your SIP account first.");
        } else if (errorMessage.includes("validation")) {
          throw new Error(`Invalid phone number: ${error.message}`);
        } else {
          throw new Error(`Failed to initiate call: ${error.message}`);
        }
      } else {
        throw new Error("Failed to initiate call: Unknown error");
      }
    }
  },

  // Simulate an incoming call
  simulateIncomingCall: (number: string) => {
    console.log("DEBUG:[CALLSTORE/INCOMING] Simulating incoming call from:", number);
    
    // Find contact name if available
    const name = findContactName(number);
    
    const call: ActiveCall = {
      id: Date.now().toString(),
      number,
      name,
      direction: "incoming",
      state: "ringing",
      startTime: null,
      duration: 0,
      isMuted: false,
      isOnHold: false,
    };

    set(call);
  },

  // Answer an incoming call
  answerCall: () => {
    console.log("DEBUG:[CALLSTORE/ANSWER] Answering call");
    update((currentCall) => {
      if (currentCall && currentCall.state === "ringing") {
        return {
          ...currentCall,
          state: "active",
          startTime: new Date(),
        };
      }
      return currentCall;
    });
  },

  // Decline an incoming call
  declineCall: () => {
    console.log("DEBUG:[CALLSTORE/DECLINE] Declining call");
    update((currentCall) => {
      if (currentCall) {
        // Clear any pending timeouts for this call
        clearCallTimeouts(currentCall.id);
        
        // Add to history as missed call
        historyStore.addEntry({
          name: currentCall.name,
          number: currentCall.number,
          direction: "missed",
          duration: 0,
          timestamp: new Date(),
        });
      }
      return null;
    });
  },

  // End the current call
  endCall: async () => {
    console.log("DEBUG:[CALLSTORE/END] Ending call");
    const currentCall = get({ subscribe });
    if (!currentCall) {
      console.warn("DEBUG:[CALLSTORE/END] No active call to end");
      return;
    }

    try {
      // Call backend to end the call
      await invoke("hangup_call", { call_id: currentCall.id });
      console.log("DEBUG:[CALLSTORE/END] Call ended successfully via backend");
      // Note: The backend will emit a call_state_changed event which will update the store
      // via the event listener, so we don't need to manually update here
    } catch (error) {
      console.error("DEBUG:[CALLSTORE/END] Failed to end call via backend:", error);
      // Fallback to local state update if backend call fails
      update((call) => {
        if (call) {
          // Clear any pending timeouts for this call
          clearCallTimeouts(call.id);

          // Calculate duration
          const duration = call.startTime
            ? Math.floor((Date.now() - call.startTime.getTime()) / 1000)
            : 0;

          // Add to history
          const historyDirection: CallDirection = call.direction === "incoming" ? "incoming" : "outgoing";
          historyStore.addEntry({
            name: call.name,
            number: call.number,
            direction: historyDirection,
            duration,
            timestamp: call.startTime || new Date(),
          });

          // Transition to ended state
          const endedCall = {
            ...call,
            state: "ended" as CallState,
            duration,
          };

          // Clear call after 1 second
          const clearCallTimeout = setTimeout(() => {
            set(null);
          }, 1000);
          
          // Track this timeout too (though it's less critical)
          pendingTimeouts.set(call.id, [clearCallTimeout]);

          return endedCall;
        }
        return null;
      });
    }
  },

  // Toggle mute state
  toggleMute: async () => {
    console.log("DEBUG:[CALLSTORE/MUTE] Toggling mute");
    const currentCall = get({ subscribe });
    if (!currentCall || (currentCall.state !== "active" && currentCall.state !== "onHold")) {
      console.warn("DEBUG:[CALLSTORE/MUTE] Cannot mute - call not in active/onHold state");
      return;
    }

    const newMutedState = !currentCall.isMuted;
    try {
      // Call backend to set mute state
      await invoke("mute_call", { call_id: currentCall.id, muted: newMutedState });
      console.log("DEBUG:[CALLSTORE/MUTE] Mute state set successfully via backend");
      // Update local state
      update((call) => {
        if (call && (call.state === "active" || call.state === "onHold")) {
          return { ...call, isMuted: newMutedState };
        }
        return call;
      });
    } catch (error) {
      console.error("DEBUG:[CALLSTORE/MUTE] Failed to set mute state via backend:", error);
      // Fallback to local state update if backend call fails
      update((call) => {
        if (call && (call.state === "active" || call.state === "onHold")) {
          return { ...call, isMuted: newMutedState };
        }
        return call;
      });
    }
  },

  // Toggle hold state
  toggleHold: async () => {
    console.log("DEBUG:[CALLSTORE/HOLD] Toggling hold");
    const currentCall = get({ subscribe });
    if (!currentCall || (currentCall.state !== "active" && currentCall.state !== "onHold")) {
      console.warn("DEBUG:[CALLSTORE/HOLD] Cannot toggle hold - call not in active/onHold state");
      return;
    }

    const newHoldState = currentCall.state === "active";
    try {
      // Call backend to set hold state
      await invoke("hold_call", { call_id: currentCall.id, on_hold: newHoldState });
      console.log("DEBUG:[CALLSTORE/HOLD] Hold state set successfully via backend");
      // Update local state
      update((call) => {
        if (call && call.state === "active") {
          return { ...call, state: "onHold", isOnHold: true };
        } else if (call && call.state === "onHold") {
          return { ...call, state: "active", isOnHold: false };
        }
        return call;
      });
    } catch (error) {
      console.error("DEBUG:[CALLSTORE/HOLD] Failed to set hold state via backend:", error);
      // Fallback to local state update if backend call fails
      update((call) => {
        if (call && call.state === "active") {
          return { ...call, state: "onHold", isOnHold: true };
        } else if (call && call.state === "onHold") {
          return { ...call, state: "active", isOnHold: false };
        }
        return call;
      });
    }
  },
};

// Event listener for call state changes from backend
interface CallStateChangedPayload {
  call_id: string;
  state: string;
  start_time: number | null;
}

// Initialize event listener on module load
let eventListenerUnsubscribe: (() => void) | null = null;

/**
 * Initialize the call state event listener
 * This should be called once when the app starts
 */
export function initializeCallStateListener(): Promise<void> {
  return new Promise((resolve, reject) => {
    listen<CallStateChangedPayload>("call_state_changed", (event) => {
      const payload = event.payload;
      console.log("DEBUG:[CALLSTORE/EVENT] Received call_state_changed:", payload);

      update((currentCall) => {
        // Only update if this event is for the current call
        if (!currentCall || currentCall.id !== payload.call_id) {
          console.log(
            "DEBUG:[CALLSTORE/EVENT] Ignoring event - call ID mismatch or no active call",
            { currentCallId: currentCall?.id, eventCallId: payload.call_id }
          );
          return currentCall;
        }

        // Map backend state to frontend state
        const newState = payload.state as CallState;
        const startTime = payload.start_time
          ? new Date(payload.start_time)
          : currentCall.startTime;

        console.log(
          "DEBUG:[CALLSTORE/EVENT] Updating call state:",
          { from: currentCall.state, to: newState, startTime }
        );

        // Handle state transitions
        if (newState === "ended") {
          // Calculate duration before clearing
          const duration = currentCall.startTime
            ? Math.floor((Date.now() - currentCall.startTime.getTime()) / 1000)
            : 0;

          // Add to history
          const historyDirection: CallDirection =
            currentCall.direction === "incoming" ? "incoming" : "outgoing";
          historyStore.addEntry({
            name: currentCall.name,
            number: currentCall.number,
            direction: historyDirection,
            duration,
            timestamp: currentCall.startTime || new Date(),
          });

          // Clear call after 1 second
          const clearCallTimeout = setTimeout(() => {
            set(null);
          }, 1000);
          clearCallTimeouts(currentCall.id);
          pendingTimeouts.set(currentCall.id, [clearCallTimeout]);

          return {
            ...currentCall,
            state: "ended" as CallState,
            duration,
          };
        }

        // Update call with new state
        return {
          ...currentCall,
          state: newState,
          startTime,
        };
      });
    })
      .then((unsubscribe) => {
        eventListenerUnsubscribe = unsubscribe;
        console.log("DEBUG:[CALLSTORE/EVENT] Event listener initialized");
        resolve();
      })
      .catch((error) => {
        console.error("DEBUG:[CALLSTORE/EVENT] Failed to initialize event listener:", error);
        reject(error);
      });
  });
}

/**
 * Cleanup event listener
 * Should be called when the app is closing
 */
export function cleanupCallStateListener(): void {
  if (eventListenerUnsubscribe) {
    eventListenerUnsubscribe();
    eventListenerUnsubscribe = null;
    console.log("DEBUG:[CALLSTORE/EVENT] Event listener cleaned up");
  }
}

