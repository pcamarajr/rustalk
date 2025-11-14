<script lang="ts">
  import { Volume2, Play, Square } from "lucide-svelte";
  import { Button } from "$lib/components/ui/button";
  import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
  } from "$lib/components/ui/select";
  import { Label } from "$lib/components/ui/label";
  import { Separator } from "$lib/components/ui/separator";
  import { audioStore } from "$lib/stores/audioStore";

  // Get devices from store
  let speakers = $state<Array<{ id: string; name: string }>>([]);
  let selectedSpeaker = $state("");
  let speakerVolume = $state([75]);
  let isTestingSpeaker = $state(false);

  $effect(() => {
    const unsubscribeDevices = audioStore.outputDevices.subscribe((devices) => {
      if (devices && devices.length > 0) {
        speakers = devices.map((d) => ({ id: d.id, name: d.name }));
        if (!selectedSpeaker) {
          selectedSpeaker = speakers[0].id;
        }
      }
    });
    const unsubscribeSelected = audioStore.selectedOutputDevice.subscribe((device) => {
      if (device) {
        selectedSpeaker = device.id;
      }
    });
    return () => {
      unsubscribeDevices();
      unsubscribeSelected();
    };
  });

  function handleTestSpeaker() {
    console.log("DEBUG:[SETTINGS/AUDIO] Test speaker clicked");
    isTestingSpeaker = !isTestingSpeaker;
    // TODO: Play test sound
  }
</script>

<div class="space-y-3">
  <div class="flex items-center gap-2">
    <Volume2 class="h-5 w-5 text-gray-600" />
    <Label class="text-base font-semibold">Speaker</Label>
  </div>
  <Select
    type="single"
    value={selectedSpeaker}
    onValueChange={(value) => {
      if (value) {
        audioStore.setOutputDevice(value);
      }
    }}
  >
    <SelectTrigger class="w-full">
      {speakers.find((s) => s.id === selectedSpeaker)?.name || "Select speaker"}
    </SelectTrigger>
    <SelectContent>
      {#each speakers as speaker}
        <SelectItem value={speaker.id} label={speaker.name} />
      {/each}
    </SelectContent>
  </Select>
  <div class="space-y-3">
    <div class="flex items-center gap-3">
      <Button
        variant={isTestingSpeaker ? "destructive" : "outline"}
        size="sm"
        onclick={handleTestSpeaker}
        class="shrink-0"
      >
        {#if isTestingSpeaker}
          <Square class="h-4 w-4 mr-2" />
          Stop Test
        {:else}
          <Play class="h-4 w-4 mr-2" />
          Test
        {/if}
      </Button>
      <div class="flex-1 flex items-center gap-3">
        <span class="text-xs text-gray-500 w-16">Volume:</span>
        <div class="flex-1 relative flex items-center">
          <input
            type="range"
            bind:value={speakerVolume[0]}
            min={0}
            max={100}
            step={1}
            oninput={(e) => {
              speakerVolume = [Number((e.target as HTMLInputElement).value)];
            }}
            class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer slider"
          />
        </div>
        <span class="text-xs text-gray-700 w-12 text-right"
          >{speakerVolume[0]}%</span
        >
      </div>
    </div>
  </div>
</div>
