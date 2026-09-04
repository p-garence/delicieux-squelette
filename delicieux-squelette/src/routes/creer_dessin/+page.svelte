<script lang="ts">
	import { roomAdd } from './addRoomStore';
	import { createRoom } from '../../components/appState';
	import CreateRoomForm from './CreateRoomForm.svelte';
	import { goto } from '$app/navigation';

	let promise: Promise<void> = $state();

	let bar = async () => {
		await createRoom($roomAdd);
		await goto('/dessiner');
	}

	const handleSubmit = () => {
		promise = bar();
	};
</script>

{#await promise}
	Loading...
{:then _}
	<div class="flex flex-col items-stretch w-full p-4">
		<CreateRoomForm {handleSubmit} />
	</div>
{:catch e}
	<div>Erreur envoi !</div>
	<div class="flex flex-col items-stretch w-full p-4">
		<CreateRoomForm {handleSubmit} />
	</div>
{/await}
