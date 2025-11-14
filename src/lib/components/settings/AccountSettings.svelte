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

  // Self-contained state
  let userName = $state("John Doe");
  let userEmail = $state("john.doe@example.com");
  let connectionStatus = $state<"connected" | "disconnected">("connected");

  function handleEditProfile() {
    console.log("DEBUG:[SETTINGS/ACCOUNT] Edit profile clicked");
    // TODO: Open edit profile dialog
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
            : 'bg-gray-400'}"
        ></span>
        <span class="text-sm font-medium text-gray-900">
          {connectionStatus === "connected" ? "Connected" : "Disconnected"}
        </span>
      </div>
    </div>

    <Button
      variant="outline"
      onclick={handleEditProfile}
      class="w-full sm:w-auto"
    >
      <SquarePen class="h-4 w-4 mr-2" />
      Edit Profile
    </Button>
  </CardContent>
</Card>
