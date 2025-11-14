import { writable, derived, type Readable } from "svelte/store";

export type CallDirection = "incoming" | "outgoing" | "missed";
export type DateFilter = "all" | "today" | "yesterday" | "this-week";
export type TypeFilter = "all" | CallDirection;

export interface CallHistoryEntry {
  id: string;
  name: string | null; // null for unknown numbers
  number: string;
  direction: CallDirection;
  duration: number; // seconds, 0 for missed
  timestamp: Date;
  contactId?: string; // optional link to contact
}

// Mock call history data
const mockHistory: CallHistoryEntry[] = [
  {
    id: "1",
    name: "Sarah Johnson",
    number: "+15554567890",
    direction: "outgoing",
    duration: 165, // 2:45
    timestamp: new Date(Date.now() - 2 * 60 * 60 * 1000), // 2 hours ago
    contactId: "6",
  },
  {
    id: "2",
    name: "Mom",
    number: "+15551234567",
    direction: "incoming",
    duration: 923, // 15:23
    timestamp: new Date(Date.now() - 4 * 60 * 60 * 1000), // 4 hours ago
    contactId: "4",
  },
  {
    id: "3",
    name: null,
    number: "+15559876543",
    direction: "missed",
    duration: 0,
    timestamp: new Date(Date.now() - 5 * 60 * 60 * 1000), // 5 hours ago
  },
  {
    id: "4",
    name: "Work - Conference",
    number: "+15555678901",
    direction: "outgoing",
    duration: 2712, // 45:12
    timestamp: new Date(Date.now() - 24 * 60 * 60 * 1000), // Yesterday
    contactId: "7",
  },
  {
    id: "5",
    name: "Alice Smith",
    number: "+15551234567",
    direction: "incoming",
    duration: 342, // 5:42
    timestamp: new Date(Date.now() - 2 * 24 * 60 * 60 * 1000), // 2 days ago
    contactId: "1",
  },
  {
    id: "6",
    name: "Bob Williams",
    number: "+15553456789",
    direction: "outgoing",
    duration: 120, // 2:00
    timestamp: new Date(Date.now() - 3 * 24 * 60 * 60 * 1000), // 3 days ago
    contactId: "3",
  },
  {
    id: "7",
    name: null,
    number: "+15551112222",
    direction: "missed",
    duration: 0,
    timestamp: new Date(Date.now() - 4 * 24 * 60 * 60 * 1000), // 4 days ago
  },
  {
    id: "8",
    name: "David Miller",
    number: "+15556789012",
    direction: "incoming",
    duration: 456, // 7:36
    timestamp: new Date(Date.now() - 5 * 24 * 60 * 60 * 1000), // 5 days ago
    contactId: "8",
  },
];

// Create writable store
const { subscribe, set, update } = writable<CallHistoryEntry[]>(mockHistory);

// Filter stores for component-managed filter state
const { subscribe: subscribeTypeFilter, set: setTypeFilter } =
  writable<TypeFilter>("all");
export const typeFilterStore = {
  subscribe: subscribeTypeFilter,
  set: setTypeFilter,
};

const { subscribe: subscribeDateFilter, set: setDateFilter } =
  writable<DateFilter>("all");
export const dateFilterStore = {
  subscribe: subscribeDateFilter,
  set: setDateFilter,
};

// Derived stores
export const allHistory = derived({ subscribe }, ($history) => $history);

// Filter by type function
export function filterByType(type: TypeFilter): Readable<CallHistoryEntry[]> {
  if (type === "all") {
    return allHistory;
  }
  return derived({ subscribe }, ($history) =>
    $history.filter((entry) => entry.direction === type)
  );
}

// Filter by date function
function isToday(date: Date): boolean {
  const today = new Date();
  return (
    date.getDate() === today.getDate() &&
    date.getMonth() === today.getMonth() &&
    date.getFullYear() === today.getFullYear()
  );
}

function isYesterday(date: Date): boolean {
  const yesterday = new Date();
  yesterday.setDate(yesterday.getDate() - 1);
  return (
    date.getDate() === yesterday.getDate() &&
    date.getMonth() === yesterday.getMonth() &&
    date.getFullYear() === yesterday.getFullYear()
  );
}

function isThisWeek(date: Date): boolean {
  const today = new Date();
  const weekAgo = new Date(today);
  weekAgo.setDate(today.getDate() - 7);
  return date >= weekAgo;
}

export function filterByDate(range: DateFilter): Readable<CallHistoryEntry[]> {
  if (range === "all") {
    return allHistory;
  }
  return derived({ subscribe }, ($history) =>
    $history.filter((entry) => {
      switch (range) {
        case "today":
          return isToday(entry.timestamp);
        case "yesterday":
          return isYesterday(entry.timestamp);
        case "this-week":
          return isThisWeek(entry.timestamp);
        default:
          return true;
      }
    })
  );
}

// Combined filter function
export function getFilteredHistory(
  type: TypeFilter,
  date: DateFilter
): Readable<CallHistoryEntry[]> {
  return derived(
    { subscribe },
    ($history) => {
      let filtered = $history;

      // Filter by type
      if (type !== "all") {
        filtered = filtered.filter((entry) => entry.direction === type);
      }

      // Filter by date
      if (date !== "all") {
        filtered = filtered.filter((entry) => {
          switch (date) {
            case "today":
              return isToday(entry.timestamp);
            case "yesterday":
              return isYesterday(entry.timestamp);
            case "this-week":
              return isThisWeek(entry.timestamp);
            default:
              return true;
          }
        });
      }

      // Sort by timestamp (newest first)
      return filtered.sort(
        (a, b) => b.timestamp.getTime() - a.timestamp.getTime()
      );
    }
  );
}

// Store methods
export const historyStore = {
  subscribe,
  // Get all history (returns the store directly)
  getAllHistory: () => allHistory,
  // Filter by type
  filterByType,
  // Filter by date
  filterByDate,
  // Get filtered history (combined filters)
  getFilteredHistory,
  // Clear all history
  clearHistory: () => {
    set([]);
  },
  // Delete specific entry
  deleteEntry: (id: string) => {
    update((history) => history.filter((entry) => entry.id !== id));
  },
  // Add new entry (for future use when real calls are made)
  addEntry: (entry: Omit<CallHistoryEntry, "id">) => {
    update((history) => {
      const newEntry: CallHistoryEntry = {
        ...entry,
        id: Date.now().toString(), // Simple ID generation for mock
      };
      return [newEntry, ...history];
    });
  },
  // Filter stores
  typeFilter: typeFilterStore,
  dateFilter: dateFilterStore,
};

