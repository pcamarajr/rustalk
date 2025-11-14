<script lang="ts">
  import { Card, CardContent } from "$lib/components/ui/card";
  import HistoryItem from "./HistoryItem.svelte";
  import {
    historyStore,
    type CallHistoryEntry,
    type TypeFilter,
    type DateFilter,
  } from "$lib/stores/historyStore";
  import { formatRelativeDate } from "$lib/utils";

  // Subscribe to filter state
  let typeFilter = $state<TypeFilter>("all");
  let dateFilter = $state<DateFilter>("all");

  $effect(() => {
    const unsubscribeType = historyStore.typeFilter.subscribe((value) => {
      typeFilter = value;
    });
    const unsubscribeDate = historyStore.dateFilter.subscribe((value) => {
      dateFilter = value;
    });

    return () => {
      unsubscribeType();
      unsubscribeDate();
    };
  });

  // Get filtered history entries
  let entries = $state<CallHistoryEntry[]>([]);

  $effect(() => {
    const filteredStore = historyStore.getFilteredHistory(
      typeFilter,
      dateFilter
    );
    const unsubscribe = filteredStore.subscribe((value) => {
      entries = value;
    });

    return unsubscribe;
  });

  // Group entries by date
  let groupedEntries = $derived.by(() => {
    const grouped: Record<string, CallHistoryEntry[]> = {};

    entries.forEach((entry) => {
      const dateKey = formatRelativeDate(entry.timestamp);
      if (!grouped[dateKey]) {
        grouped[dateKey] = [];
      }
      grouped[dateKey].push(entry);
    });

    // Sort date keys: Today first, then Yesterday, then others chronologically
    const sortedKeys = Object.keys(grouped).sort((a, b) => {
      if (a === "Today") return -1;
      if (b === "Today") return 1;
      if (a === "Yesterday") return -1;
      if (b === "Yesterday") return 1;
      // For other dates, sort by the first entry's timestamp (newest first)
      const dateA = grouped[a][0]?.timestamp.getTime() || 0;
      const dateB = grouped[b][0]?.timestamp.getTime() || 0;
      return dateB - dateA;
    });

    return sortedKeys.map((key) => ({ date: key, entries: grouped[key] }));
  });
</script>

{#if entries.length === 0}
  <Card>
    <CardContent class="py-12 text-center">
      <p class="text-gray-500">No calls yet.</p>
    </CardContent>
  </Card>
{:else}
  <div class="space-y-6">
    {#each groupedEntries as { date, entries } (date)}
      <div>
        <!-- Date Section Header -->
        <div class="mb-3">
          <h3 class="text-sm font-semibold text-gray-700">{date}</h3>
        </div>

        <!-- Entries for this date -->
        <Card>
          <CardContent class="p-0">
            <div class="divide-y divide-gray-100">
              {#each entries as entry (entry.id)}
                <HistoryItem {entry} />
              {/each}
            </div>
          </CardContent>
        </Card>
      </div>
    {/each}
  </div>
{/if}
