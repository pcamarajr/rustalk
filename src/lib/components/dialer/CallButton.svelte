<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import { Phone } from "lucide-svelte";

  interface Props {
    disabled?: boolean;
    loading?: boolean;
    onCall?: () => void;
  }

  let { disabled = false, loading = false, onCall }: Props = $props();

  function handleCall() {
    if (!disabled && !loading && onCall) {
      onCall();
    }
  }
</script>

<Button
  type="button"
  onclick={handleCall}
  disabled={disabled || loading}
  variant="default"
  size="lg"
  class="w-full h-12 text-base font-semibold bg-primary hover:bg-primary-hover"
  aria-label={loading ? "Calling..." : "Make call"}
  aria-describedby={disabled ? "call-disabled-hint" : undefined}
>
  {#if loading}
    <span class="mr-2">⏳</span>
    Calling...
  {:else}
    <Phone class="h-5 w-5 mr-2" />
    Call
  {/if}
</Button>
{#if disabled}
  <span id="call-disabled-hint" class="sr-only"
    >Enter a phone number to enable call button</span
  >
{/if}

