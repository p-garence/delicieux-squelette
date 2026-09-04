<script lang="ts">
	import { toolSize } from './tools';
</script>

<label class="label w-36 text-center px-2" for="slide">
	<span>Epaisseur : {$toolSize}</span>
	<input
		id="slide"
		type="range"
		min="1"
		max="4"
		bind:value={$toolSize}
		step="1"
		class=" input w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700"
	/>
</label>
