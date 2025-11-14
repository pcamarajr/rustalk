<script lang="ts">
  import {
    PhoneIncoming,
    PhoneOutgoing,
    PhoneMissed,
  } from "lucide-svelte";
  import { historyStore, type CallDirection } from "$lib/stores/historyStore";
  import { formatTimeAgo } from "$lib/utils";

  interface Props {
    onCallClick?: (number: string) => void;
  }

  let { onCallClick }: Props = $props();
  
  // Get recent calls from history store (last 5, sorted by timestamp, newest first)
  let recentCalls = $derived.by(() => {
    const history = $historyStore;
    return history
      .slice()
      .sort((a, b) => b.timestamp.getTime() - a.timestamp.getTime())
      .slice(0, 5)
      .map((entry) => ({
        name: entry.name || "Unknown",
        number: entry.number,
        time: formatTimeAgo(entry.timestamp),
        direction: entry.direction as CallDirection,
      }));
  });

  function handleCallClick(number: string) {
    onCallClick?.(number);
  }

  function getDirectionIcon(direction: CallDirection) {
    switch (direction) {
      case "incoming":
        return PhoneIncoming;
      case "outgoing":
        return PhoneOutgoing;
      case "missed":
        return PhoneMissed;
    }
  }
</script>

{#if recentCalls.length > 0}
  <div class="mt-4">
    <h3
      class="text-sm font-semibold text-gray-700 mb-3"
      id="recent-calls-heading"
    >
      Recent Calls:
    </h3>
    <div class="space-y-2" role="list" aria-labelledby="recent-calls-heading">
      {#each recentCalls as call}
        {@const DirectionIcon = getDirectionIcon(call.direction)}
        <div role="listitem">
          <button
            type="button"
            onclick={() => handleCallClick(call.number)}
            class="w-full flex items-center justify-between p-3 rounded-lg hover:bg-gray-50 active:bg-gray-100 transition-colors text-left focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
            aria-label={`Call ${call.name} at ${call.number}`}
          >
            <div class="flex items-center gap-3 flex-1 min-w-0">
              <DirectionIcon
                class="h-5 w-5 shrink-0 {call.direction === 'missed'
                  ? 'text-red-500'
                  : call.direction === 'incoming'
                    ? 'text-green-500'
                    : 'text-blue-500'}"
              />
              <div class="flex-1 min-w-0">
                <div class="text-sm font-medium text-gray-900 truncate">
                  {call.name}
                </div>
                <div class="text-xs text-gray-500 truncate">
                  {call.number}
                </div>
              </div>
            </div>
            <div class="text-xs text-gray-500 shrink-0 ml-2">{call.time}</div>
          </button>
        </div>
      {/each}
    </div>
  </div>
{/if}

