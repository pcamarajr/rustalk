<script lang="ts">
  import { goto } from "$app/navigation";
  import { Settings, ChevronDown, User } from "lucide-svelte";
  import { Button } from "$lib/components/ui/button/index.js";

  // Mock user data - will be replaced with store in UI-2.6
  let userName = $state("John Doe");
  let connectionStatus = $state<"connected" | "disconnected">("connected");
  let showProfileDropdown = $state(false);

  // Toggle profile dropdown
  function toggleProfileDropdown() {
    showProfileDropdown = !showProfileDropdown;
  }

  // Close dropdown when clicking outside
  function handleClickOutside(event: MouseEvent) {
    const target = event.target as HTMLElement;
    if (!target.closest(".profile-dropdown")) {
      showProfileDropdown = false;
    }
  }

  // Handle settings navigation
  function handleSettingsClick() {
    console.log("DEBUG:[HEADER/SETTINGS] Navigating to settings");
    goto("/settings");
  }

  // Handle profile dropdown actions
  function handleProfileAction(action: string) {
    console.log("DEBUG:[HEADER/PROFILE] Action:", action);
    showProfileDropdown = false;
    // TODO: Implement profile actions (edit profile, logout, etc.)
  }

  // Set up click outside listener
  $effect(() => {
    if (showProfileDropdown) {
      document.addEventListener("click", handleClickOutside);
      return () => {
        document.removeEventListener("click", handleClickOutside);
      };
    }
  });
</script>

<header
  class="h-16 bg-white border-b border-gray-200 px-6 flex items-center justify-between"
>
  <!-- App Title -->
  <h1 class="text-xl font-semibold text-gray-900">Rustalk</h1>

  <!-- Right side: Profile and Settings -->
  <div class="flex items-center gap-4">
    <!-- Profile Dropdown -->
    <div class="relative profile-dropdown">
      <button
        type="button"
        onclick={toggleProfileDropdown}
        class="flex items-center gap-2 px-3 py-2 rounded-lg hover:bg-gray-50 transition-colors focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
        aria-label="User profile menu"
        aria-expanded={showProfileDropdown}
        aria-haspopup="true"
      >
        <div class="flex items-center gap-2">
          <div
            class="w-8 h-8 rounded-full bg-linear-to-br from-blue-400 to-blue-600 flex items-center justify-center text-white text-sm font-semibold"
          >
            {userName
              .split(" ")
              .map((n) => n[0])
              .join("")}
          </div>
          <div class="flex flex-col items-start">
            <span class="text-sm font-medium text-gray-900">{userName}</span>
            <span class="text-xs text-gray-500 flex items-center gap-1">
              <span
                class="w-1.5 h-1.5 rounded-full {connectionStatus ===
                'connected'
                  ? 'bg-green-500'
                  : 'bg-gray-400'}"
              ></span>
              {connectionStatus === "connected" ? "Connected" : "Disconnected"}
            </span>
          </div>
        </div>
        <ChevronDown
          class="h-4 w-4 text-gray-500 transition-transform {showProfileDropdown
            ? 'rotate-180'
            : ''}"
        />
      </button>

      <!-- Dropdown Menu -->
      {#if showProfileDropdown}
        <div
          class="absolute right-0 mt-2 w-56 bg-white rounded-lg shadow-lg border border-gray-200 py-1 z-50"
          role="menu"
          aria-orientation="vertical"
        >
          <div class="px-4 py-3 border-b border-gray-200">
            <p class="text-sm font-medium text-gray-900">{userName}</p>
            <p class="text-xs text-gray-500 mt-0.5">
              {connectionStatus === "connected" ? "Connected" : "Disconnected"}
            </p>
          </div>
          <button
            type="button"
            onclick={() => handleProfileAction("edit")}
            class="w-full px-4 py-2 text-left text-sm text-gray-700 hover:bg-gray-50 transition-colors flex items-center gap-2"
            role="menuitem"
          >
            <User class="h-4 w-4" />
            Edit Profile
          </button>
          <button
            type="button"
            onclick={() => handleProfileAction("logout")}
            class="w-full px-4 py-2 text-left text-sm text-red-600 hover:bg-red-50 transition-colors"
            role="menuitem"
          >
            Logout
          </button>
        </div>
      {/if}
    </div>

    <!-- Settings Button -->
    <button
      type="button"
      onclick={handleSettingsClick}
      class="p-2 rounded-lg hover:bg-gray-100 transition-colors focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
      aria-label="Settings"
    >
      <Settings class="h-5 w-5 text-gray-600" />
    </button>
  </div>
</header>

<style>
  /* Additional styles if needed */
</style>
