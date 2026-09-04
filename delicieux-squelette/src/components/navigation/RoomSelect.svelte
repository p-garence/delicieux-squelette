<script lang="ts">
  import { ChevronDown } from "lucide-svelte";
  import { Popover } from "@skeletonlabs/skeleton-svelte";
  import { usersRooms, selectedRoom, setSelectedRoom } from "../appState";

  let index = $derived($usersRooms?.findIndex((elem) => elem.id === $selectedRoom?.id));
  
  let popoverOpen = $state(false);
</script>

<!-- 1. The Root component manages the state -->
<Popover open={popoverOpen} onOpenChange={(e) => popoverOpen = e.open} positioning={{ placement: "bottom" }}>
  
  <!-- 2. The Trigger component automatically handles clicks and accessibility -->
  <Popover.Trigger class="btn preset-filled flex items-center justify-between">
    {#if $selectedRoom?.name}
      <div class="min-w-50">
        {$selectedRoom.name}
      </div>
      <ChevronDown class="w-4 h-4 ml-2" />
    {:else}
      pas de dessin selectionné
      <ChevronDown class="ml-2 w-4 h-4"/>
    {/if}
  </Popover.Trigger>

  <!-- 3. The Content component holds your dropdown UI -->
  <Popover.Content class="card p-4 w-72 flex-col flex bg-surface-100-900 border border-surface-200-800 shadow-xl rounded-container z-50">
    <Popover.Arrow />
    
    {#if $usersRooms.length !== 0}
      <div class="flex flex-col items-stretch space-y-1 mb-4">
        {#each $usersRooms as r, i}
          <label class="flex items-center space-x-2 p-2 cursor-pointer hover:bg-surface-200-800 rounded">
            <input
              type="radio"
              name="room_selection"
              value={i}
              checked={index === i}
              onchange={() => {
                setSelectedRoom(r);
                popoverOpen = false;
              }}
              class="radio"
            />
            <span class="pl-2">{r.name}</span>
          </label>
        {/each}
      </div>
    {:else}
      <a href="/parcourir" class="my-2 btn w-full preset-filled" onclick={() => popoverOpen = false}>
        parcourir les dessins publics
      </a>
    {/if}
    
    <a href="/gerer_dessins" class="mt-2 btn w-full preset-tonal" onclick={() => popoverOpen = false}>gerer les dessins</a>
    <a href="/creer_dessin" class="mt-2 btn w-full preset-tonal" onclick={() => popoverOpen = false}>créer un dessin</a>
  </Popover.Content>
</Popover>