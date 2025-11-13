<script lang="ts">
  import Dialer from "$lib/components/Dialer.svelte";
  import { Settings } from "lucide-svelte";
</script>

<main class="min-h-screen bg-gray-50">
  <!-- Header -->
  <header class="bg-white border-b border-gray-200 px-6 py-4">
    <div class="flex items-center justify-between">
      <h1 class="text-xl font-semibold text-gray-900">Rustalk</h1>
      <div class="flex items-center gap-4">
        <!-- Profile placeholder (simplified for now) -->
        <div class="text-sm text-gray-600">John Doe</div>
        <button
          type="button"
          class="p-2 rounded-lg hover:bg-gray-100 transition-colors focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
          aria-label="Settings"
        >
          <Settings class="h-5 w-5 text-gray-600" />
        </button>
      </div>
    </div>
  </header>

  <!-- Main Content -->
  <div class="container mx-auto max-w-md px-4 py-6">
    <Dialer />
  </div>
</main>
