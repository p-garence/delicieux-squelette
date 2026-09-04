<script lang="ts">
	let { isOpen = $bindable(false) } = $props();
</script>

<div>
	<button
        class="z-50 flex relative sm:w-8 sm:h-8 h-6 w-6 flex-col justify-between items-center {isOpen ? '' : 'md:hidden'}"
        onclick={() => isOpen = !isOpen}
    >
        <span class="sr-only">Open main menu</span>
        <span class="h-1 w-full bg-white rounded-lg transform transition duration-300 ease-in-out {isOpen ? 'rotate-45 sm:translate-y-3.5 translate-y-2.5' : ''}"></span>
        <span class="h-1 w-full bg-white rounded-lg transition-all duration-300 ease-in-out {isOpen ? 'scale-0' : ''}"></span>
        <span class="h-1 w-full bg-white rounded-lg transform transition duration-300 ease-in-out {isOpen ? '-rotate-45 sm:-translate-y-3.5 -translate-y-2.5' : ''}"></span>
    </button>
</div>
