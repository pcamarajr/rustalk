import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChild<T> = T extends { child?: any } ? Omit<T, "child"> : T;
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChildren<T> = T extends { children?: any } ? Omit<T, "children"> : T;
export type WithoutChildrenOrChild<T> = WithoutChildren<WithoutChild<T>>;
export type WithElementRef<T, U extends HTMLElement = HTMLElement> = T & { ref?: U | null };

/**
 * Formats a phone number for display
 * @param number - The phone number string (can include formatting characters)
 * @returns Formatted phone number string
 */
export function formatPhoneNumber(number: string): string {
  const digits = number.replace(/\D/g, "");
  if (digits.length === 11 && digits[0] === "1") {
    // US number with country code
    return `+${digits[0]} (${digits.slice(1, 4)}) ${digits.slice(4, 7)}-${digits.slice(7)}`;
  } else if (digits.length === 10) {
    // US number without country code
    return `(${digits.slice(0, 3)}) ${digits.slice(3, 6)}-${digits.slice(6)}`;
  }
  return number;
}

/**
 * Formats call duration in seconds to MM:SS or HH:MM:SS format
 * @param seconds - Duration in seconds
 * @returns Formatted duration string
 */
export function formatDuration(seconds: number): string {
  if (seconds === 0) return "0:00";
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = seconds % 60;
  
  if (hours > 0) {
    return `${hours}:${minutes.toString().padStart(2, "0")}:${secs.toString().padStart(2, "0")}`;
  }
  return `${minutes}:${secs.toString().padStart(2, "0")}`;
}

/**
 * Formats a date to a relative time string (Today, Yesterday) or formatted date
 * @param date - Date object
 * @returns Formatted date string
 */
export function formatRelativeDate(date: Date): string {
  const today = new Date();
  const yesterday = new Date(today);
  yesterday.setDate(yesterday.getDate() - 1);
  
  const isToday =
    date.getDate() === today.getDate() &&
    date.getMonth() === today.getMonth() &&
    date.getFullYear() === today.getFullYear();
  
  const isYesterday =
    date.getDate() === yesterday.getDate() &&
    date.getMonth() === yesterday.getMonth() &&
    date.getFullYear() === yesterday.getFullYear();
  
  if (isToday) {
    return "Today";
  } else if (isYesterday) {
    return "Yesterday";
  } else {
    return date.toLocaleDateString("en-US", {
      month: "short",
      day: "numeric",
      year: date.getFullYear() !== today.getFullYear() ? "numeric" : undefined,
    });
  }
}

/**
 * Formats a date to time string (HH:MM AM/PM)
 * @param date - Date object
 * @returns Formatted time string
 */
export function formatTime(date: Date): string {
  return date.toLocaleTimeString("en-US", {
    hour: "numeric",
    minute: "2-digit",
    hour12: true,
  });
}

/**
 * Formats a date to a relative time string (e.g., "2m ago", "1h ago", "2h ago")
 * @param date - Date object
 * @returns Formatted relative time string
 */
export function formatTimeAgo(date: Date): string {
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffSeconds = Math.floor(diffMs / 1000);
  const diffMinutes = Math.floor(diffSeconds / 60);
  const diffHours = Math.floor(diffMinutes / 60);
  const diffDays = Math.floor(diffHours / 24);

  if (diffSeconds < 60) {
    return "just now";
  } else if (diffMinutes < 60) {
    return `${diffMinutes}m ago`;
  } else if (diffHours < 24) {
    return `${diffHours}h ago`;
  } else if (diffDays === 1) {
    return "yesterday";
  } else if (diffDays < 7) {
    return `${diffDays}d ago`;
  } else {
    return date.toLocaleDateString("en-US", {
      month: "short",
      day: "numeric",
    });
  }
}

/**
 * Gets the appropriate icon component for a call direction
 * @param direction - The call direction
 * @returns The icon component name (for use with dynamic imports)
 */
export function getDirectionIconName(
  direction: "incoming" | "outgoing" | "missed"
): "PhoneIncoming" | "PhoneOutgoing" | "PhoneMissed" | "Phone" {
  switch (direction) {
    case "incoming":
      return "PhoneIncoming";
    case "outgoing":
      return "PhoneOutgoing";
    case "missed":
      return "PhoneMissed";
    default:
      return "Phone";
  }
}

/**
 * Gets the Tailwind CSS color class for a call direction icon
 * @param direction - The call direction
 * @returns Tailwind CSS color class string
 */
export function getDirectionIconColor(
  direction: "incoming" | "outgoing" | "missed"
): string {
  switch (direction) {
    case "incoming":
      return "text-green-500";
    case "outgoing":
      return "text-blue-500";
    case "missed":
      return "text-red-500";
    default:
      return "text-gray-500";
  }
}

/**
 * Gets the human-readable label for a call direction
 * @param direction - The call direction
 * @returns Label string
 */
export function getDirectionLabel(
  direction: "incoming" | "outgoing" | "missed"
): string {
  switch (direction) {
    case "incoming":
      return "Incoming";
    case "outgoing":
      return "Outgoing";
    case "missed":
      return "Missed";
    default:
      return "";
  }
}
