<script lang="ts">
  import {
    Dialog,
    DialogContent,
    DialogHeader,
    DialogTitle,
  } from "$lib/components/ui/dialog";
  import DialPad from "$lib/components/shared/DialPad.svelte";

  interface Props {
    open?: boolean;
    onOpenChange?: (open: boolean) => void;
  }

  let { open = $bindable(false), onOpenChange }: Props = $props();

  function handleKeyPress(key: string) {
    console.log("DEBUG:[CALL/KEYPAD] Sending DTMF tone:", key);
    // TODO: Send DTMF tone via callStore in Phase 5
    // For now, just log the key press
  }

  function handleOpenChange(newOpen: boolean) {
    open = newOpen;
    onOpenChange?.(newOpen);
  }
</script>

<Dialog bind:open onOpenChange={handleOpenChange}>
  <DialogContent class="max-w-sm">
    <DialogHeader>
      <DialogTitle>Keypad</DialogTitle>
    </DialogHeader>
    <div class="py-4">
      <DialPad onKeyPress={handleKeyPress} />
    </div>
  </DialogContent>
</Dialog>
