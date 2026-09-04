
import { init, register } from 'svelte-i18n';

import en from '../locales/en.json'
import fr from '../locales/fr.json'
import { browser } from '$app/environment';

register('en', async () => en);
register('fr', async () => fr);

init({
    fallbackLocale: "en",
    initialLocale: browser && window.navigator.language.includes("fr") ? "fr" : "en", //getLocaleFromNavigator(),
});
