<script lang="ts">
    import { allRooms, joinRoom } from '../appState';
    import { selectedIndex } from './selectedIndexStore';
    import { goto } from '$app/navigation';

    // 1. Accept the closeModal function from the parent
    let { closeModal }: { closeModal: () => void } = $props();

    let room = $derived($allRooms && $selectedIndex !== -1 ? $allRooms[$selectedIndex] : undefined);

    let promise: Promise<void> | undefined = $state();
    let password: string = $state("");

    const click = () => {
        if ($allRooms && $selectedIndex !== -1 && room) {
            promise = joinRoom(room).then(() => {
                closeModal(); // 2. Close native dialog on success
                goto('/voir');
            });
        }
    };
</script>

<div class="gap-4 flex flex-col w-full max-w-[500px]">
    <h2 class="h2">{room?.name}</h2>

    <p>Resolution : {room?.resolution}</p>

    <p>Colors</p>
    <div class="grid grid-cols-3 gap-2 self-start">
        {#each room?.colors ?? [] as color}
            <div class="w-[40px] h-[20px] rounded" style="background-color: {color};"></div>
        {/each}
    </div>

    {#if room?.rules}
        <p>Description de la salle :</p>
        <p class="bg-surface-200-700-token p-2 rounded whitespace-pre-line">
            {room.rules}
        </p>
    {/if}

    {#if room?.password_protected}
        <label class="label" for="password">
            <span>Mot de passe</span>
            <input class="input" type="password" id="password" bind:value={password} />
        </label>
    {/if}

    {#if promise}
        {#await promise}
            <div>Chargement...</div>
        {:then _}
            <!-- loaded -->
        {/await}
    {/if}

    <div class="flex flex-row justify-end gap-2 mt-4">
        <!-- 3. Call closeModal on cancel -->
        <button type="button" onclick={closeModal} class="btn preset-tonal w-[100px]">
            Annuler
        </button>
        <button type="button" onclick={click} class="btn preset-filled w-[100px]">
            Rejoindre
        </button>
    </div>
</div>