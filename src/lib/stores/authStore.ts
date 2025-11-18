import { writable, derived, type Readable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";

export type RegistrationState = "unregistered" | "registering" | "registered" | "failed";

export interface UserInfo {
  name: string;
  email: string;
}

export interface SIPCredentials {
  server: string;
  port: string;
  protocol: "UDP" | "TCP" | "TLS";
  username: string;
  password?: string; // Not stored in mock, but available for future
}

// Mock initial data
const initialUserInfo: UserInfo = {
  name: "John Doe",
  email: "john.doe@example.com",
};

const initialSIPCredentials: SIPCredentials = {
  server: "sip.example.com",
  port: "5060",
  protocol: "UDP",
  username: "johndoe",
};

// Create writable stores
const { subscribe: subscribeRegistrationState, set: setRegistrationState, update: updateRegistrationState } =
  writable<RegistrationState>("unregistered");

const { subscribe: subscribeUserInfo, set: setUserInfo, update: updateUserInfo } =
  writable<UserInfo>(initialUserInfo);

const { subscribe: subscribeSIPCredentials, set: setSIPCredentials, update: updateSIPCredentials } =
  writable<SIPCredentials>(initialSIPCredentials);

// Derived stores
export const registrationState = derived({ subscribe: subscribeRegistrationState }, ($state) => $state);

export const userInfo = derived({ subscribe: subscribeUserInfo }, ($info) => $info);

export const sipCredentials = derived({ subscribe: subscribeSIPCredentials }, ($creds) => $creds);

export const isRegistered = derived({ subscribe: subscribeRegistrationState }, ($state) => $state === "registered");

export const isRegistering = derived({ subscribe: subscribeRegistrationState }, ($state) => $state === "registering");

// Polling interval for registration status (in milliseconds)
let statusPollInterval: ReturnType<typeof setInterval> | null = null;
const POLL_INTERVAL_MS = 2000; // Poll every 2 seconds

// Helper function to parse registration status from backend
function parseRegistrationStatus(statusString: string): RegistrationState {
  if (statusString.startsWith("failed:")) {
    return "failed";
  }
  if (statusString === "unregistered") {
    return "unregistered";
  }
  if (statusString === "registering") {
    return "registering";
  }
  if (statusString === "registered") {
    return "registered";
  }
  if (statusString === "expired") {
    return "unregistered";
  }
  // Default to unregistered if status is unknown
  return "unregistered";
}

// Start polling registration status
function startStatusPolling() {
  // Clear any existing interval
  if (statusPollInterval) {
    clearInterval(statusPollInterval);
  }

  statusPollInterval = setInterval(async () => {
    try {
      const statusString = await invoke<string>("get_registration_status");
      const newState = parseRegistrationStatus(statusString);
      const currentState = get(subscribeRegistrationState);
      
      // Only update if state changed
      if (newState !== currentState) {
        console.log(`DEBUG:[AUTHSTORE/POLL] Registration status changed: ${currentState} → ${newState}`);
        setRegistrationState(newState);
      }

      // Stop polling if we're in a terminal state (registered or failed)
      if (newState === "registered" || newState === "failed") {
        stopStatusPolling();
      }
    } catch (error) {
      console.error("DEBUG:[AUTHSTORE/POLL] Error polling registration status", error);
      // Don't stop polling on error, just log it
    }
  }, POLL_INTERVAL_MS);
}

// Stop polling registration status
function stopStatusPolling() {
  if (statusPollInterval) {
    clearInterval(statusPollInterval);
    statusPollInterval = null;
    console.log("DEBUG:[AUTHSTORE/POLL] Stopped polling registration status");
  }
}

// Helper to get current store value
function get<T>(subscribe: (run: (value: T) => void) => () => void): T {
  let value: T;
  const unsubscribe = subscribe((v) => {
    value = v;
  });
  unsubscribe();
  return value!;
}

// Store methods
export const authStore = {
  subscribe: subscribeRegistrationState,
  registrationState,
  userInfo,
  sipCredentials,
  isRegistered,
  isRegistering,

  // Register SIP account
  async register(
    server: string,
    port: number,
    protocol: "udp" | "tcp" | "tls",
    username: string,
    password: string,
    contactUri?: string,
    expires?: number
  ): Promise<void> {
    console.log("DEBUG:[AUTHSTORE/REGISTER] Starting registration", {
      server,
      port,
      protocol,
      username,
      contactUri,
      expires,
    });

    setRegistrationState("registering");

    try {
      // Call Tauri command
      const result = await invoke<string>("register_account", {
        server,
        port,
        protocol,
        username,
        password,
        contactUri: contactUri || null,
        expires: expires || null,
      });

      console.log("DEBUG:[AUTHSTORE/REGISTER] Registration initiated:", result);

      // Update SIP credentials in store
      updateSIPCredentials((current) => ({
        ...current,
        server,
        port: port.toString(),
        protocol: protocol.toUpperCase() as "UDP" | "TCP" | "TLS",
        username,
      }));

      // Start polling for registration status
      startStatusPolling();

      // Note: We don't set state to "registered" here because registration is async
      // The polling will update the state when registration completes
    } catch (error) {
      console.error("DEBUG:[AUTHSTORE/REGISTER] Registration failed", error);
      setRegistrationState("failed");
      stopStatusPolling();
      
      // Re-throw error so caller can handle it
      throw error;
    }
  },

  // Get registration status
  async getRegistrationStatus(): Promise<RegistrationState> {
    try {
      const statusString = await invoke<string>("get_registration_status");
      const state = parseRegistrationStatus(statusString);
      setRegistrationState(state);
      return state;
    } catch (error) {
      console.error("DEBUG:[AUTHSTORE/GET_STATUS] Error getting registration status", error);
      throw error;
    }
  },

  // Unregister SIP account
  async unregister(): Promise<void> {
    console.log("DEBUG:[AUTHSTORE/UNREGISTER] Unregistering");

    try {
      await invoke<string>("unregister_account");
      setRegistrationState("unregistered");
      stopStatusPolling();
      console.log("DEBUG:[AUTHSTORE/UNREGISTER] Unregistration successful");
    } catch (error) {
      console.error("DEBUG:[AUTHSTORE/UNREGISTER] Unregistration failed", error);
      throw error;
    }
  },

  // Update user info
  updateUserInfo: (info: Partial<UserInfo>) => {
    console.log("DEBUG:[AUTHSTORE/UPDATE_USER] Updating user info");
    updateUserInfo((current) => ({ ...current, ...info }));
  },

  // Update SIP credentials
  updateSIPCredentials: (creds: Partial<SIPCredentials>) => {
    console.log("DEBUG:[AUTHSTORE/UPDATE_SIP] Updating SIP credentials");
    updateSIPCredentials((current) => ({ ...current, ...creds }));
  },
};

