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
  import { audioStore } from "$lib/stores/audioStore";

  // Get devices from store
  let microphones = $state<Array<{ id: string; name: string }>>([]);
  let selectedMicrophone = $state("");
  let isTestingMicrophone = $state(false);
  let microphoneLevel = $state([45]); // Mock audio level
  let testInterval = $state<ReturnType<typeof setInterval> | null>(null);
  let isLoading = $state(false);
  let error = $state<string | null>(null);

  // Fetch devices on mount
  $effect(() => {
    let mounted = true;

    async function loadDevices() {
      isLoading = true;
      error = null;
      try {
        await audioStore.refreshDevices();
        // Also get current device selection
        await audioStore.getCurrentDevices();
      } catch (err) {
        if (mounted) {
          error = err instanceof Error ? err.message : "Failed to load audio devices";
          console.error("DEBUG:[SETTINGS/AUDIO] Error loading devices:", err);
        }
      } finally {
        if (mounted) {
          isLoading = false;
        }
      }
    }

    loadDevices();

    return () => {
      mounted = false;
    };
  });

  $effect(() => {
    const unsubscribeDevices = audioStore.inputDevices.subscribe((devices) => {
      if (devices && devices.length > 0) {
        microphones = devices.map((d) => ({ id: d.id, name: d.name }));
        if (!selectedMicrophone) {
          selectedMicrophone = microphones[0].id;
        }
      } else {
        microphones = [];
      }
    });
    const unsubscribeSelected = audioStore.selectedInputDevice.subscribe(
      (device) => {
        if (device) {
          selectedMicrophone = device.id;
        }
      }
    );
    const unsubscribeLoading = audioStore.isLoadingDevices.subscribe((loading: boolean) => {
      isLoading = loading;
    });
    return () => {
      unsubscribeDevices();
      unsubscribeSelected();
      unsubscribeLoading();
    };
  });

  // Cleanup interval when component unmounts or when testing stops
  $effect(() => {
    if (!isTestingMicrophone && testInterval) {
      clearInterval(testInterval);
      testInterval = null;
    }
    return () => {
      if (testInterval) {
        clearInterval(testInterval);
        testInterval = null;
      }
    };
  });

  function handleTestMicrophone() {
    console.log("DEBUG:[SETTINGS/AUDIO] Test microphone clicked");
    isTestingMicrophone = !isTestingMicrophone;
    if (isTestingMicrophone) {
      // Clear any existing interval
      if (testInterval) {
        clearInterval(testInterval);
      }
      // Mock: simulate audio level changes
      testInterval = setInterval(() => {
        microphoneLevel = [Math.floor(Math.random() * 100)];
      }, 100);
    } else {
      // Stop testing - cleanup handled by $effect
      if (testInterval) {
        clearInterval(testInterval);
        testInterval = null;
      }
    }
  }
</script>

<div class="space-y-3">
  <div class="flex items-center gap-2">
    <Mic class="h-5 w-5 text-gray-600" />
    <Label class="text-base font-semibold">Microphone</Label>
  </div>
  {#if error}
    <div class="text-sm text-red-600 mb-2">{error}</div>
  {/if}
  <Select
    type="single"
    value={selectedMicrophone}
    disabled={isLoading}
    onValueChange={async (value) => {
      if (value) {
        try {
          error = null;
          await audioStore.setInputDevice(value);
        } catch (err) {
          error = err instanceof Error ? err.message : "Failed to set input device";
          console.error("DEBUG:[SETTINGS/AUDIO] Error setting input device:", err);
        }
      }
    }}
  >
    <SelectTrigger class="w-full" disabled={isLoading}>
      {isLoading
        ? "Loading devices..."
        : microphones.find((m) => m.id === selectedMicrophone)?.name ||
          "Select microphone"}
    </SelectTrigger>
    <SelectContent>
      {#if microphones.length === 0 && !isLoading}
        <div class="px-2 py-1.5 text-sm text-gray-500">No microphones found</div>
      {:else}
        {#each microphones as microphone}
          <SelectItem value={microphone.id} label={microphone.name} />
        {/each}
      {/if}
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
