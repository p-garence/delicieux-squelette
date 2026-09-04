<script lang="ts">
  import "../app.css";
  import "./layout.css";

  import Header from "../components/navigation/Header.svelte";
  import { isLoading } from "svelte-i18n";

  let { children } = $props();
</script>

{#if $isLoading}
  <div class="flex h-screen items-center justify-center text-white">
    <p class="text-2xl animate-pulse">Invocation en cours...</p>
  </div>
{:else}
  <div
    class="flex h-screen flex-col overflow-hidden text-center text-white text-border"
  >
    <header class="flex-none">
      <Header />
    </header>

    <main class="flex-1 overflow-y-auto bgg">
      {@render children?.()}
    </main>
  </div>
{/if}
