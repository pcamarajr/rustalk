<script lang="ts">
  // Container island - simple shell that composes sub-islands
  import { goto } from "$app/navigation";
  import { callStore, isCallActive } from "$lib/stores/callStore";
  import PhoneNumberInput from "./PhoneNumberInput.svelte";
  import DialPad from "../shared/DialPad.svelte";
  import CallButton from "./CallButton.svelte";
  import RecentCallsList from "./RecentCallsList.svelte";
  import SimulateCallButton from "./SimulateCallButton.svelte";
  import { useDialerKeyboard } from "$lib/hooks/useDialerKeyboard";

  interface Props {
    initialNumber?: string;
  }

  let { initialNumber = "" }: Props = $props();

  // Reference to PhoneNumberInput component to call its methods
  let phoneNumberInput: PhoneNumberInput | null = null;

  // Current phone number state (derived from PhoneNumberInput)
  let currentPhoneNumber = $state("");

  // Error state for call initiation
  let callError = $state<string | null>(null);

  // Check if call button should be disabled
  let isCallDisabled = $derived(
    !currentPhoneNumber || currentPhoneNumber.length === 0
  );

  // Handle phone number changes from PhoneNumberInput
  function handleNumberChange(number: string) {
    currentPhoneNumber = number;
  }

  // Handle dial pad key press
  function handleDialPadKey(key: string) {
    phoneNumberInput?.addDigit(key);
  }

  // Handle call button click
  async function handleCall() {
    if (!isCallDisabled && phoneNumberInput) {
      // Clear any previous error
      callError = null;
      
      const number = phoneNumberInput.getNumber();
      console.log("DEBUG:[DIALER/CALL] Initiating call to:", number);
      // Format number with +1 prefix if not already formatted
      const formattedNumber = number.startsWith("+") ? number : `+1${number}`;
      try {
        await callStore.initiateCall(formattedNumber);
      } catch (error) {
        console.error("DEBUG:[DIALER/CALL] Failed to initiate call:", error);
        // Set error state for inline display
        if (error instanceof Error) {
          callError = error.message;
        } else {
          callError = "Failed to initiate call. Please try again.";
        }
      }
    }
  }

  // Handle simulate incoming call
  function handleSimulateIncomingCall() {
    console.log("DEBUG:[DIALER/SIMULATE] Simulating incoming call");
    // Use a mock number for incoming call simulation
    const mockNumber = "+15551234567";
    callStore.simulateIncomingCall(mockNumber);
    goto("/incoming-call");
  }

  // Handle recent call click
  function handleRecentCallClick(number: string) {
    phoneNumberInput?.setNumber(number);
  }

  // Track navigation state to prevent duplicate navigations
  let hasNavigated = $state(false);

  // Watch for call state changes and navigate when call becomes active
  $effect(() => {
    const unsubscribe = isCallActive.subscribe((active) => {
      if (active && !hasNavigated) {
        hasNavigated = true;
        goto("/active-call");
      } else if (!active) {
        hasNavigated = false;
      }
    });
    return unsubscribe;
  });

  // Set up keyboard handling
  $effect(() => {
    const cleanup = useDialerKeyboard({
      onDigit: (digit) => {
        phoneNumberInput?.addDigit(digit);
      },
      onBackspace: () => {
        phoneNumberInput?.removeLastDigit();
      },
      onEnter: () => {
        if (!isCallDisabled) {
          handleCall();
        }
      },
    });
    return cleanup;
  });
</script>

<div
  class="flex flex-col gap-6 p-6"
  role="application"
  aria-label="Phone dialer"
>
  <PhoneNumberInput
    bind:this={phoneNumberInput}
    {initialNumber}
    onNumberChange={handleNumberChange}
  />
  
  <!-- Error Display -->
  {#if callError}
    <div class="rounded-md bg-destructive/10 border border-destructive/20 p-3" role="alert">
      <p class="text-sm text-destructive">{callError}</p>
    </div>
  {/if}
  
  <DialPad onKeyPress={handleDialPadKey} />
  <CallButton disabled={isCallDisabled} onCall={handleCall} />
  <SimulateCallButton onSimulate={handleSimulateIncomingCall} />
  <RecentCallsList onCallClick={handleRecentCallClick} />
</div>
