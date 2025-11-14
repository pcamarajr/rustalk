<script lang="ts">
  import { Label } from "$lib/components/ui/label";
  import { Switch } from "$lib/components/ui/switch";

  // Self-contained state
  let echoCancellation = $state(true);
  let noiseSuppression = $state(true);
</script>

<div class="space-y-4">
  <Label class="text-base font-semibold">Audio Processing</Label>
  <div class="space-y-4">
    <div class="flex items-center justify-between">
      <div class="space-y-0.5">
        <Label for="echo-cancellation" class="text-sm font-normal"
          >Echo Cancellation</Label
        >
        <p class="text-xs text-gray-500">Reduce echo during calls</p>
      </div>
      <Switch id="echo-cancellation" bind:checked={echoCancellation} />
    </div>
    <div class="flex items-center justify-between">
      <div class="space-y-0.5">
        <Label for="noise-suppression" class="text-sm font-normal"
          >Noise Suppression</Label
        >
        <p class="text-xs text-gray-500">Reduce background noise</p>
      </div>
      <Switch id="noise-suppression" bind:checked={noiseSuppression} />
    </div>
  </div>
</div>

