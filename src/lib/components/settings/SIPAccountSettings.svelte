<script lang="ts">
  import { Server, SquarePen } from "lucide-svelte";
  import { Button } from "$lib/components/ui/button";
  import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
  } from "$lib/components/ui/card";
  import { Label } from "$lib/components/ui/label";

  // Self-contained state
  let sipServer = $state("sip.example.com");
  let sipPort = $state("5060");
  let sipProtocol = $state<"UDP" | "TCP" | "TLS">("UDP");

  function handleEditConnection() {
    console.log("DEBUG:[SETTINGS/SIP] Edit connection clicked");
    // TODO: Open SIP connection edit dialog
  }
</script>

<Card>
  <CardHeader>
    <CardTitle>SIP Account</CardTitle>
    <CardDescription>Your SIP connection settings</CardDescription>
  </CardHeader>
  <CardContent class="flex flex-col gap-2">
    <div class="flex flex-col gap-2">
      <div class="flex items-center gap-2">
        <Server class="h-5 w-5 text-gray-600" />
        <Label class="text-base font-semibold">Connection Details</Label>
      </div>
      <div class="flex flex-col gap-2 pl-7">
        <div
          class="flex items-center justify-between py-2 border-b border-gray-100"
        >
          <span class="text-sm text-gray-600">Server:</span>
          <span class="text-sm font-medium text-gray-900">{sipServer}</span>
        </div>
        <div
          class="flex items-center justify-between py-2 border-b border-gray-100"
        >
          <span class="text-sm text-gray-600">Port:</span>
          <span class="text-sm font-medium text-gray-900">{sipPort}</span>
        </div>
        <div class="flex items-center justify-between py-2">
          <span class="text-sm text-gray-600">Protocol:</span>
          <span class="text-sm font-medium text-gray-900">{sipProtocol}</span>
        </div>
      </div>
    </div>

    <Button
      variant="outline"
      onclick={handleEditConnection}
      class="w-full sm:w-auto"
    >
      <SquarePen class="h-4 w-4 mr-2" />
      Edit Connection
    </Button>
  </CardContent>
</Card>
