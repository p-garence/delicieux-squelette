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

<button type="button" class="btn preset-filled p-4 btn-lg" onclick={showModal}>
    Rejoindre
</button>

<dialog 
    bind:this={dialogRef} 
    class="dialog preset-filled-surface-100-900 animate-dialog border border-surface-200-800 shadow-xl rounded-container p-6 backdrop:bg-black/50"
>
    <JoinModal {closeModal} />
</dialog>