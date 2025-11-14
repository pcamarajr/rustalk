<script lang="ts">
  import {
    Phone,
    PhoneIncoming,
    PhoneOutgoing,
    PhoneMissed,
  } from "lucide-svelte";
  import { Button } from "$lib/components/ui/button";
  import CallHistoryDetails from "./CallHistoryDetails.svelte";
  import { useCallNavigation } from "$lib/hooks/useCallNavigation";
  import {
    formatPhoneNumber,
    formatDuration,
    formatTime,
    getDirectionIconColor,
    getDirectionLabel,
  } from "$lib/utils";
  import type { CallHistoryEntry } from "$lib/stores/historyStore";

  interface Props {
    entry: CallHistoryEntry;
  }

  let { entry }: Props = $props();

  // Reference to CallHistoryDetails component
  let callDetailsRef: CallHistoryDetails | null = null;

  // Use call navigation composable
  const { initiateCall } = useCallNavigation();

  function getDirectionIcon() {
    switch (entry.direction) {
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
    return entry.name || formatPhoneNumber(entry.number);
  }

  function handleCall(event: MouseEvent) {
    event.stopPropagation();
    console.log("DEBUG:[HISTORY/CALL] Initiating call to:", entry.number);
    initiateCall(entry.number);
  }

  function handleClick() {
    callDetailsRef?.openDialog();
  }

  const DirectionIcon = getDirectionIcon();
</script>

<div
  class="flex items-center justify-between p-3 hover:bg-gray-50 rounded-lg cursor-pointer transition-colors"
  role="button"
  tabindex="0"
  onclick={handleClick}
  onkeydown={(e) => {
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      handleClick();
    }
  }}
>
  <div class="flex items-center gap-3 flex-1 min-w-0">
    <!-- Direction Icon -->
    <DirectionIcon
      class="h-5 w-5 {getDirectionIconColor(entry.direction)} shrink-0"
    />

    <!-- Call Info -->
    <div class="flex-1 min-w-0">
      <div class="font-medium text-gray-900 truncate">{getDisplayName()}</div>
      <div class="text-sm text-gray-500">
        {getDirectionLabel(entry.direction)}
        {entry.duration > 0 ? ` · ${formatDuration(entry.duration)}` : ""}
        {entry.direction === "missed" ? " · Not answered" : ""}
      </div>
    </div>

    <!-- Time -->
    <div class="text-sm text-gray-500 shrink-0 ml-2">
      {formatTime(entry.timestamp)}
    </div>
  </div>

  <!-- Quick Call Button -->
  <Button
    variant="ghost"
    size="icon-sm"
    onclick={handleCall}
    aria-label={`Call ${getDisplayName()}`}
    class="ml-2 shrink-0"
  >
    <Phone class="h-4 w-4" />
  </Button>
</div>

<!-- Call History Details (self-contained) -->
<CallHistoryDetails bind:this={callDetailsRef} entryId={entry.id} />

