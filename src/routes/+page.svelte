<script lang="ts">
  import Dialer from "$lib/components/Dialer.svelte";
  import { page } from "$app/stores";

  // Get number from query parameter
  let initialNumber = $derived($page.url.searchParams.get("number") || "");
</script>

<div class="container mx-auto max-w-md px-4 py-6">
  <Dialer initialNumber={initialNumber} />
</div>
