<script lang="ts">
  import { Phone } from "lucide-svelte";
</script>

<div class="text-center">
  <div
    class="text-lg font-medium text-gray-700 flex items-center justify-center gap-2"
    role="status"
    aria-live="polite"
    aria-label="Incoming call"
  >
    <Phone class="h-5 w-5" />
    <span>Incoming call...</span>
  </div>
</div>

