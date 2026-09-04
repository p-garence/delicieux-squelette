<script lang="ts">
  import ViewChoose from "../../components/voir/ViewChoose.svelte";

  import { allRooms } from "../../components/appState";
  import type { Room } from "../../components/types";
  import ServerError from "../../components/ServerError.svelte";
  import { invalidateAll } from "$app/navigation";

  interface Props {
    /** @type {import('./$types').PageData} */
    data: { allRooms: Room[]; error?: Error };
  }

  let isRetrying = $state(false);
  let { data }: Props = $props();

  $effect(() => {
    if (data.allRooms) {
      allRooms.set(data.allRooms);
    }
  });

  const reload = async () => {
    console.log("Relance de la requête serveur...");
    isRetrying = true;
    await invalidateAll(); 
    isRetrying = false
  };
</script>

{#if data.error}
  <ServerError error={data.error} reload={reload} disabled={isRetrying}/>
{/if}
<div class="bg-primary w-full h-full pt-12">
  <ViewChoose />
</div>
