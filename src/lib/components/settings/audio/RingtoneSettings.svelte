<script lang="ts">
  import { Bell, Play, Square } from "lucide-svelte";
  import { Button } from "$lib/components/ui/button";
  import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
  } from "$lib/components/ui/select";
  import { Label } from "$lib/components/ui/label";
  import { audioStore } from "$lib/stores/audioStore";

  // Get ringtones from store
  let ringtones = $state<Array<{ id: string; name: string }>>([]);
  let selectedRingtone = $state("");
  let ringtoneVolume = $state(80);
  let isPlayingRingtone = $state(false);

  $effect(() => {
    const unsubscribeRingtones = audioStore.ringtones.subscribe(
      (ringtoneList) => {
        if (ringtoneList && ringtoneList.length > 0) {
          ringtones = ringtoneList.map((r) => ({ id: r.id, name: r.name }));
          if (!selectedRingtone) {
            selectedRingtone = ringtones[0].id;
          }
        }
      }
    );
    const unsubscribeSelected = audioStore.selectedRingtone.subscribe(
      (ringtone) => {
        if (ringtone) {
          selectedRingtone = ringtone.id;
        }
      }
    );
    const unsubscribeVolume = audioStore.ringtoneVolume.subscribe((volume) => {
      ringtoneVolume = volume;
    });
    return () => {
      unsubscribeRingtones();
      unsubscribeSelected();
      unsubscribeVolume();
    };
  });

  function handlePlayRingtone() {
    console.log("DEBUG:[SETTINGS/AUDIO] Play ringtone clicked");
    isPlayingRingtone = !isPlayingRingtone;
    // TODO: Play/stop ringtone
  }
</script>

<div class="space-y-3">
  <div class="flex items-center gap-2">
    <Bell class="h-5 w-5 text-gray-600" />
    <Label class="text-base font-semibold">Ringtone</Label>
  </div>
  <Select
    type="single"
    value={selectedRingtone}
    onValueChange={(value) => {
      if (value) {
        audioStore.setRingtone(value);
      }
    }}
  >
    <SelectTrigger class="w-full">
      {ringtones.find((r) => r.id === selectedRingtone)?.name ||
        "Select ringtone"}
    </SelectTrigger>
    <SelectContent>
      {#each ringtones as ringtone}
        <SelectItem value={ringtone.id} label={ringtone.name} />
      {/each}
    </SelectContent>
  </Select>
  <div class="space-y-3">
    <div class="flex items-center gap-3">
      <Button
        variant={isPlayingRingtone ? "destructive" : "outline"}
        size="sm"
        onclick={handlePlayRingtone}
        class="shrink-0"
      >
        {#if isPlayingRingtone}
          <Square class="h-4 w-4 mr-2" />
          Stop
        {:else}
          <Play class="h-4 w-4 mr-2" />
          Play
        {/if}
      </Button>
      <div class="flex-1 flex items-center gap-3">
        <span class="text-xs text-gray-500 w-16">Volume:</span>
        <div class="flex-1 relative flex items-center">
          <input
            type="range"
            value={ringtoneVolume}
            min={0}
            max={100}
            step={1}
            oninput={(e) => {
              const volume = Number((e.target as HTMLInputElement).value);
              audioStore.setRingtoneVolume(volume);
            }}
            class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer slider"
          />
        </div>
        <span class="text-xs text-gray-700 w-12 text-right"
          >{ringtoneVolume}%</span
        >
      </div>
    </div>
  </div>
</div>
