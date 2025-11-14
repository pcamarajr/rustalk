<script lang="ts">
  import {
    Phone,
    PhoneIncoming,
    PhoneOutgoing,
    PhoneMissed,
    Calendar,
    Clock,
    Timer,
    BarChart3,
    Trash2,
    UserPlus,
  } from "lucide-svelte";
  import { Button } from "$lib/components/ui/button";
  import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogHeader,
    DialogTitle,
  } from "$lib/components/ui/dialog";
  import {
    historyStore,
    type CallHistoryEntry,
  } from "$lib/stores/historyStore";
  import { useCallNavigation } from "$lib/hooks/useCallNavigation";
  import {
    formatPhoneNumber,
    formatDuration,
    formatTime,
    getDirectionIconColor,
    getDirectionLabel,
  } from "$lib/utils";
  import { derived, get } from "svelte/store";

  interface Props {
    entryId: string;
    onOpenChange?: (open: boolean) => void;
  }

  let { entryId, onOpenChange }: Props = $props();

  // Manage visibility state internally
  let open = $state(false);

  // Optimized: Create derived store that only updates when this specific entry changes
  const entryStore = derived(
    historyStore,
    ($history) => $history.find((e) => e.id === entryId) || null
  );

  // Subscribe to entry data (only updates when this entry changes)
  let entryData = $state<CallHistoryEntry | null>(null);
  let isLoading = $state(false);
  let isError = $state(false);

  $effect(() => {
    const unsubscribe = entryStore.subscribe((value) => {
      entryData = value;
      isLoading = false;
      isError = value === null && open; // Only error if dialog is open and entry not found
    });

    return unsubscribe;
  });

  // Use call navigation composable
  const { initiateCall } = useCallNavigation();

  let showDeleteConfirm = $state(false);

  // Expose open method
  export function openDialog() {
    open = true;
    // Get current value immediately (synchronous store lookup)
    const currentValue = get(entryStore);
    entryData = currentValue;
    isLoading = false;
    isError = currentValue === null;
  }

  // Handle dialog open state changes
  function handleOpenChange(newOpen: boolean) {
    open = newOpen;
    if (!newOpen) {
      showDeleteConfirm = false;
    }
    onOpenChange?.(newOpen);
  }

  function getDirectionIcon() {
    if (!entryData) return Phone;
    switch (entryData.direction) {
      case "incoming":
        return PhoneIncoming;
      case "outgoing":
        return PhoneOutgoing;
      case "missed":
        return PhoneMissed;
      default:
        return Phone;
    }
  }

  function getDisplayName(): string {
    if (!entryData) return "";
    return entryData.name || formatPhoneNumber(entryData.number);
  }

  function handleDelete() {
    if (showDeleteConfirm && entryData) {
      console.log("DEBUG:[HISTORY/DELETE] Deleting entry:", entryData.id);
      historyStore.deleteEntry(entryData.id);
      open = false;
    } else {
      showDeleteConfirm = true;
    }
  }

  function handleCall() {
    if (entryData) {
      console.log("DEBUG:[HISTORY/CALL] Initiating call to:", entryData.number);
      initiateCall(entryData.number);
      open = false;
    }
  }

  function handleAddToContacts() {
    if (entryData && !entryData.name) {
      // Navigate to contacts page - user can add contact there
      // For Phase 2, we'll just log this action
      console.log(
        "DEBUG:[HISTORY/ADD_CONTACT] Navigate to contacts to add:",
        entryData.number
      );
      // TODO: In future phases, navigate to contacts page with number pre-filled
      open = false;
    }
  }

  function getCallStartTime(): string {
    if (!entryData) return "";
    return formatTime(entryData.timestamp);
  }

  function getCallEndTime(): string {
    if (!entryData || entryData.duration === 0) return "";
    const endTime = new Date(
      entryData.timestamp.getTime() + entryData.duration * 1000
    );
    return formatTime(endTime);
  }

  function getFullDate(): string {
    if (!entryData) return "";
    return entryData.timestamp.toLocaleDateString("en-US", {
      weekday: "long",
      year: "numeric",
      month: "long",
      day: "numeric",
    });
  }

  let DirectionIcon = $derived(getDirectionIcon());
</script>

<Dialog bind:open onOpenChange={handleOpenChange}>
  <DialogContent class="max-w-md" onclick={(e) => e.stopPropagation()}>
    {#if isLoading}
      <DialogHeader>
        <DialogTitle>Call Details</DialogTitle>
        <DialogDescription>Loading call information...</DialogDescription>
      </DialogHeader>
      <div class="space-y-4 py-4">
        <div class="h-4 w-full bg-gray-200 rounded animate-pulse"></div>
        <div class="h-4 w-3/4 bg-gray-200 rounded animate-pulse"></div>
        <div class="h-4 w-1/2 bg-gray-200 rounded animate-pulse"></div>
      </div>
    {:else if isError || !entryData}
      <DialogHeader>
        <DialogTitle>Call Details</DialogTitle>
        <DialogDescription>
          Unable to load call details. The entry may have been deleted.
        </DialogDescription>
      </DialogHeader>
      <div class="py-4">
        <p class="text-sm text-gray-500 text-center">Call entry not found</p>
      </div>
    {:else}
      <DialogHeader>
        <div class="flex items-center gap-3">
          <DirectionIcon
            class="h-6 w-6 {entryData
              ? getDirectionIconColor(entryData.direction)
              : 'text-gray-500'}"
          />
          <div class="flex-1">
            <DialogTitle>{getDisplayName()}</DialogTitle>
            <DialogDescription>
              {entryData
                ? getDirectionLabel(entryData.direction)
                : "Call information"}
            </DialogDescription>
          </div>
        </div>
      </DialogHeader>

      <div class="space-y-4 py-4">
        <!-- Phone Number -->
        <div class="flex items-center gap-3">
          <Phone class="h-5 w-5 text-gray-400 shrink-0" />
          <div class="flex-1 min-w-0">
            <div class="text-sm font-medium text-gray-900">Phone Number</div>
            <div class="text-sm text-gray-500 truncate">
              {formatPhoneNumber(entryData.number)}
            </div>
          </div>
        </div>

        <!-- Date -->
        <div class="flex items-center gap-3">
          <Calendar class="h-5 w-5 text-gray-400 shrink-0" />
          <div class="flex-1 min-w-0">
            <div class="text-sm font-medium text-gray-900">Date</div>
            <div class="text-sm text-gray-500">{getFullDate()}</div>
          </div>
        </div>

        <!-- Time Range -->
        {#if entryData.duration > 0}
          <div class="flex items-center gap-3">
            <Clock class="h-5 w-5 text-gray-400 shrink-0" />
            <div class="flex-1 min-w-0">
              <div class="text-sm font-medium text-gray-900">Time</div>
              <div class="text-sm text-gray-500">
                {getCallStartTime()} - {getCallEndTime()}
              </div>
            </div>
          </div>
        {:else}
          <div class="flex items-center gap-3">
            <Clock class="h-5 w-5 text-gray-400 shrink-0" />
            <div class="flex-1 min-w-0">
              <div class="text-sm font-medium text-gray-900">Time</div>
              <div class="text-sm text-gray-500">{getCallStartTime()}</div>
            </div>
          </div>
        {/if}

        <!-- Duration -->
        {#if entryData.duration > 0}
          <div class="flex items-center gap-3">
            <Timer class="h-5 w-5 text-gray-400 shrink-0" />
            <div class="flex-1 min-w-0">
              <div class="text-sm font-medium text-gray-900">Duration</div>
              <div class="text-sm text-gray-500">
                {formatDuration(entryData.duration)}
              </div>
            </div>
          </div>
        {/if}

        <!-- Call Type -->
        <div class="flex items-center gap-3">
          <Phone class="h-5 w-5 text-gray-400 shrink-0" />
          <div class="flex-1 min-w-0">
            <div class="text-sm font-medium text-gray-900">Call Type</div>
            <div class="text-sm text-gray-500">
              {getDirectionLabel(entryData.direction)}
            </div>
          </div>
        </div>

        <!-- Quality (mock) -->
        {#if entryData.duration > 0}
          <div class="flex items-center gap-3">
            <BarChart3 class="h-5 w-5 text-gray-400 shrink-0" />
            <div class="flex-1 min-w-0">
              <div class="text-sm font-medium text-gray-900">Quality</div>
              <div class="text-sm text-gray-500">Excellent</div>
            </div>
          </div>
        {/if}

        <!-- Actions -->
        <div class="flex flex-col gap-2 pt-4 border-t border-gray-200">
          <Button
            onclick={(e) => {
              e.stopPropagation();
              handleCall();
            }}
            class="w-full"
          >
            <Phone class="h-4 w-4 mr-2" />
            Call Back
          </Button>
          {#if !entryData.name}
            <Button
              variant="outline"
              onclick={(e) => {
                e.stopPropagation();
                handleAddToContacts();
              }}
              class="w-full"
            >
              <UserPlus class="h-4 w-4 mr-2" />
              Add to Contacts
            </Button>
          {/if}
          <Button
            variant={showDeleteConfirm ? "destructive" : "outline"}
            onclick={(e) => {
              e.stopPropagation();
              handleDelete();
            }}
            class="w-full"
          >
            <Trash2 class="h-4 w-4 mr-2" />
            {showDeleteConfirm ? "Confirm Delete" : "Delete Entry"}
          </Button>
        </div>
      </div>
    {/if}
  </DialogContent>
</Dialog>
