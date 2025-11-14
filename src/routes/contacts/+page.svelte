<script lang="ts">
  import { Plus } from "lucide-svelte";
  import { Button } from "$lib/components/ui/button";
  import ContactSearchBar from "$lib/components/contacts/ContactSearchBar.svelte";
  import FavoritesSection from "$lib/components/contacts/FavoritesSection.svelte";
  import ContactList from "$lib/components/contacts/ContactList.svelte";
  import AddContactDialog from "$lib/components/contacts/AddContactDialog.svelte";
</script>

<div class="flex flex-col h-full">
  <!-- Header -->
  <div class="flex items-center justify-between p-6 border-b border-gray-200">
    <div>
      <h1 class="text-2xl font-bold text-gray-900">Contacts</h1>
      <p class="text-sm text-gray-500 mt-1">Manage your contacts</p>
    </div>
    <AddContactDialog>
      {#snippet trigger()}
        <Button>
          <Plus class="h-4 w-4 mr-2" />
          Add Contact
        </Button>
      {/snippet}
    </AddContactDialog>
  </div>

  <!-- Main Content -->
  <div class="flex-1 overflow-y-auto p-6 space-y-6">
    <!-- Search Bar -->
    <ContactSearchBar />

    <!-- Favorites Section -->
    <FavoritesSection />

    <!-- Contact List -->
    <ContactList />
  </div>
</div>
