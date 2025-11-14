import { writable, derived, type Readable } from "svelte/store";

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

// Store methods
export const authStore = {
  subscribe: subscribeRegistrationState,
  registrationState,
  userInfo,
  sipCredentials,
  isRegistered,
  isRegistering,

  // Register SIP account (mock)
  register: () => {
    console.log("DEBUG:[AUTHSTORE/REGISTER] Starting registration");
    setRegistrationState("registering");

    // Mock registration delay
    setTimeout(() => {
      console.log("DEBUG:[AUTHSTORE/REGISTER] Registration successful");
      setRegistrationState("registered");
    }, 1000);
  },

  // Unregister SIP account
  unregister: () => {
    console.log("DEBUG:[AUTHSTORE/UNREGISTER] Unregistering");
    setRegistrationState("unregistered");
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

