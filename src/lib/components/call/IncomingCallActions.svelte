<script lang="ts">
  import { goto } from "$app/navigation";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Phone, PhoneOff } from "lucide-svelte";
  import { callStore, isCallActive } from "$lib/stores/callStore";

  function handleAnswer() {
    console.log("DEBUG:[CALL/ANSWER] Answering call");
    callStore.answerCall();
    // Navigate to active call when answered
    goto("/active-call");
  }

  function handleDecline() {
    console.log("DEBUG:[CALL/DECLINE] Declining call");
    callStore.declineCall();
    // Navigate back to dialer
    goto("/");
  }
</script>

<div class="flex gap-4">
  <!-- Decline Button -->
  <Button
    type="button"
    onclick={handleDecline}
    variant="destructive"
    size="lg"
    class="flex-1 h-14 text-base font-semibold bg-red-500 hover:bg-red-600"
    aria-label="Decline call"
  >
    <PhoneOff class="h-5 w-5 mr-2" />
    Decline
  </Button>

  <!-- Answer Button -->
  <Button
    type="button"
    onclick={handleAnswer}
    variant="default"
    size="lg"
    class="flex-1 h-14 text-base font-semibold bg-green-500 hover:bg-green-600 text-white"
    aria-label="Answer call"
  >
    <Phone class="h-5 w-5 mr-2" />
    Answer
  </Button>
</div>

