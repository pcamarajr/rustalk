<script lang="ts">
  import { Search, X } from "lucide-svelte";
  import { Input } from "$lib/components/ui/input";
  import { contactsStore } from "$lib/stores/contactsStore";

  interface Props {
    placeholder?: string;
  }

  let { placeholder = "Search contacts..." }: Props = $props();

  // Manage search query state internally
  let searchQuery = $state("");

  function handleClear() {
    searchQuery = "";
    contactsStore.searchQuery.set("");
  }

  // Debounce search updates to store
  let searchTimeout = $state.raw<ReturnType<typeof setTimeout> | null>(null);

  function handleInput(event: Event) {
    const target = event.target as HTMLInputElement;
    searchQuery = target.value;

    // Clear existing timeout
    if (searchTimeout) {
      clearTimeout(searchTimeout);
    }

    // Debounce search store update
    searchTimeout = setTimeout(() => {
      contactsStore.searchQuery.set(searchQuery);
    }, 300);
  }

  // Cleanup timeout on unmount
  $effect(() => {
    return () => {
      if (searchTimeout) {
        clearTimeout(searchTimeout);
        searchTimeout = null;
      }
    };
  });
</script>

<div class="relative">
  <Search
    class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-gray-400"
  />
  <Input
    type="text"
    value={searchQuery}
    oninput={handleInput}
    {placeholder}
    class="pl-9 pr-9"
  />
  {#if searchQuery}
    <button
      type="button"
      onclick={handleClear}
      class="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600"
      aria-label="Clear search"
    >
      <X class="h-4 w-4" />
    </button>
  {/if}
</div>
