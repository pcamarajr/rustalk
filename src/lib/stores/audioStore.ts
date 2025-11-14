import { writable, derived, type Readable } from "svelte/store";

export type AudioDeviceType = "input" | "output";

export interface AudioDevice {
  id: string;
  name: string;
  type: AudioDeviceType;
  isDefault: boolean;
}

export interface Ringtone {
  id: string;
  name: string;
}

// Mock audio devices
const mockInputDevices: AudioDevice[] = [
  { id: "mic-1", name: "Built-in Microphone", type: "input", isDefault: true },
  { id: "mic-2", name: "USB Headset Microphone", type: "input", isDefault: false },
  { id: "mic-3", name: "Blue Yeti", type: "input", isDefault: false },
];

const mockOutputDevices: AudioDevice[] = [
  { id: "speaker-1", name: "Built-in Speakers", type: "output", isDefault: true },
  { id: "speaker-2", name: "USB Headset", type: "output", isDefault: false },
  { id: "speaker-3", name: "AirPods Pro", type: "output", isDefault: false },
];

// Create writable stores
const { subscribe: subscribeInputDevices, set: setInputDevices } = writable<AudioDevice[]>(mockInputDevices);

const { subscribe: subscribeOutputDevices, set: setOutputDevices } = writable<AudioDevice[]>(mockOutputDevices);

const { subscribe: subscribeSelectedInputDeviceId, set: setSelectedInputDeviceId } = writable<string>(
  mockInputDevices.find((d) => d.isDefault)?.id || mockInputDevices[0].id
);

const { subscribe: subscribeSelectedOutputDeviceId, set: setSelectedOutputDeviceId } = writable<string>(
  mockOutputDevices.find((d) => d.isDefault)?.id || mockOutputDevices[0].id
);

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
      return mockInputDevices[0]; // Fallback to first mock device
    }
    if (!$selectedId) {
      return $devices[0]; // Fallback if no selected ID
    }
    return $devices.find((d: AudioDevice) => d.id === $selectedId) || $devices[0];
  }
);

export const selectedOutputDevice = derived(
  [outputDevicesReadable, selectedOutputDeviceIdReadable],
  ([$devices, $selectedId]) => {
    if (!$devices || $devices.length === 0) {
      return mockOutputDevices[0]; // Fallback to first mock device
    }
    if (!$selectedId) {
      return $devices[0]; // Fallback if no selected ID
    }
    return $devices.find((d: AudioDevice) => d.id === $selectedId) || $devices[0];
  }
);

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

  // Set selected input device
  setInputDevice: (deviceId: string) => {
    console.log("DEBUG:[AUDIOSTORE/SET_INPUT] Setting input device:", deviceId);
    setSelectedInputDeviceId(deviceId);
  },

  // Set selected output device
  setOutputDevice: (deviceId: string) => {
    console.log("DEBUG:[AUDIOSTORE/SET_OUTPUT] Setting output device:", deviceId);
    setSelectedOutputDeviceId(deviceId);
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

  // Refresh devices (mock - just returns current devices)
  refreshDevices: () => {
    console.log("DEBUG:[AUDIOSTORE/REFRESH] Refreshing audio devices");
    // In Phase 3, this will call Tauri command to enumerate real devices
    // For now, just log
  },
};

