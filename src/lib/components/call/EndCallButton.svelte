<script lang="ts">
  import { goto } from "$app/navigation";
  import { Button } from "$lib/components/ui/button/index.js";
  import { PhoneOff } from "lucide-svelte";
  import { callStore, callState } from "$lib/stores/callStore";

  function handleEndCall() {
    console.log("DEBUG:[CALL/END] Ending call");
    callStore.endCall();
  }

  // Navigate back to dialer when call ends
  $effect(() => {
    const unsubscribe = callState.subscribe((state) => {
      if (state === "idle") {
        goto("/");
      }
    });
    return unsubscribe;
  });
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
