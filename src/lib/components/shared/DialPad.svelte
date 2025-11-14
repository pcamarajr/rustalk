<script lang="ts">
  interface Props {
    onKeyPress?: (key: string) => void;
  }

  let { onKeyPress }: Props = $props();

  // Dial pad configuration
  const dialPadKeys = [
    { number: "1", letters: "" },
    { number: "2", letters: "ABC" },
    { number: "3", letters: "DEF" },
    { number: "4", letters: "GHI" },
    { number: "5", letters: "JKL" },
    { number: "6", letters: "MNO" },
    { number: "7", letters: "PQRS" },
    { number: "8", letters: "TUV" },
    { number: "9", letters: "WXYZ" },
    { number: "*", letters: "" },
    { number: "0", letters: "" },
    { number: "#", letters: "" },
  ];

  function handleKeyClick(key: string) {
    onKeyPress?.(key);
  }
</script>

<div class="grid grid-cols-3 gap-2" role="group" aria-label="Dial pad">
  {#each dialPadKeys as key}
    <button
      type="button"
      onclick={() => handleKeyClick(key.number)}
      class="flex flex-col items-center justify-center h-16 rounded-lg bg-white border border-gray-200 hover:bg-gray-50 active:bg-gray-100 active:scale-95 transition-all focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
      aria-label={`Dial ${key.number}${key.letters ? ` (${key.letters})` : ""}`}
    >
      <span class="text-2xl font-semibold text-gray-900">{key.number}</span>
      {#if key.letters}
        <span class="text-xs text-gray-500 mt-0.5">{key.letters}</span>
      {/if}
    </button>
  {/each}
</div>

