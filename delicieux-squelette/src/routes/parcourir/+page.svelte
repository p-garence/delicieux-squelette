<script lang="ts">
  import { onMount } from "svelte";
  import RoomJoin from "../../components/parcourir/RoomJoin.svelte";
  import { allRooms } from "../../components/appState";
  import type { Error, Room } from "../../components/types";
  import ServerError from "../../components/ServerError.svelte";
  import { invalidateAll } from "$app/navigation";

  interface Props {
    /** @type {import('./$types').PageData} */
    data: { allRooms: Room[]; error?: Error };
  }

  let { data }: Props = $props();
  let isRetrying = $state(false);

  $effect(() => {
    if (data.allRooms) {
      allRooms.set(data.allRooms);
    }
  });

  const reload = async () => {
    console.log("Relance de la requête serveur...");
    isRetrying = true;
    await invalidateAll();
    isRetrying = false;
  };
</script>

{#if data.error}
  <ServerError error={data.error} reload={reload} disabled={isRetrying} />
{/if}
<div class="bg-primary w-full h-full pt-12">
  <RoomJoin />
</div>
