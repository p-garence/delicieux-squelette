<script lang="ts">
  import {
    request,
    currentDessin,
    selectedRoom,
    complete,
    cancelDessin,
  } from "../../components/appState";
  import ChooseRoom from "./ChooseRoom.svelte";
  import Draw from "./Draw.svelte";
  // @ts-ignore
  import { ArrowLeft } from 'lucide-svelte';;
  import ToolSelector from "./ToolSelector.svelte";
  import { _ } from "svelte-i18n";

  let promise: Promise<void> = $state();
  let error: string = $state("");

  const req = () => {
    promise = request().catch((e) => {
      error = e;
    });
  };

  const comp = () => {
    promise = complete().catch((e) => {
      error = e;
    });
  };
</script>

<div class="w-full h-full flex flex-col justify-center items-center">
  {#if error}
    <div
      class="w-full card bg-error-200-700-token text-error-800-100-token p-4"
    >
      {error}
    </div>
  {/if}

  {#await promise}
    load ...
  {:then}
    {#if $currentDessin}
      <Draw />
      <div class="absolute top-0 z-10 right-0">
        <button
          class="btn preset-filled p-4 m-4 btn-lg"
          onclick={cancelDessin}>{$_("cancel")}</button
        >
      </div>
      <div class="absolute bottom-0 ml-auto mr-auto">
        <button class="btn preset-filled p-4 m-4 btn-lg" onclick={comp}
          >{$_("send")}</button
        >
      </div>
      <div class="absolute left-0">
        <ToolSelector />
      </div>
    {:else}
      <div
        class="flex flex-col items-center p-4 gap-2 bg-dessin rounded-3xl shadow-fuzz no-text-border m-2"
      >
        <ChooseRoom />
        {#if $selectedRoom}
          <button
            class="btn preset-filled p-3 m-1 btn-sm "
            onclick={req}>{$_("startDrawing")}</button
          >
        {/if}
      </div>
    {/if}
  {/await}
</div>
