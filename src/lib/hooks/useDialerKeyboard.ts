/**
 * Composable for handling dialer keyboard interactions
 * 
 * Handles:
 * - Dial pad keys (0-9, *, #)
 * - Backspace/Delete for removing digits
 * - Enter to trigger call
 * 
 * @param callbacks - Object containing callback functions
 * @returns Cleanup function to remove event listeners
 */
export function useDialerKeyboard(callbacks: {
  onDigit?: (digit: string) => void;
  onBackspace?: () => void;
  onEnter?: () => void;
}) {
  let isMounted = true;

  function handleKeyPress(event: KeyboardEvent) {
    // Prevent execution after component unmounts
    if (!isMounted) return;

    // Only handle keys when not typing in input field
    const target = event.target as HTMLElement;
    const key = event.key;

    if (target.tagName === "INPUT") {
      // Let input handle its own events, but allow Enter to trigger call
      if (key === "Enter" && callbacks.onEnter) {
        event.preventDefault();
        callbacks.onEnter();
      }
      return;
    }

    // Handle dial pad keys when focus is elsewhere
    if (key >= "0" && key <= "9") {
      event.preventDefault();
      callbacks.onDigit?.(key);
    } else if (key === "*" || key === "#") {
      event.preventDefault();
      callbacks.onDigit?.(key);
    } else if (key === "Backspace" || key === "Delete") {
      event.preventDefault();
      callbacks.onBackspace?.();
    } else if (key === "Enter" && callbacks.onEnter) {
      event.preventDefault();
      callbacks.onEnter();
    }
  }

  // Add document-level keyboard handler
  document.addEventListener("keydown", handleKeyPress);

  // Return cleanup function
  return () => {
    isMounted = false;
    document.removeEventListener("keydown", handleKeyPress);
  };
}

