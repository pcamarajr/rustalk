import { writable, derived, type Readable } from "svelte/store";

export interface PhoneNumber {
  type: "mobile" | "work" | "home" | "other";
  number: string;
}

export interface Contact {
  id: string;
  name: string;
  numbers: PhoneNumber[];
  email?: string;
  favorite: boolean;
}

// Mock contacts data
const mockContacts: Contact[] = [
  {
    id: "1",
    name: "Alice Smith",
    numbers: [
      { type: "mobile", number: "+15551234567" },
      { type: "work", number: "+15559876543" },
    ],
    email: "alice@example.com",
    favorite: false,
  },
  {
    id: "2",
    name: "Alex Jones",
    numbers: [{ type: "mobile", number: "+15552345678" }],
    email: "alex@example.com",
    favorite: false,
  },
  {
    id: "3",
    name: "Bob Williams",
    numbers: [
      { type: "mobile", number: "+15553456789" },
      { type: "home", number: "+15554567890" },
    ],
    favorite: true,
  },
  {
    id: "4",
    name: "Mom",
    numbers: [{ type: "mobile", number: "+15551234567" }],
    favorite: true,
  },
  {
    id: "5",
    name: "Dad",
    numbers: [{ type: "mobile", number: "+15552345678" }],
    favorite: true,
  },
  {
    id: "6",
    name: "Sarah Johnson",
    numbers: [{ type: "mobile", number: "+15554567890" }],
    email: "sarah@example.com",
    favorite: true,
  },
  {
    id: "7",
    name: "Charlie Brown",
    numbers: [{ type: "work", number: "+15555678901" }],
    favorite: false,
  },
  {
    id: "8",
    name: "David Miller",
    numbers: [
      { type: "mobile", number: "+15556789012" },
      { type: "work", number: "+15557890123" },
    ],
    email: "david@example.com",
    favorite: false,
  },
  {
    id: "9",
    name: "Emma Davis",
    numbers: [{ type: "mobile", number: "+15558901234" }],
    favorite: false,
  },
  {
    id: "10",
    name: "Frank Wilson",
    numbers: [
      { type: "home", number: "+15559012345" },
      { type: "mobile", number: "+15550123456" },
    ],
    favorite: false,
  },
  {
    id: "11",
    name: "Grace Lee",
    numbers: [{ type: "mobile", number: "+15551234567" }],
    email: "grace@example.com",
    favorite: false,
  },
  {
    id: "12",
    name: "Henry Taylor",
    numbers: [{ type: "work", number: "+15552345678" }],
    favorite: false,
  },
];

// Create writable store
const { subscribe, set, update } = writable<Contact[]>(mockContacts);

// Search query store for component-managed search state
const { subscribe: subscribeSearchQuery, set: setSearchQuery, update: updateSearchQuery } = writable<string>("");
export const searchQueryStore = {
  subscribe: subscribeSearchQuery,
  set: setSearchQuery,
  update: updateSearchQuery,
};

// Derived stores
export const allContacts = derived({ subscribe }, ($contacts) => $contacts);

export const favoriteContacts = derived({ subscribe }, ($contacts) =>
  $contacts.filter((c) => c.favorite)
);

// Search function that returns a derived store
export function searchContacts(query: string): Readable<Contact[]> {
  const lowerQuery = query.toLowerCase().trim();
  if (!lowerQuery) {
    return allContacts;
  }
  return derived({ subscribe }, ($contacts) =>
    $contacts.filter((contact) => {
      const nameMatch = contact.name.toLowerCase().includes(lowerQuery);
      const numberMatch = contact.numbers.some((num) =>
        num.number.includes(lowerQuery.replace(/\D/g, ""))
      );
      const emailMatch = contact.email?.toLowerCase().includes(lowerQuery);
      return nameMatch || numberMatch || emailMatch;
    })
  );
}

// Store methods
export const contactsStore = {
  subscribe,
  // Get all contacts (returns the store directly)
  getAllContacts: () => allContacts,
  // Get favorite contacts (returns the store directly)
  getFavorites: () => favoriteContacts,
  // Search contacts
  searchContacts,
  // Add new contact
  addContact: (contact: Omit<Contact, "id">) => {
    update((contacts) => {
      const newContact: Contact = {
        ...contact,
        id: Date.now().toString(), // Simple ID generation for mock
      };
      return [...contacts, newContact];
    });
  },
  // Update existing contact
  updateContact: (id: string, updates: Partial<Contact>) => {
    update((contacts) =>
      contacts.map((contact) =>
        contact.id === id ? { ...contact, ...updates } : contact
      )
    );
  },
  // Delete contact
  deleteContact: (id: string) => {
    update((contacts) => contacts.filter((contact) => contact.id !== id));
  },
  // Toggle favorite status
  toggleFavorite: (id: string) => {
    update((contacts) =>
      contacts.map((contact) =>
        contact.id === id
          ? { ...contact, favorite: !contact.favorite }
          : contact
      )
    );
  },
  // Get contact by ID
  getContactById: (id: string): Readable<Contact | undefined> => {
    return derived({ subscribe }, ($contacts) =>
      $contacts.find((c) => c.id === id)
    );
  },
  // Search query store
  searchQuery: searchQueryStore,
};

