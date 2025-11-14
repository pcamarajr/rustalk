<script lang="ts">
  // Container island - simple shell that composes sub-islands
  import CallHeader from "./CallHeader.svelte";
  import CallTimer from "./CallTimer.svelte";
  import AudioVisualizer from "./AudioVisualizer.svelte";
  import CallControls from "./CallControls.svelte";
  import EndCallButton from "./EndCallButton.svelte";
</script>

<div class="flex flex-col gap-6 p-6" role="main" aria-label="Active call">
  <CallHeader />
  <CallTimer />
  <AudioVisualizer />
  <CallControls />
  <EndCallButton />
</div>
