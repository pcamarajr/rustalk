<script lang="ts">
  import { callStore } from "$lib/stores/callStore";

  // Use store subscription for reactivity
  let currentCall = $state<{ name: string | null; number: string } | null>(null);

  $effect(() => {
    const unsubscribe = callStore.subscribe((call) => {
      if (call) {
        currentCall = { name: call.name, number: call.number };
      } else {
        currentCall = null;
      }
    });
    return unsubscribe;
  });

  // Default values if no call
  let contactName = $derived(currentCall?.name || "Unknown");
  let phoneNumber = $derived(currentCall?.number || "");

  // Generate initials from contact name
  let initials = $derived.by(() => {
    const parts = contactName.split(" ");
    if (parts.length >= 2) {
      return `${parts[0][0]}${parts[1][0]}`.toUpperCase();
    }
    return contactName.substring(0, 2).toUpperCase();
  });
</script>

<div class="flex flex-col items-center gap-4">
  <!-- Avatar Circle with Initials -->
  <div
    class="flex items-center justify-center w-24 h-24 rounded-full bg-primary text-white text-2xl font-semibold"
    role="img"
    aria-label="Contact avatar for {contactName}"
  >
    {initials}
  </div>

  <!-- Contact Name -->
  <div class="text-center">
    <h2 class="text-2xl font-bold text-gray-900">{contactName}</h2>
    <p class="text-sm text-gray-500 mt-1">{phoneNumber}</p>
  </div>
</div>

