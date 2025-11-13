<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<main
  class="m-0 pt-[10vh] flex flex-col justify-center text-center min-h-screen bg-gray-50"
>
  <h1 class="text-2xl font-semibold text-gray-900 mb-4">
    Welcome to Tauri + Svelte
  </h1>

  <div class="flex justify-center gap-6 mb-4">
    <a
      href="https://vite.dev"
      target="_blank"
      class="font-medium text-primary hover:text-primary-hover transition-colors"
    >
      <img
        src="/vite.svg"
        class="h-24 p-6 will-change-[filter] transition-[filter] duration-300 hover:drop-shadow-[0_0_2em_#747bff]"
        alt="Vite Logo"
      />
    </a>
    <a
      href="https://tauri.app"
      target="_blank"
      class="font-medium text-primary hover:text-primary-hover transition-colors"
    >
      <img
        src="/tauri.svg"
        class="h-24 p-6 will-change-[filter] transition-[filter] duration-300 hover:drop-shadow-[0_0_2em_#24c8db]"
        alt="Tauri Logo"
      />
    </a>
    <a
      href="https://svelte.dev"
      target="_blank"
      class="font-medium text-primary hover:text-primary-hover transition-colors"
    >
      <img
        src="/svelte.svg"
        class="h-24 p-6 will-change-[filter] transition-[filter] duration-300 hover:drop-shadow-[0_0_2em_#ff3e00]"
        alt="SvelteKit Logo"
      />
    </a>
  </div>
  <p class="text-gray-600 mb-6">
    Click on the Tauri, Vite, and SvelteKit logos to learn more.
  </p>

  <form class="flex justify-center gap-2 mb-4" onsubmit={greet}>
    <input
      id="greet-input"
      type="text"
      placeholder="Enter a name..."
      bind:value={name}
      class="rounded-lg border border-gray-200 px-4 py-3 text-base font-medium text-gray-900 bg-white shadow-sm transition-colors focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent"
    />
    <button
      type="submit"
      class="rounded-lg border border-transparent px-4 py-3 text-base font-medium text-white bg-primary hover:bg-primary-hover active:bg-primary-dark cursor-pointer transition-colors shadow-sm focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
    >
      Greet
    </button>
  </form>
  {#if greetMsg}
    <p class="text-gray-700 font-medium">{greetMsg}</p>
  {/if}
</main>
