<script lang="ts">
  import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
  } from "$lib/components/ui/card";
  import { Switch } from "$lib/components/ui/switch";
  import { Label } from "$lib/components/ui/label";

  // Self-contained state
  let launchOnStartup = $state(false);
  let minimizeToTray = $state(true);
  let notificationsEnabled = $state(true);
</script>

<Card>
  <CardHeader>
    <CardTitle>General</CardTitle>
    <CardDescription>Application behavior and preferences</CardDescription>
  </CardHeader>
  <CardContent class="space-y-4">
    <div class="flex items-center justify-between">
      <div class="space-y-0.5">
        <Label for="launch-startup" class="text-sm font-normal"
          >Launch on System Startup</Label
        >
        <p class="text-xs text-gray-500">
          Automatically start Rustalk when you log in
        </p>
      </div>
      <Switch id="launch-startup" bind:checked={launchOnStartup} />
    </div>
    <div class="flex items-center justify-between">
      <div class="space-y-0.5">
        <Label for="minimize-tray" class="text-sm font-normal"
          >Minimize to Tray</Label
        >
        <p class="text-xs text-gray-500">
          Keep Rustalk running in the system tray
        </p>
      </div>
      <Switch id="minimize-tray" bind:checked={minimizeToTray} />
    </div>
    <div class="flex items-center justify-between">
      <div class="space-y-0.5">
        <Label for="notifications" class="text-sm font-normal"
          >Notifications Enabled</Label
        >
        <p class="text-xs text-gray-500">
          Show notifications for incoming calls and messages
        </p>
      </div>
      <Switch id="notifications" bind:checked={notificationsEnabled} />
    </div>
  </CardContent>
</Card>

