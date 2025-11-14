<script lang="ts">
  import { Trash2 } from "lucide-svelte";
  import { Button } from "$lib/components/ui/button";
  import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
  } from "$lib/components/ui/select";
  import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
  } from "$lib/components/ui/dialog";
  import {
    historyStore,
    type TypeFilter,
    type DateFilter,
  } from "$lib/stores/historyStore";

  // Manage filter state internally
  let typeFilter = $state<TypeFilter>("all");
  let dateFilter = $state<DateFilter>("all");

  // Dialog state for clear confirmation
  let showClearConfirm = $state(false);

  // Sync with store for sharing with other components
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

  // Type guard for TypeFilter
  function isValidTypeFilter(value: string): value is TypeFilter {
    return (
      value === "all" ||
      value === "incoming" ||
      value === "outgoing" ||
      value === "missed"
    );
  }

  // Type guard for DateFilter
  function isValidDateFilter(value: string): value is DateFilter {
    return (
      value === "all" ||
      value === "today" ||
      value === "yesterday" ||
      value === "this-week"
    );
  }

  function handleTypeFilterChange(value: string) {
    if (isValidTypeFilter(value)) {
      typeFilter = value;
      historyStore.typeFilter.set(typeFilter);
    } else {
      console.warn("DEBUG:[HISTORY/FILTER] Invalid type filter value:", value);
    }
  }

  function handleDateFilterChange(value: string) {
    if (isValidDateFilter(value)) {
      dateFilter = value;
      historyStore.dateFilter.set(dateFilter);
    } else {
      console.warn("DEBUG:[HISTORY/FILTER] Invalid date filter value:", value);
    }
  }

  function handleClearHistory() {
    showClearConfirm = true;
  }

  function handleConfirmClear() {
    console.log("DEBUG:[HISTORY/CLEAR] Clearing all call history");
    historyStore.clearHistory();
    showClearConfirm = false;
  }

  function handleCancelClear() {
    showClearConfirm = false;
  }

  function getTypeLabel(type: TypeFilter): string {
    const labels: Record<TypeFilter, string> = {
      all: "All",
      incoming: "Incoming",
      outgoing: "Outgoing",
      missed: "Missed",
    };
    return labels[type] || "All";
  }

  function getDateLabel(date: DateFilter): string {
    const labels: Record<DateFilter, string> = {
      all: "All Time",
      today: "Today",
      yesterday: "Yesterday",
      "this-week": "This Week",
    };
    return labels[date] || "All Time";
  }
</script>

<div class="flex items-center gap-3">
  <!-- Type Filter -->
  <Select
    type="single"
    value={typeFilter}
    onValueChange={handleTypeFilterChange}
  >
    <SelectTrigger class="w-32">
      {getTypeLabel(typeFilter)}
    </SelectTrigger>
    <SelectContent>
      <SelectItem value="all" label="All" />
      <SelectItem value="incoming" label="Incoming" />
      <SelectItem value="outgoing" label="Outgoing" />
      <SelectItem value="missed" label="Missed" />
    </SelectContent>
  </Select>

  <!-- Date Filter -->
  <Select
    type="single"
    value={dateFilter}
    onValueChange={handleDateFilterChange}
  >
    <SelectTrigger class="w-36">
      {getDateLabel(dateFilter)}
    </SelectTrigger>
    <SelectContent>
      <SelectItem value="all" label="All Time" />
      <SelectItem value="today" label="Today" />
      <SelectItem value="yesterday" label="Yesterday" />
      <SelectItem value="this-week" label="This Week" />
    </SelectContent>
  </Select>

  <!-- Clear History Button -->
  <Button
    variant="ghost"
    size="icon-sm"
    onclick={handleClearHistory}
    aria-label="Clear history"
    class="ml-auto"
  >
    <Trash2 class="h-4 w-4" />
  </Button>
</div>

<!-- Clear History Confirmation Dialog -->
<Dialog bind:open={showClearConfirm}>
  <DialogContent class="max-w-md" onclick={(e) => e.stopPropagation()}>
    <DialogHeader>
      <DialogTitle>Clear Call History</DialogTitle>
      <DialogDescription>
        Are you sure you want to clear all call history? This action cannot be
        undone.
      </DialogDescription>
    </DialogHeader>
    <DialogFooter>
      <Button
        variant="outline"
        onclick={(e) => {
          e.stopPropagation();
          handleCancelClear();
        }}
      >
        Cancel
      </Button>
      <Button
        variant="destructive"
        onclick={(e) => {
          e.stopPropagation();
          handleConfirmClear();
        }}
      >
        Clear History
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>

