<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import { Phone } from "lucide-svelte";

  interface Props {
    disabled?: boolean;
    onCall?: () => void;
  }

  let { disabled = false, onCall }: Props = $props();

  function handleCall() {
    if (!disabled && onCall) {
      onCall();
    }
  }
</script>

<Button
  type="button"
  onclick={handleCall}
  disabled={disabled}
  variant="default"
  size="lg"
  class="w-full h-12 text-base font-semibold bg-primary hover:bg-primary-hover"
  aria-label="Make call"
  aria-describedby={disabled ? "call-disabled-hint" : undefined}
>
  <Phone class="h-5 w-5 mr-2" />
  Call
</Button>
{#if disabled}
  <span id="call-disabled-hint" class="sr-only"
    >Enter a phone number to enable call button</span
  >
{/if}

