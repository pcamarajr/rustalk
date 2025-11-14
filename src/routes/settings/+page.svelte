<script lang="ts">
  import AccountSettings from "$lib/components/settings/AccountSettings.svelte";
  import AudioSettings from "$lib/components/settings/audio/AudioSettings.svelte";
  import SIPAccountSettings from "$lib/components/settings/SIPAccountSettings.svelte";
  import GeneralSettings from "$lib/components/settings/GeneralSettings.svelte";
  import AboutSettings from "$lib/components/settings/AboutSettings.svelte";
</script>

<div class="flex flex-col h-full">
  <!-- Header -->
  <div class="flex items-center justify-between p-6 border-b border-gray-200">
    <div>
      <h1 class="text-2xl font-bold text-gray-900">Settings</h1>
      <p class="text-sm text-gray-500 mt-1">
        Manage your account and application preferences
      </p>
    </div>
  </div>

  <!-- Main Content -->
  <div class="flex-1 overflow-y-auto p-6 space-y-6">
    <AccountSettings />
    <AudioSettings />
    <SIPAccountSettings />
    <GeneralSettings />
    <AboutSettings />
  </div>
</div>
