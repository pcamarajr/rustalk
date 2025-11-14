<script lang="ts">
  import { Phone, Star } from "lucide-svelte";
  import { Button } from "$lib/components/ui/button";
  import ContactDetails from "./ContactDetails.svelte";
  import { useCallNavigation } from "$lib/hooks/useCallNavigation";
  import { formatPhoneNumber } from "$lib/utils";
  import type { Contact } from "$lib/stores/contactsStore";

  interface Props {
    contact: Contact;
    showFavorite?: boolean;
  }

  let { contact, showFavorite = true }: Props = $props();

  // Reference to ContactDetails component
  let contactDetailsRef: ContactDetails | null = null;

  // Use call navigation composable
  const { initiateCall } = useCallNavigation();

  function getInitials(name: string): string {
    return name
      .split(" ")
      .map((n) => n[0])
      .join("")
      .toUpperCase()
      .slice(0, 2);
  }

  function getPrimaryNumber(): string {
    // Prefer mobile, then work, then first available
    const mobile = contact.numbers.find((n) => n.type === "mobile");
    if (mobile) return mobile.number;
    const work = contact.numbers.find((n) => n.type === "work");
    if (work) return work.number;
    return contact.numbers[0]?.number || "";
  }

  function handleCall(event: MouseEvent) {
    event.stopPropagation();
    const number = getPrimaryNumber();
    if (number) {
      initiateCall(number);
    }
  }

  function handleClick() {
    contactDetailsRef?.openDialog();
  }
</script>

<div
  class="flex items-center gap-3 p-3 hover:bg-gray-50 rounded-lg cursor-pointer transition-colors"
  role="button"
  tabindex="0"
  onclick={handleClick}
  onkeydown={(e) => {
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      handleClick();
    }
  }}
>
  <!-- Avatar -->
  <div
    class="w-10 h-10 rounded-full bg-linear-to-br from-blue-400 to-blue-600 flex items-center justify-center text-white text-sm font-semibold shrink-0"
  >
    {getInitials(contact.name)}
  </div>

  <!-- Contact Info -->
  <div class="flex-1 min-w-0">
    <div class="flex items-center gap-2">
      <span class="font-medium text-gray-900 truncate">{contact.name}</span>
      {#if contact.favorite && showFavorite}
        <Star class="h-4 w-4 text-yellow-500 fill-yellow-500 shrink-0" />
      {/if}
    </div>
    <div class="text-sm text-gray-500 truncate">
      {formatPhoneNumber(getPrimaryNumber())}
    </div>
  </div>

  <!-- Quick Call Button -->
  <Button
    variant="ghost"
    size="icon-sm"
    onclick={handleCall}
    aria-label={`Call ${contact.name}`}
  >
    <Phone class="h-4 w-4" />
  </Button>
</div>

<!-- Contact Details (self-contained) -->
<ContactDetails bind:this={contactDetailsRef} contactId={contact.id} />
