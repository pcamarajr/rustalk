<script lang="ts">
  // Self-contained timer state
  let callDuration = $state(0);

  // Format duration as MM:SS or HH:MM:SS
  let formattedTime = $derived.by(() => {
    const hours = Math.floor(callDuration / 3600);
    const minutes = Math.floor((callDuration % 3600) / 60);
    const seconds = callDuration % 60;

    if (hours > 0) {
      return `${String(hours).padStart(2, "0")}:${String(minutes).padStart(2, "0")}:${String(seconds).padStart(2, "0")}`;
    }
    return `${String(minutes).padStart(2, "0")}:${String(seconds).padStart(2, "0")}`;
  });

  // Update timer every second
  $effect(() => {
    const interval = setInterval(() => {
      callDuration += 1;
    }, 1000);

    // Cleanup on component destroy
    return () => {
      clearInterval(interval);
    };
  });
</script>

<div class="text-center">
  <div
    class="text-lg font-medium text-gray-700 flex items-center justify-center gap-2"
    role="timer"
    aria-live="polite"
    aria-label="Call duration: {formattedTime}"
  >
    <span class="text-xl">⏱</span>
    <span>{formattedTime}</span>
  </div>
</div>

