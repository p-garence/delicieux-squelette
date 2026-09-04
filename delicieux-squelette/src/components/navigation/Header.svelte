<script lang="ts">
  import Burger from "./Burger.svelte";
  import MobileNav from "./MobileNav.svelte";
  import { navigationValues } from "./navigation";

  let drawerOpen = $state(false);
</script>

<nav class="base w-full sm:h-20 h-12">
  <div class="flex items-center md:justify-center h-full w-full text-2xl p-4">
    <Burger bind:isOpen={drawerOpen} />
    <MobileNav bind:isOpen={drawerOpen}/>
    <div class="hidden md:flex flex-row items-center gap-10">
      <ul class="gap-10 items-center justify-center flex">
        {#each $navigationValues as item}
          <li>
            <a href={item.route}>
              {item.name}
            </a>
          </li>
        {/each}
      </ul>
    </div>
  </div>
  <div class="shadow-blend h-12"></div>
</nav>
