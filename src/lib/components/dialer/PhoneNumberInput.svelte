<script lang="ts">
  import { Input } from "$lib/components/ui/input/index.js";
  import { X } from "lucide-svelte";

  interface Props {
    initialNumber?: string;
    onNumberChange?: (number: string) => void;
  }

  let { initialNumber = "", onNumberChange }: Props = $props();

  // Phone number state (stored as digits only)
  let phoneNumber = $state(initialNumber ? initialNumber.replace(/\D/g, "") : "");

  // Update phone number when initialNumber prop changes
  $effect(() => {
    phoneNumber = initialNumber ? initialNumber.replace(/\D/g, "") : "";
  });

  // Format phone number for display
  let formattedNumber = $derived.by(() => {
    if (!phoneNumber) return "";
    const digits = phoneNumber.replace(/\D/g, "");

    // Format as +1 (555) 123-4567 for US numbers
    if (digits.length <= 1) {
      return digits;
    } else if (digits.length <= 3) {
      return `+${digits}`;
    } else if (digits.length <= 6) {
      return `+${digits.slice(0, 1)} (${digits.slice(1)})`;
    } else if (digits.length <= 10) {
      return `+${digits.slice(0, 1)} (${digits.slice(1, 4)}) ${digits.slice(4)}`;
    } else {
      return `+${digits.slice(0, 1)} (${digits.slice(1, 4)}) ${digits.slice(4, 7)}-${digits.slice(7, 11)}`;
    }
  });

  // Expose current number via callback
  $effect(() => {
    onNumberChange?.(phoneNumber);
  });

  function handleInput(event: Event) {
    const target = event.target as HTMLInputElement;
    // Strip non-digits and update phoneNumber
    phoneNumber = target.value.replace(/\D/g, "");
  }

  function handleClear() {
    phoneNumber = "";
    shouldFocus = true;
  }

  // Expose methods for parent components
  export function getNumber(): string {
    return phoneNumber;
  }

  export function setNumber(number: string) {
    phoneNumber = number.replace(/\D/g, "");
  }

  export function addDigit(digit: string) {
    phoneNumber += digit;
  }

  export function removeLastDigit() {
    phoneNumber = phoneNumber.slice(0, -1);
  }

  // Focus management
  let inputRef: HTMLInputElement | null = $state(null);
  let shouldFocus = $state(false);

  $effect(() => {
    if (inputRef && shouldFocus) {
      inputRef.focus();
      shouldFocus = false;
    }
  });
</script>

<div class="relative">
  <Input
    bind:ref={inputRef}
    type="tel"
    value={formattedNumber}
    oninput={handleInput}
    placeholder="Enter number or name"
    class="h-14 text-xl font-medium pr-12"
    aria-label="Phone number input"
    aria-describedby="dialer-instructions"
  />
  <span id="dialer-instructions" class="sr-only">
    Enter phone number using dial pad or keyboard. Press Enter to call.
  </span>
  {#if phoneNumber}
    <button
      type="button"
      onclick={handleClear}
      class="absolute right-3 top-1/2 -translate-y-1/2 rounded-full p-1 hover:bg-gray-100 transition-colors"
      aria-label="Clear phone number"
    >
      <X class="h-5 w-5 text-gray-500" />
    </button>
  {/if}
</div>

