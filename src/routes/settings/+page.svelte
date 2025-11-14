<script lang="ts">
  import AccountSettings from "$lib/components/settings/AccountSettings.svelte";
  import AudioSettings from "$lib/components/settings/audio/AudioSettings.svelte";
  import SIPAccountSettings from "$lib/components/settings/SIPAccountSettings.svelte";
  import GeneralSettings from "$lib/components/settings/GeneralSettings.svelte";
  import AboutSettings from "$lib/components/settings/AboutSettings.svelte";
</script>

<div class="flex flex-col gap-6 p-6 max-w-2xl">
  <div>
    <h1 class="text-2xl font-bold text-gray-900">Settings</h1>
    <p class="text-sm text-gray-500 mt-1">
      Manage your account and application preferences
    </p>
  </div>

  <AccountSettings />
  <AudioSettings />
  <SIPAccountSettings />
  <GeneralSettings />
  <AboutSettings />
</div>
