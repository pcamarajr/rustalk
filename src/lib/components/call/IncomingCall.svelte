<script lang="ts">
  // Container island - simple shell that composes sub-islands
  import CallHeader from "./CallHeader.svelte";
  import IncomingCallStatus from "./IncomingCallStatus.svelte";
  import IncomingCallActions from "./IncomingCallActions.svelte";
</script>

<div class="flex flex-col gap-6 p-6" role="main" aria-label="Incoming call">
  <CallHeader />
  <IncomingCallStatus />
  <IncomingCallActions />
</div>

