<script lang="ts">
  import { goto } from "$app/navigation";
  import { allRooms, setSelectedRoom } from "../appState";
</script>

<svelte:window />

<div class="w-full h-full overflow-x-scroll">
  <div
    class="p-4 grid sm:grid-flow-cols-1 md:grid-cols-2 lg:grid-cols-3 justify-evenly justify-items-center content-evenly items-center gap-4"
  >
    {#each $allRooms as room, i}
      <button
        onclick={() => {
            setSelectedRoom(room);
            goto("/canvas");
        }}
        class="shadow-fuzz w-64 h-64 p-2 bg-dessin rounded-3xl mb-4"
      >
        {room.name}
      </button>
    {/each}
  </div>
</div>
