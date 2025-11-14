<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import {
    Mic,
    MicOff,
    Pause,
    Play,
    Grid3x3,
    MoreVertical,
  } from "lucide-svelte";

  // Self-contained state for each control
  let isMuted = $state(false);
  let isOnHold = $state(false);
  let showKeypad = $state(false);
  let showMoreMenu = $state(false);

  function handleMute() {
    isMuted = !isMuted;
    console.log("DEBUG:[CALL/MUTE]", isMuted ? "Muted" : "Unmuted");
  }

  function handleHold() {
    isOnHold = !isOnHold;
    console.log("DEBUG:[CALL/HOLD]", isOnHold ? "On Hold" : "Resumed");
  }

  function handleKeypad() {
    showKeypad = !showKeypad;
    console.log("DEBUG:[CALL/KEYPAD]", showKeypad ? "Opened" : "Closed");
    // TODO: Open DTMF keypad overlay in Phase 5
  }

  function handleMore() {
    showMoreMenu = !showMoreMenu;
    console.log("DEBUG:[CALL/MORE]", showMoreMenu ? "Opened" : "Closed");
    // TODO: Open more menu in Phase 5
  }
</script>

<div class="grid grid-cols-4 gap-3">
  <!-- Mute Button -->
  <button
    type="button"
    onclick={handleMute}
    class="flex flex-col items-center justify-center gap-2 p-4 rounded-lg border transition-all focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2 {isMuted
      ? 'bg-red-100 border-red-300 text-red-700 hover:bg-red-200'
      : 'bg-white border-gray-200 text-gray-700 hover:bg-gray-50'}"
    aria-label={isMuted ? "Unmute microphone" : "Mute microphone"}
    aria-pressed={isMuted}
  >
    {#if isMuted}
      <MicOff class="h-6 w-6" />
    {:else}
      <Mic class="h-6 w-6" />
    {/if}
    <span class="text-xs font-medium">{isMuted ? "Unmute" : "Mute"}</span>
  </button>

  <!-- Hold Button -->
  <button
    type="button"
    onclick={handleHold}
    class="flex flex-col items-center justify-center gap-2 p-4 rounded-lg border transition-all focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2 {isOnHold
      ? 'bg-amber-100 border-amber-300 text-amber-700 hover:bg-amber-200'
      : 'bg-white border-gray-200 text-gray-700 hover:bg-gray-50'}"
    aria-label={isOnHold ? "Resume call" : "Hold call"}
    aria-pressed={isOnHold}
  >
    {#if isOnHold}
      <Play class="h-6 w-6" />
    {:else}
      <Pause class="h-6 w-6" />
    {/if}
    <span class="text-xs font-medium">{isOnHold ? "Resume" : "Hold"}</span>
  </button>

  <!-- Keypad Button -->
  <button
    type="button"
    onclick={handleKeypad}
    class="flex flex-col items-center justify-center gap-2 p-4 rounded-lg border bg-white border-gray-200 text-gray-700 hover:bg-gray-50 transition-all focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
    aria-label="Open keypad"
    aria-pressed={showKeypad}
  >
    <Grid3x3 class="h-6 w-6" />
    <span class="text-xs font-medium">Pad</span>
  </button>

  <!-- More Button -->
  <button
    type="button"
    onclick={handleMore}
    class="flex flex-col items-center justify-center gap-2 p-4 rounded-lg border bg-white border-gray-200 text-gray-700 hover:bg-gray-50 transition-all focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
    aria-label="More options"
    aria-pressed={showMoreMenu}
    aria-haspopup="true"
  >
    <MoreVertical class="h-6 w-6" />
    <span class="text-xs font-medium">More</span>
  </button>
</div>
