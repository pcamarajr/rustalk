import { writable, derived, type Readable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";

export type AudioDeviceType = "input" | "output";

export interface AudioDevice {
  id: string;
  name: string;
  type: AudioDeviceType;
  isDefault: boolean;
}

// Rust AudioDevice structure from Tauri
interface RustAudioDevice {
  id: string;
  name: string;
  is_input: boolean;
}

export interface Ringtone {
  id: string;
  name: string;
}

// Convert Rust AudioDevice to frontend AudioDevice
function convertRustDevice(device: RustAudioDevice, isDefault: boolean = false): AudioDevice {
  return {
    id: device.id,
    name: device.name,
    type: device.is_input ? "input" : "output",
    isDefault,
  };
}

// Create writable stores - initialize as empty arrays
const { subscribe: subscribeInputDevices, set: setInputDevices } = writable<AudioDevice[]>([]);

const { subscribe: subscribeOutputDevices, set: setOutputDevices } = writable<AudioDevice[]>([]);

const { subscribe: subscribeSelectedInputDeviceId, set: setSelectedInputDeviceId } = writable<string>("");

const { subscribe: subscribeSelectedOutputDeviceId, set: setSelectedOutputDeviceId } = writable<string>("");

// Loading states
const { subscribe: subscribeIsLoadingDevices, set: setIsLoadingDevices } = writable<boolean>(false);

// Mock ringtones
const mockRingtones: Ringtone[] = [
  { id: "ring1", name: "Default Ringtone" },
  { id: "ring2", name: "Classic" },
  { id: "ring3", name: "Modern" },
  { id: "ring4", name: "Soft" },
];

// Create writable stores for ringtones
const { subscribe: subscribeRingtones, set: setRingtones } = writable<Ringtone[]>(mockRingtones);

const { subscribe: subscribeSelectedRingtoneId, set: setSelectedRingtoneId } = writable<string>(
  mockRingtones[0].id
);

const { subscribe: subscribeRingtoneVolume, set: setRingtoneVolumeValue } = writable<number>(80);

// Create readable stores - use the subscribe functions directly
const inputDevicesReadable = { subscribe: subscribeInputDevices };
const outputDevicesReadable = { subscribe: subscribeOutputDevices };
const selectedInputDeviceIdReadable = { subscribe: subscribeSelectedInputDeviceId };
const selectedOutputDeviceIdReadable = { subscribe: subscribeSelectedOutputDeviceId };
const ringtonesReadable = { subscribe: subscribeRingtones };
const selectedRingtoneIdReadable = { subscribe: subscribeSelectedRingtoneId };
const ringtoneVolumeReadable = { subscribe: subscribeRingtoneVolume };

// Derived stores
export const inputDevices = derived(inputDevicesReadable, ($devices) => $devices);

export const outputDevices = derived(outputDevicesReadable, ($devices) => $devices);

export const selectedInputDevice = derived(
  [inputDevicesReadable, selectedInputDeviceIdReadable],
  ([$devices, $selectedId]) => {
    if (!$devices || $devices.length === 0) {
      return null;
    }
    if (!$selectedId) {
      return $devices[0]; // Fallback to first device
    }
    return $devices.find((d: AudioDevice) => d.id === $selectedId) || $devices[0];
  }
);

export const selectedOutputDevice = derived(
  [outputDevicesReadable, selectedOutputDeviceIdReadable],
  ([$devices, $selectedId]) => {
    if (!$devices || $devices.length === 0) {
      return null;
    }
    if (!$selectedId) {
      return $devices[0]; // Fallback to first device
    }
    return $devices.find((d: AudioDevice) => d.id === $selectedId) || $devices[0];
  }
);

export const isLoadingDevices = { subscribe: subscribeIsLoadingDevices };

// Derived stores for ringtones
export const ringtones = derived(ringtonesReadable, ($ringtones) => $ringtones);

export const selectedRingtone = derived(
  [ringtonesReadable, selectedRingtoneIdReadable],
  ([$ringtones, $selectedId]) => {
    if (!$ringtones || $ringtones.length === 0) {
      return mockRingtones[0]; // Fallback to first mock ringtone
    }
    if (!$selectedId) {
      return $ringtones[0]; // Fallback if no selected ID
    }
    return $ringtones.find((r: Ringtone) => r.id === $selectedId) || $ringtones[0];
  }
);

export const ringtoneVolume = derived(ringtoneVolumeReadable, ($volume) => $volume);

// Store methods
export const audioStore = {
  subscribe: subscribeInputDevices, // Default subscription to input devices
  inputDevices,
  outputDevices,
  selectedInputDevice,
  selectedOutputDevice,
  ringtones,
  selectedRingtone,
  ringtoneVolume,
  isLoadingDevices,

  // Refresh devices from Tauri
  async refreshDevices(): Promise<void> {
    console.log("DEBUG:[AUDIOSTORE/REFRESH] Refreshing audio devices");
    setIsLoadingDevices(true);

    try {
      // Fetch input and output devices in parallel
      const [inputDevicesResult, outputDevicesResult] = await Promise.all([
        invoke<RustAudioDevice[]>("list_input_devices"),
        invoke<RustAudioDevice[]>("list_output_devices"),
      ]);

      // Convert Rust devices to frontend format
      const inputDevices = inputDevicesResult.map((device) => convertRustDevice(device, false));
      const outputDevices = outputDevicesResult.map((device) => convertRustDevice(device, false));

      // Mark first device as default if available
      if (inputDevices.length > 0) {
        inputDevices[0].isDefault = true;
      }
      if (outputDevices.length > 0) {
        outputDevices[0].isDefault = true;
      }

      setInputDevices(inputDevices);
      setOutputDevices(outputDevices);

      console.log(
        `DEBUG:[AUDIOSTORE/REFRESH] Found ${inputDevices.length} input devices and ${outputDevices.length} output devices`
      );

      // If no device is selected, select the first one (or default)
      // Note: We check the store value directly via subscription
      let currentInputId = "";
      let currentOutputId = "";
      
      const unsubscribeInput = subscribeSelectedInputDeviceId((id) => {
        currentInputId = id;
      });
      const unsubscribeOutput = subscribeSelectedOutputDeviceId((id) => {
        currentOutputId = id;
      });
      
      unsubscribeInput();
      unsubscribeOutput();
      
      if (!currentInputId && inputDevices.length > 0) {
        setSelectedInputDeviceId(inputDevices[0].id);
      }

      if (!currentOutputId && outputDevices.length > 0) {
        setSelectedOutputDeviceId(outputDevices[0].id);
      }
    } catch (error) {
      console.error("DEBUG:[AUDIOSTORE/REFRESH] Error refreshing devices:", error);
      // On error, set empty arrays
      setInputDevices([]);
      setOutputDevices([]);
      throw error;
    } finally {
      setIsLoadingDevices(false);
    }
  },

  // Get current devices from Tauri
  async getCurrentDevices(): Promise<void> {
    console.log("DEBUG:[AUDIOSTORE/GET_CURRENT] Getting current audio devices");
    try {
      const [inputDevice, outputDevice] = await Promise.all([
        invoke<RustAudioDevice | null>("get_input_device"),
        invoke<RustAudioDevice | null>("get_output_device"),
      ]);

      if (inputDevice) {
        setSelectedInputDeviceId(inputDevice.id);
        console.log("DEBUG:[AUDIOSTORE/GET_CURRENT] Current input device:", inputDevice.id);
      }

      if (outputDevice) {
        setSelectedOutputDeviceId(outputDevice.id);
        console.log("DEBUG:[AUDIOSTORE/GET_CURRENT] Current output device:", outputDevice.id);
      }
    } catch (error) {
      console.error("DEBUG:[AUDIOSTORE/GET_CURRENT] Error getting current devices:", error);
      throw error;
    }
  },

  // Set selected input device via Tauri
  async setInputDevice(deviceId: string): Promise<void> {
    console.log("DEBUG:[AUDIOSTORE/SET_INPUT] Setting input device:", deviceId);
    try {
      await invoke<string>("set_input_device", { deviceId });
      setSelectedInputDeviceId(deviceId);
      console.log("DEBUG:[AUDIOSTORE/SET_INPUT] Input device set successfully");
    } catch (error) {
      console.error("DEBUG:[AUDIOSTORE/SET_INPUT] Error setting input device:", error);
      throw error;
    }
  },

  // Set selected output device via Tauri
  async setOutputDevice(deviceId: string): Promise<void> {
    console.log("DEBUG:[AUDIOSTORE/SET_OUTPUT] Setting output device:", deviceId);
    try {
      await invoke<string>("set_output_device", { deviceId });
      setSelectedOutputDeviceId(deviceId);
      console.log("DEBUG:[AUDIOSTORE/SET_OUTPUT] Output device set successfully");
    } catch (error) {
      console.error("DEBUG:[AUDIOSTORE/SET_OUTPUT] Error setting output device:", error);
      throw error;
    }
  },

  // Set selected ringtone
  setRingtone: (ringtoneId: string) => {
    console.log("DEBUG:[AUDIOSTORE/SET_RINGTONE] Setting ringtone:", ringtoneId);
    setSelectedRingtoneId(ringtoneId);
  },

  // Set ringtone volume
  setRingtoneVolume: (volume: number) => {
    console.log("DEBUG:[AUDIOSTORE/SET_RINGTONE_VOLUME] Setting ringtone volume:", volume);
    setRingtoneVolumeValue(volume);
  },
};


