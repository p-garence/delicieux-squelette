<script lang="ts">
    import { RefreshCw, TriangleAlert } from "lucide-svelte";
    import type { Error } from "./types";

    let {
        disabled,
        error,
        reload,
    }: { disabled: boolean; error: Error; reload?: () => void } = $props();

    $effect(() => {
        console.log("L'état de l'erreur a changé :", error);
    });
</script>

<div
    class="bg-error-500 text-white p-2 flex flex-wrap justify-center items-center gap-4 shadow-lg z-50"
>
    <TriangleAlert class="w-5 h-5" />
    <span>Serveur injoignable. Mode hors-ligne actif.</span>
    <button
        disabled={disabled}
        class="btn preset-filled-surface py-1 px-3 text-sm flex items-center gap-2"
        onclick={() => (reload ? reload() : window.location.reload())}
    >
        <RefreshCw class="w-4 h-4" /> Recharger
    </button>
</div>
