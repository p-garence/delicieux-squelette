<script lang="ts">
  import { goto } from "$app/navigation";
  import LanguageSwitch from "./LanguageSwitch.svelte";
  import { navigationValues } from "./navigation";
  
  let { isOpen = $bindable(false) } = $props();

  function click(route: string) {
    goto(route);
    isOpen = false; // Close drawer after navigating
  }
</script>

{#if isOpen}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div 
    class="fixed inset-0 z-40 bg-black/50 transition-opacity"
    onclick={() => isOpen = false}
  ></div>
{/if}

<div 
  class="fixed top-0 left-0 h-full w-64 md:w-80 bg-surface-900 z-50 transform transition-transform duration-300 ease-in-out base {isOpen ? 'translate-x-0' : '-translate-x-full'}"
>
  <div class="flex h-20 items-center justify-center filter drop-shadow-md">
    <!-- potential name -->
  </div>
  
  <div class="flex flex-col items-center">
    {#each $navigationValues as item}
      <button
        class="w-full py-4 text-2xl text-white hover:bg-surface-800 transition-colors"
        onclick={() => click(item.route)}
      >
        {item.name}
      </button>
    {/each}
  </div>
</div>