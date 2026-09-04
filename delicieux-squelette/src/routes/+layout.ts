// +layout.ts
import { browser } from '$app/environment'
import '$lib/i18n' // Import to initialize. Important :)
import { locale, waitLocale } from 'svelte-i18n'
import type { LayoutLoad } from './$types'

export const load: LayoutLoad = async () => {
	if (browser && window.navigator.language.includes("fr")) {
		locale.set("fr")
	}
	await waitLocale()
}