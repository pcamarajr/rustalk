<script lang="ts">
  import HistoryFilters from "$lib/components/history/HistoryFilters.svelte";
  import HistoryList from "$lib/components/history/HistoryList.svelte";
</script>

<div class="flex flex-col h-full">
  <!-- Header -->
  <div class="flex items-center justify-between p-6 border-b border-gray-200">
    <div>
      <h1 class="text-2xl font-bold text-gray-900">Call History</h1>
      <p class="text-sm text-gray-500 mt-1">View your call history</p>
    </div>
  </div>

  <!-- Main Content -->
  <div class="flex-1 overflow-y-auto p-6 space-y-6">
    <!-- Filter Controls -->
    <HistoryFilters />

    <!-- History List -->
    <HistoryList />
  </div>
</div>

