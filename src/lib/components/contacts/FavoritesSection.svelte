<script lang="ts">
  import { Star } from "lucide-svelte";
  import {
    Card,
    CardContent,
    CardHeader,
    CardTitle,
  } from "$lib/components/ui/card";
  import ContactItem from "./ContactItem.svelte";
  import { contactsStore } from "$lib/stores/contactsStore";
  import type { Contact } from "$lib/stores/contactsStore";

  // No props needed - ContactItem handles everything internally

  // Get favorites from store - subscribe to the store
  let favorites = $state<Contact[]>([]);

  $effect(() => {
    const store = contactsStore.getFavorites();
    const unsubscribe = store.subscribe((value) => {
      favorites = value;
    });
    return unsubscribe;
  });
</script>

{#if favorites.length > 0}
  <Card>
    <CardHeader>
      <CardTitle class="flex items-center gap-2">
        <Star class="h-5 w-5 text-yellow-500 fill-yellow-500" />
        Favorites
      </CardTitle>
    </CardHeader>
    <CardContent>
      <div class="space-y-1">
        {#each favorites as contact (contact.id)}
          <ContactItem {contact} showFavorite={false} />
        {/each}
      </div>
    </CardContent>
  </Card>
{/if}
