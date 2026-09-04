<script>
  import { goto } from "$app/navigation";
  import { selectedRoom } from "../appState";
  import { _ } from "svelte-i18n";
</script>

<div class=" flex flex-col items-center p-4 gap-2">
  {#if $selectedRoom}
    <h3
      class="text-center whitespace-pre-line border-b border-white pb-2 pr-4 pl-4"
    >
      {$selectedRoom.name}
    </h3>
    {#if $selectedRoom.rules}
      <p>{$selectedRoom.rules}</p>
    {/if}
    <p class="">
      {$selectedRoom.resolution}x{$selectedRoom.resolution}
    </p>
    <div class="grid grid-cols-3 gap-2">
      {#each $selectedRoom.colors as color, i}
        <div class=" w-10 h-5" style="background-color : {color};"></div>
      {/each}
    </div>
  {:else}
    <p class="text-center lg:max-w-lg max-w-xs">Pas de dessin selectionné !</p>
  {/if}
</div>
<button
  class="btn preset-filled p-3 m-1 mb-0 btn-sm"
  onclick={() => {
    goto("/parcourir");
  }}>{$_('changeDrawing')}</button
>
