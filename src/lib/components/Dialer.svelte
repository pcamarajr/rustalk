<script lang="ts">
  import { goto } from "$app/navigation";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import {
    Phone,
    X,
    PhoneIncoming,
    PhoneOutgoing,
    PhoneMissed,
  } from "lucide-svelte";

  // Phone number state (stored as digits only)
  let phoneNumber = $state("");

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

  // Check if call button should be disabled
  let isCallDisabled = $derived(!phoneNumber || phoneNumber.length === 0);

  // Dial pad configuration
  const dialPadKeys = [
    { number: "1", letters: "" },
    { number: "2", letters: "ABC" },
    { number: "3", letters: "DEF" },
    { number: "4", letters: "GHI" },
    { number: "5", letters: "JKL" },
    { number: "6", letters: "MNO" },
    { number: "7", letters: "PQRS" },
    { number: "8", letters: "TUV" },
    { number: "9", letters: "WXYZ" },
    { number: "*", letters: "" },
    { number: "0", letters: "" },
    { number: "#", letters: "" },
  ];

  // Mock recent calls data
  type CallDirection = "incoming" | "outgoing" | "missed";
  const recentCalls: Array<{
    name: string;
    number: string;
    time: string;
    direction: CallDirection;
  }> = [
    {
      name: "Mom (Mobile)",
      number: "+1 (555) 123-4567",
      time: "2m ago",
      direction: "incoming",
    },
    {
      name: "Work - Conference",
      number: "+1 (555) 987-6543",
      time: "1h ago",
      direction: "outgoing",
    },
    {
      name: "Sarah Johnson",
      number: "+1 (555) 456-7890",
      time: "2h ago",
      direction: "outgoing",
    },
  ];

  function handleDialPadClick(key: string) {
    phoneNumber += key;
  }

  function handleClear() {
    phoneNumber = "";
  }

  function handleCall() {
    if (!isCallDisabled) {
      console.log("DEBUG:[DIALER/CALL] Initiating call to:", phoneNumber);
      // Navigate to active call screen
      goto("/active-call");
      // TODO: Connect to call store in UI-2.6
    }
  }

  function handleSimulateIncomingCall() {
    console.log("DEBUG:[DIALER/SIMULATE] Simulating incoming call");
    // Navigate to incoming call screen for testing
    goto("/incoming-call");
    // TODO: Connect to call store in UI-2.6, then to Tauri event in Phase 5
  }

  function handleNumberInput(event: Event) {
    const target = event.target as HTMLInputElement;
    // Strip non-digits and update phoneNumber
    phoneNumber = target.value.replace(/\D/g, "");
  }

  function handleRecentCallClick(number: string) {
    // Set the phone number from recent call
    phoneNumber = number.replace(/\D/g, "");
  }

  function getDirectionIcon(direction: "incoming" | "outgoing" | "missed") {
    switch (direction) {
      case "incoming":
        return PhoneIncoming;
      case "outgoing":
        return PhoneOutgoing;
      case "missed":
        return PhoneMissed;
    }
  }

  function handleKeyPress(event: KeyboardEvent) {
    // Only handle keys when not typing in input field
    const target = event.target as HTMLElement;
    const key = event.key;

    if (target.tagName === "INPUT") {
      // Let input handle its own events, but allow Enter to trigger call
      if (key === "Enter" && !isCallDisabled) {
        event.preventDefault();
        handleCall();
      }
      return;
    }

    // Handle dial pad keys when focus is elsewhere
    if (key >= "0" && key <= "9") {
      event.preventDefault();
      handleDialPadClick(key);
    } else if (key === "*" || key === "#") {
      event.preventDefault();
      handleDialPadClick(key);
    } else if (key === "Backspace" || key === "Delete") {
      event.preventDefault();
      phoneNumber = phoneNumber.slice(0, -1);
    } else if (key === "Enter" && !isCallDisabled) {
      event.preventDefault();
      handleCall();
    }
  }

  // Focus management - focus input on mount
  let inputRef: HTMLInputElement | null = $state(null);

  $effect(() => {
    if (inputRef) {
      inputRef.focus();
    }

    // Add document-level keyboard handler
    document.addEventListener("keydown", handleKeyPress);
    return () => {
      document.removeEventListener("keydown", handleKeyPress);
    };
  });
</script>

<div
  class="flex flex-col gap-6 p-6"
  role="application"
  aria-label="Phone dialer"
>
  <!-- Number Input Field -->
  <div class="relative">
    <Input
      bind:ref={inputRef}
      type="tel"
      value={formattedNumber}
      oninput={handleNumberInput}
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

  <!-- Dial Pad -->
  <div class="grid grid-cols-3 gap-2" role="group" aria-label="Dial pad">
    {#each dialPadKeys as key}
      <button
        type="button"
        onclick={() => handleDialPadClick(key.number)}
        class="flex flex-col items-center justify-center h-16 rounded-lg bg-white border border-gray-200 hover:bg-gray-50 active:bg-gray-100 active:scale-95 transition-all focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
        aria-label={`Dial ${key.number}${key.letters ? ` (${key.letters})` : ""}`}
      >
        <span class="text-2xl font-semibold text-gray-900">{key.number}</span>
        {#if key.letters}
          <span class="text-xs text-gray-500 mt-0.5">{key.letters}</span>
        {/if}
      </button>
    {/each}
  </div>

  <!-- Call Button -->
  <Button
    type="button"
    onclick={handleCall}
    disabled={isCallDisabled}
    variant="default"
    size="lg"
    class="w-full h-12 text-base font-semibold bg-primary hover:bg-primary-hover"
    aria-label="Make call"
    aria-describedby={isCallDisabled ? "call-disabled-hint" : undefined}
  >
    <Phone class="h-5 w-5 mr-2" />
    Call
  </Button>
  {#if isCallDisabled}
    <span id="call-disabled-hint" class="sr-only"
      >Enter a phone number to enable call button</span
    >
  {/if}

  <!-- Mock: Simulate Incoming Call Button (for testing) -->
  <Button
    type="button"
    onclick={handleSimulateIncomingCall}
    variant="outline"
    size="lg"
    class="w-full h-12 text-base font-semibold border-2 border-dashed border-gray-300 hover:border-gray-400 hover:bg-gray-50"
    aria-label="Simulate incoming call (for testing)"
  >
    <PhoneIncoming class="h-5 w-5 mr-2" />
    Simulate Incoming Call
  </Button>

  <!-- Recent Calls Preview -->
  {#if recentCalls.length > 0}
    <div class="mt-4">
      <h3
        class="text-sm font-semibold text-gray-700 mb-3"
        id="recent-calls-heading"
      >
        Recent Calls:
      </h3>
      <div class="space-y-2" role="list" aria-labelledby="recent-calls-heading">
        {#each recentCalls as call}
          {@const DirectionIcon = getDirectionIcon(call.direction)}
          <div role="listitem">
            <button
              type="button"
              onclick={() => handleRecentCallClick(call.number)}
              class="w-full flex items-center justify-between p-3 rounded-lg hover:bg-gray-50 active:bg-gray-100 transition-colors text-left focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
              aria-label={`Call ${call.name} at ${call.number}`}
            >
              <div class="flex items-center gap-3 flex-1 min-w-0">
                <DirectionIcon
                  class="h-5 w-5 shrink-0 {call.direction === 'missed'
                    ? 'text-red-500'
                    : call.direction === 'incoming'
                      ? 'text-green-500'
                      : 'text-blue-500'}"
                />
                <div class="flex-1 min-w-0">
                  <div class="text-sm font-medium text-gray-900 truncate">
                    {call.name}
                  </div>
                  <div class="text-xs text-gray-500 truncate">
                    {call.number}
                  </div>
                </div>
              </div>
              <div class="text-xs text-gray-500 shrink-0 ml-2">{call.time}</div>
            </button>
          </div>
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  /* Additional styles if needed */
</style>
