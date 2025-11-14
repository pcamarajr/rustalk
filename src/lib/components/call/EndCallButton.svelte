<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import { PhoneOff } from "lucide-svelte";

  function handleEndCall() {
    console.log("DEBUG:[CALL/END] Ending call");
    // TODO: Connect to call store in UI-2.6, then to Tauri command in Phase 5
  }
</script>

<Button
  type="button"
  onclick={handleEndCall}
  variant="destructive"
  size="lg"
  class="w-full h-12 text-base font-semibold bg-red-500 hover:bg-red-600"
  aria-label="End call"
>
  <PhoneOff class="h-5 w-5 mr-2" />
  End Call
</Button>
