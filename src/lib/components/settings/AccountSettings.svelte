<script lang="ts">
  import { SquarePen } from "lucide-svelte";
  import { Button } from "$lib/components/ui/button";
  import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
  } from "$lib/components/ui/card";
  import { authStore } from "$lib/stores/authStore";
  import { goto } from "$app/navigation";

  // Get user info and registration state from store
  let userName = $state("John Doe");
  let userEmail = $state("john.doe@example.com");
  let connectionStatus = $state<"connected" | "disconnected">("connected");

  // Get registration state to show current status
  let registrationState = $state<
    "unregistered" | "registering" | "registered" | "failed"
  >("unregistered");

  $effect(() => {
    const unsubscribeUserInfo = authStore.userInfo.subscribe((info) => {
      userName = info.name;
      userEmail = info.email;
    });
    const unsubscribeRegistration = authStore.registrationState.subscribe(
      (state) => {
        registrationState = state;
        // Connection status is "connected" only when registered
        connectionStatus =
          state === "registered" ? "connected" : "disconnected";
      }
    );
    return () => {
      unsubscribeUserInfo();
      unsubscribeRegistration();
    };
  });

  function handleEditProfile() {
    console.log("DEBUG:[SETTINGS/ACCOUNT] Edit profile clicked");
    // TODO: Open edit profile dialog
  }

  async function handleRegister() {
    console.log("DEBUG:[SETTINGS/ACCOUNT] Register button clicked - redirecting to login");
    goto("/login");
  }

  async function handleUnregister() {
    console.log("DEBUG:[SETTINGS/ACCOUNT] Unregister button clicked");
    try {
      await authStore.unregister();
    } catch (error) {
      console.error("DEBUG:[SETTINGS/ACCOUNT] Unregister failed", error);
    }
  }
</script>

<Card>
  <CardHeader>
    <CardTitle>Account</CardTitle>
    <CardDescription
      >Your account information and connection status</CardDescription
    >
  </CardHeader>
  <CardContent class="flex flex-col gap-2">
    <div class="flex items-center gap-3">
      <div
        class="w-12 h-12 rounded-full bg-linear-to-br from-blue-400 to-blue-600 flex items-center justify-center text-white text-lg font-semibold"
      >
        {userName
          .split(" ")
          .map((n) => n[0])
          .join("")}
      </div>
      <div class="flex-1">
        <div class="font-semibold text-gray-900">{userName}</div>
        <div class="text-sm text-gray-500">{userEmail}</div>
      </div>
    </div>

    <div class="flex items-center gap-2">
      <span class="text-sm text-gray-700">Status:</span>
      <div class="flex items-center gap-2">
        <span
          class="w-2 h-2 rounded-full {connectionStatus === 'connected'
            ? 'bg-green-500'
            : registrationState === 'registering'
              ? 'bg-yellow-500 animate-pulse'
              : 'bg-gray-400'}"
        ></span>
        <span class="text-sm font-medium text-gray-900">
          {connectionStatus === "connected"
            ? "Connected"
            : registrationState === "registering"
              ? "Registering..."
              : registrationState === "failed"
                ? "Failed"
                : "Disconnected"}
        </span>
      </div>
    </div>

    <div class="flex gap-2">
      {#if registrationState === "unregistered" || registrationState === "failed"}
        <Button variant="default" onclick={handleRegister} class="flex-1">
          Register
        </Button>
      {:else if registrationState === "registering"}
        <Button variant="outline" disabled class="flex-1">
          Registering...
        </Button>
      {:else if registrationState === "registered"}
        <Button variant="destructive" onclick={handleUnregister} class="flex-1">
          Unregister
        </Button>
      {/if}
      <Button
        variant="outline"
        onclick={handleEditProfile}
        class="w-full sm:w-auto"
      >
        <SquarePen class="h-4 w-4 mr-2" />
        Edit Profile
      </Button>
    </div>
  </CardContent>
</Card>
