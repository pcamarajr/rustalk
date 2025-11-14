<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import { Phone, PhoneOff } from "lucide-svelte";

  function handleAnswer() {
    console.log("DEBUG:[CALL/ANSWER] Answering call");
    // TODO: Connect to call store in UI-2.6, then to Tauri command in Phase 5
  }

  function handleDecline() {
    console.log("DEBUG:[CALL/DECLINE] Declining call");
    // TODO: Connect to call store in UI-2.6, then to Tauri command in Phase 5
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

