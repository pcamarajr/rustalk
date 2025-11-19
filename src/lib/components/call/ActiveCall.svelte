<script lang="ts">
  // Container island - simple shell that composes sub-islands
  import { callStore, callState } from "$lib/stores/callStore";
  import CallHeader from "./CallHeader.svelte";
  import CallTimer from "./CallTimer.svelte";
  import AudioVisualizer from "./AudioVisualizer.svelte";
  import CallControls from "./CallControls.svelte";
  import EndCallButton from "./EndCallButton.svelte";

  // Get current call state
  let state = $derived.by(() => $callState);
</script>

<div class="flex flex-col gap-6 p-6" role="main" aria-label="Active call">
  <CallHeader />

  <!-- Show state-specific UI -->
  {#if state === "ringing" || state === "connecting"}
    <div class="text-center py-8">
      <div class="text-lg font-medium text-gray-700">
        {#if state === "ringing"}
          <span class="inline-block animate-pulse">📞</span>
          <p class="mt-2">Ringing...</p>
        {:else if state === "connecting"}
          <span class="inline-block animate-pulse">🔗</span>
          <p class="mt-2">Connecting...</p>
        {/if}
      </div>
    </div>
  {:else if state === "active" || state === "onHold"}
    <CallTimer />
    <AudioVisualizer />
    <CallControls />
  {/if}

  <EndCallButton />
</div>
