<script lang="ts">
  import { selectedRoom } from "../appState";

  import DrawCanvas from "./DrawCanvas.svelte";
  import SideCellsCanvas from "./SideCellsCanvas.svelte";



  let screenWidth: number = $state();
  let screenHeight: number = $state();

  let getPixelSize = $derived(() => {
    let value;
    if ($selectedRoom && screenHeight && screenWidth) {
      if (screenWidth < 450) {
        value = Math.floor((0.5 * screenWidth) / $selectedRoom?.resolution);
      } else {
        value = Math.floor((0.5 * screenHeight) / $selectedRoom?.resolution);;
      }

    } else {
      value = null;
    }
    return value;
  });
  let pixel_size = $derived(getPixelSize());
</script>

<div
  class="w-full h-full flex items-center sm:justify-center justify-end"
  bind:clientWidth={screenWidth}
  bind:clientHeight={screenHeight}
>
  {#if pixel_size && $selectedRoom}
    <SideCellsCanvas
      image_resolution={$selectedRoom.resolution}
      {pixel_size}
    >
      <DrawCanvas
        image_resolution={$selectedRoom.resolution}
        {pixel_size}
      />
    </SideCellsCanvas>

  {/if}
</div>
