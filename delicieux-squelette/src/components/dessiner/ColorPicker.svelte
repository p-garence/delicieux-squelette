<script lang="ts">
  import { selectedRoom } from "../appState";
  import { toolColor } from "./tools";

  let colors = $selectedRoom?.colors
    ? $selectedRoom?.colors //.concat(["#33ff35", "#173bac", "#d936a9"])
    : null;

  let selected_color = $state(colors ? colors[0] : null);
  toolColor.update(() => selected_color ?? "");

  toolColor.subscribe((value) => {
    selected_color = value;
  });
</script>

<div class="w-16 flex items-center justify-center flex-col p-0.5">
  <span class="text-sm">{selected_color}</span>
  <div class="grid grid-cols-2 gap-0.5">
    {#if colors}
      {#each colors as color, i}
        <button
          title="color"
          class=" w-7.25 h-7.25 {selected_color === color
            ? 'border-green-600 border-4'
            : 'border-black border-2'}"
          style="background-color : {color};"
          onclick={() => {
            toolColor.update(() => color);
          }}
        ></button>
      {/each}
    {:else}
      No colors
    {/if}
  </div>
</div>
