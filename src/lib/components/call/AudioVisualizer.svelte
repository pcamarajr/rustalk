<script lang="ts">
  import { Volume2 } from "lucide-svelte";

  /**
   * Audio Visualizer Component
   * 
   * NOTE: This is a mock implementation for OUT-2.6.
   * Real audio levels will be implemented when the backend provides
   * a `get_audio_levels` Tauri command in a future phase.
   * 
   * The component currently displays simulated audio levels with random
   * values for visual feedback during calls.
   */

  // Self-contained mock audio level state
  let incomingLevel = $state(50);
  let outgoingLevel = $state(45);

  // Animate audio levels with mock random values
  $effect(() => {
    const interval = setInterval(() => {
      // Random values between 20-80 for demo
      incomingLevel = Math.floor(Math.random() * 60) + 20;
      outgoingLevel = Math.floor(Math.random() * 60) + 20;
    }, 150); // Update every 150ms for smooth animation

    // Cleanup on component destroy
    return () => {
      clearInterval(interval);
    };
  });

  // Helper to get width percentage for progress bars
  let incomingWidth = $derived(`${incomingLevel}%`);
  let outgoingWidth = $derived(`${outgoingLevel}%`);
</script>

<div class="w-full">
  <div
    class="flex flex-col gap-3 p-4 bg-gray-50 rounded-lg border border-gray-200"
    role="region"
    aria-label="Audio levels"
  >
    <!-- Header -->
    <div class="flex items-center justify-between">
      <span class="text-sm font-medium text-gray-700">Audio:</span>
      <Volume2 class="h-5 w-5 text-gray-500" role="img" aria-label="Speaker" />
    </div>

    <!-- Incoming Audio Bar -->
    <div class="flex items-center gap-2">
      <span class="text-xs text-gray-500 w-8">In</span>
      <div
        class="flex-1 h-[26px] bg-gray-200 rounded-full overflow-hidden"
        role="progressbar"
        aria-valuenow={incomingLevel}
        aria-valuemin="0"
        aria-valuemax="100"
        aria-label="Incoming audio level"
      >
        <div
          class="h-full bg-primary transition-all duration-150 ease-linear rounded-full"
          style="width: {incomingWidth}"
        ></div>
      </div>
    </div>

    <!-- Outgoing Audio Bar -->
    <div class="flex items-center gap-2">
      <span class="text-xs text-gray-500 w-8">Out</span>
      <div
        class="flex-1 h-[26px] bg-gray-200 rounded-full overflow-hidden"
        role="progressbar"
        aria-valuenow={outgoingLevel}
        aria-valuemin="0"
        aria-valuemax="100"
        aria-label="Outgoing audio level"
      >
        <div
          class="h-full bg-green-500 transition-all duration-150 ease-linear rounded-full"
          style="width: {outgoingWidth}"
        ></div>
      </div>
    </div>
  </div>
</div>
