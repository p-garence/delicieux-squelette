<script lang="ts">
    import { roomAdd } from './addRoomStore';
    
    import { X, Plus } from 'lucide-svelte';
    
    interface Props {
        handleSubmit: () => void;
    }

    let { handleSubmit }: Props = $props();
</script>

<form
    onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}
    class="card max-w-lg p-4 gap-2 flex flex-col"
>
    <label class="label" for="name">
        <span>Nom de la salle</span>
        <input class="input" type="text" id="name" bind:value={$roomAdd.name} required minlength="5" />
    </label>

    <label class="label" for="rules">
        <span>Message de la salle</span>
        <textarea id="rules" class="textarea" bind:value={$roomAdd.rules} rows="4" placeholder="Enter some long form content."></textarea>
    </label>

    <label class="label p-2" for="publique">
        <input type="checkbox" id="publique" bind:checked={$roomAdd.is_public} />
        <span>Publique</span>
    </label>

    <label class="label p-2" for="password_protected">
        <input type="checkbox" id="password_protected" bind:checked={$roomAdd.password_protected} />
        <span>Protégé par mot de passe</span>
    </label>

    {#if $roomAdd.password_protected}
        <label class="label" for="password">
            <span>Mot de passe</span>
            <input class="input" type="password" id="password" bind:value={$roomAdd.password} />
        </label>
    {/if}

    <label class="label p-2" for="resolution">
        <span>Taille des cotés des dessins : {$roomAdd.resolution}px</span>
        <input type="range" id="resolution" bind:value={$roomAdd.resolution} max="100" min="5" />
    </label>

    <fieldset class="label p-2">
        <div>Couleurs du dessin</div>
        <div class="grid grid-cols-3 gap-2 mb-2">
            {#each $roomAdd.colors as color, i}
                <label for="color{i}" class="flex flex-row items-stretch">
                    
                    <!-- CORRIGÉ : On bind sur l'index du tableau, pas sur la variable locale -->
                    <input class="h-full w-full cursor-pointer" type="color" id="color{i}" bind:value={$roomAdd.colors[i]} />
                    
                    {#if i !== 0}
                        <!-- CORRIGÉ : type="button" empêche la soumission du form ! -->
                        <button
                            type="button"
                            class="btn btn-icon preset-filled rounded-none"
                            onclick={() =>
                                roomAdd.update((elem) => {
                                    elem.colors = elem.colors.filter((_, index) => index !== i);
                                    return elem;
                                })
                            }
                        >
                            <X class="w-4 h-4"/>
                        </button>
                    {/if}
                </label>
            {/each}
            
            {#if $roomAdd.colors.length < 6}
                <button
                    type="button"
                    class="btn btn-icon preset-filled rounded-none"
                    onclick={() =>
                        roomAdd.update((elem) => {
                            elem.colors.push('#000000');
                            return elem;
                        })
                    }
                >
                    <Plus class="w-5 h-5"/>
                </button>
            {/if}
        </div>
    </fieldset>

    <label class="label" for="admin_password">
        <span>Mot de passe admin</span>
        <input class="input" type="password" id="admin_password" bind:value={$roomAdd.admin_password} required minlength="5"/>
    </label>

    <div>
        <button class="btn preset-filled p-4 m-4">Créer la salle</button>
    </div>
</form>