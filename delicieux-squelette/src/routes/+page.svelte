<script lang="ts">
  import { _, locale } from "svelte-i18n";
</script>

<div
  class="
	h-full
  	container
	mx-auto
  	md:max-w-2xl max-w-xs
	flex flex-col
	pt-12"
>
  <h1 class="text-3xl pb-8">{$_("titre")}</h1>

  <div class="flex-[2_2_0%] flex flex-col gap-4 items-center justify-center">
    <p>{$_("welcome.1")}</p>
    <p>{$_("welcome.2")}</p>
    <p>{$_("welcome.3")}</p>
    <p>{$_("welcome.4")}&gt;</p>
    <button
      onclick={() => ($locale === "fr" ? locale.set("en") : locale.set("fr"))}
    >
      lang : {$locale}
    </button>
  </div>

  <div class="flex-[1_0_0%]"></div>
</div>
