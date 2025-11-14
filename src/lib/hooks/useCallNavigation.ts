import { goto } from "$app/navigation";

/**
 * Composable for handling call navigation
 * 
 * This is a temporary solution for mocking. When real call functionality is implemented,
 * this can be extended to handle actual call initiation.
 * 
 * @returns A function that navigates to the dialer with the given number
 */
export function useCallNavigation() {
  /**
   * Navigate to dialer with number pre-filled
   * @param number - The phone number to call
   */
  function initiateCall(number: string) {
    console.log("DEBUG:[CALL/NAVIGATION] Initiating call to:", number);
    // Navigate to dialer with number pre-filled
    // For now, we'll navigate to home and the dialer should accept a query param
    goto(`/?number=${encodeURIComponent(number)}`);
  }

  return {
    initiateCall,
  };
}

