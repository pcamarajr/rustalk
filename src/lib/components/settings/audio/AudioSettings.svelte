<script lang="ts">
  import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
  } from "$lib/components/ui/card";
  import { Separator } from "$lib/components/ui/separator";

  import MicrophoneSettings from "./MicrophoneSettings.svelte";
  import SpeakerSettings from "./SpeakerSettings.svelte";
  import RingtoneSettings from "./RingtoneSettings.svelte";
  import AudioProcessingSettings from "./AudioProcessingSettings.svelte";
</script>

<Card>
  <CardHeader>
    <CardTitle>Audio Settings</CardTitle>
    <CardDescription
      >Configure your audio input, output, and ringtone preferences</CardDescription
    >
  </CardHeader>
  <CardContent class="flex flex-col gap-4">
    <MicrophoneSettings />
    <SpeakerSettings />
    <RingtoneSettings />
    <AudioProcessingSettings />
  </CardContent>
</Card>
