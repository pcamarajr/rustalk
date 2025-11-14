<script lang="ts">
  import { Mic, Play, Square } from "lucide-svelte";
  import { Button } from "$lib/components/ui/button";
  import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
  } from "$lib/components/ui/select";
  import { Label } from "$lib/components/ui/label";
  import { Separator } from "$lib/components/ui/separator";

  // Self-contained state
  let microphones = $state([
    { id: "mic1", name: "Built-in Microphone" },
    { id: "mic2", name: "USB Microphone" },
    { id: "mic3", name: "Bluetooth Headset" },
  ]);
  let selectedMicrophone = $state("mic1");
  let isTestingMicrophone = $state(false);
  let microphoneLevel = $state([45]); // Mock audio level

  function handleTestMicrophone() {
    console.log("DEBUG:[SETTINGS/AUDIO] Test microphone clicked");
    isTestingMicrophone = !isTestingMicrophone;
    if (isTestingMicrophone) {
      // Mock: simulate audio level changes
      const interval = setInterval(() => {
        if (!isTestingMicrophone) {
          clearInterval(interval);
          return;
        }
        microphoneLevel = [Math.floor(Math.random() * 100)];
      }, 100);
    }
  }
</script>

<div class="space-y-3">
  <div class="flex items-center gap-2">
    <Mic class="h-5 w-5 text-gray-600" />
    <Label class="text-base font-semibold">Microphone</Label>
  </div>
  <Select type="single">
    <SelectTrigger class="w-full">
      {microphones.find((m) => m.id === selectedMicrophone)?.name ||
        "Select microphone"}
    </SelectTrigger>
    <SelectContent>
      {#each microphones as microphone}
        <SelectItem value={microphone.id} label={microphone.name} />
      {/each}
    </SelectContent>
  </Select>
  <div class="flex items-center gap-3">
    <Button
      variant={isTestingMicrophone ? "destructive" : "outline"}
      size="sm"
      onclick={handleTestMicrophone}
      class="shrink-0"
    >
      {#if isTestingMicrophone}
        <Square class="h-4 w-4 mr-2" />
        Stop Test
      {:else}
        <Play class="h-4 w-4 mr-2" />
        Test
      {/if}
    </Button>
    {#if isTestingMicrophone}
      <div class="flex-1 flex items-center gap-2">
        <span class="text-xs text-gray-500 w-12">Level:</span>
        <div class="flex-1 flex items-center gap-1">
          {#each Array(10) as _, i}
            <div
              class="h-4 w-2 rounded-sm {microphoneLevel[0] > i * 10
                ? 'bg-blue-500'
                : 'bg-gray-200'}"
            ></div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</div>

