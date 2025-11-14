<script lang="ts">
  import { Card, CardContent } from "$lib/components/ui/card";
  import ContactItem from "./ContactItem.svelte";
  import { contactsStore } from "$lib/stores/contactsStore";
  import type { Contact } from "$lib/stores/contactsStore";

  // No props needed - ContactItem handles everything internally

  // Subscribe to search query from store
  let searchQuery = $state("");

  $effect(() => {
    const unsubscribe = contactsStore.searchQuery.subscribe((value) => {
      searchQuery = value;
    });
    return unsubscribe;
  });

  // Get contacts based on search query
  // Subscribe to the appropriate store based on search query
  let contacts = $state<Contact[]>([]);

  $effect(() => {
    let store: ReturnType<typeof contactsStore.getAllContacts>;
    if (!searchQuery.trim()) {
      store = contactsStore.getAllContacts();
    } else {
      store = contactsStore.searchContacts(searchQuery);
    }

    const unsubscribe = store.subscribe((value) => {
      contacts = value;
    });

    return unsubscribe;
  });

  // Group contacts alphabetically
  let groupedContacts = $derived.by(() => {
    const grouped: Record<string, Contact[]> = {};
    contacts.forEach((contact) => {
      const firstLetter = contact.name.charAt(0).toUpperCase();
      if (!/[A-Z]/.test(firstLetter)) {
        // Non-letter characters go to "#"
        if (!grouped["#"]) {
          grouped["#"] = [];
        }
        grouped["#"].push(contact);
      } else {
        if (!grouped[firstLetter]) {
          grouped[firstLetter] = [];
        }
        grouped[firstLetter].push(contact);
      }
    });

    // Sort each group
    Object.keys(grouped).forEach((key) => {
      grouped[key].sort((a, b) => a.name.localeCompare(b.name));
    });

    // Sort keys alphabetically, with # at the end
    const sortedKeys = Object.keys(grouped).sort((a, b) => {
      if (a === "#") return 1;
      if (b === "#") return -1;
      return a.localeCompare(b);
    });

    return sortedKeys.map((key) => ({ letter: key, contacts: grouped[key] }));
  });
</script>

{#if contacts.length === 0}
  <Card>
    <CardContent class="py-12 text-center">
      <p class="text-gray-500">
        {searchQuery
          ? "No contacts found matching your search."
          : "No contacts yet."}
      </p>
    </CardContent>
  </Card>
{:else}
  <Card>
    <CardContent class="p-0">
      <div class="divide-y divide-gray-100">
        {#each groupedContacts as { letter, contacts } (letter)}
          <div class="py-2">
            <!-- Section Header -->
            <div class="px-4 py-2 bg-gray-50">
              <h3 class="text-sm font-semibold text-gray-700">{letter}</h3>
            </div>
            <!-- Contact Items -->
            <div class="divide-y divide-gray-50">
              {#each contacts as contact (contact.id)}
                <ContactItem {contact} />
              {/each}
            </div>
          </div>
        {/each}
      </div>
    </CardContent>
  </Card>
{/if}
