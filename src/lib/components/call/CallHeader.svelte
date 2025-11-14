<script lang="ts">
  // Self-contained state for contact information
  let contactName = $state("Sarah Johnson");
  let phoneNumber = $state("+1 (555) 987-6543");

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

