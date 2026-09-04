<script lang="ts">
	import { allRooms } from '../appState';
	import { selectedIndex } from './selectedIndexStore';
 
	let setSeleted = (i: number) => {
		selectedIndex.update((val) => {
			if (val === i) {
				return -1;
			} else {
				return i;
			}
		});
	};
</script>

<div class="card p-4 rounded-2xl w-full max-w-xl bg-surface-50-900-token">
	<div
		class="flex flex-row gap-2 p-4 bg-surface-100-800-token text-black rounded-t-2xl text-center"
	>
		<div class="w-[60%] truncate">nom</div>
		<div class="w-[20%] truncate">couleurs</div>
		<div class="w-[20%] truncate">resolution</div>
	</div>
	<div class="h-0.5"></div>

	<div class="flex flex-col items-stretch max-h-[60vh] overflow-scroll">
		{#each $allRooms as room, i}
			<button
				onclick={() => setSeleted(i)}
				class="flex flex-row gap-2 p-4 {$selectedIndex === i
					? 'bg-surface-300-600-token'
					: 'bg-surface-100-800-token hover:text-white '}  text-black
				 {i === $allRooms.length - 1 ? ' rounded-b-2xl' : ''}"
			>
				<div class="w-[60%] truncate">{room.name}</div>
				<div class="w-[20%] truncate">
					<div class="grid grid-cols-3 gap-1">
						{#each room.colors as color, i}
							<div class=" w-5 h-2.5" style="background-color : {color};"></div>
						{/each}
					</div>
				</div>
				<div class="w-[20%] truncate">{room.resolution}x{room.resolution}</div>
			</button>
		{/each}
	</div>
</div>
