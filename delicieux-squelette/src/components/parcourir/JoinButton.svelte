<script lang="ts">
    import JoinModal from './JoinModal.svelte';

    let dialogRef: HTMLDialogElement;

    function showModal() {
        dialogRef?.showModal();
    }

    function closeModal() {
        dialogRef?.close();
    }
</script>

<!-- 1. The Trigger Button -->
<button type="button" class="btn preset-filled p-4 btn-lg" onclick={showModal}>
    Rejoindre
</button>

<!-- 2. The Native Dialog -->
<!-- The 'dialog' and 'animate-dialog' classes are Skeleton v3 standards -->
<dialog 
    bind:this={dialogRef} 
    class="dialog preset-filled-surface-100-900 animate-dialog border border-surface-200-800 shadow-xl rounded-container p-6 backdrop:bg-black/50"
>
    <!-- Pass the close function to your custom component -->
    <JoinModal {closeModal} />
</dialog>