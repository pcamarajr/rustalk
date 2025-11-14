<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import { PhoneIncoming } from "lucide-svelte";

  interface Props {
    onSimulate?: () => void;
  }

  let { onSimulate }: Props = $props();

  function handleSimulate() {
    if (onSimulate) {
      onSimulate();
    }
  }
</script>

<Button
  type="button"
  onclick={handleSimulate}
  variant="outline"
  size="lg"
  class="w-full h-12 text-base font-semibold border-2 border-dashed border-gray-300 hover:border-gray-400 hover:bg-gray-50"
  aria-label="Simulate incoming call (for testing)"
>
  <PhoneIncoming class="h-5 w-5 mr-2" />
  Simulate Incoming Call
</Button>

