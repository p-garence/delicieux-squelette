
import { addMessages, getLocaleFromNavigator, init } from 'svelte-i18n';

import en from './locales/en.json'
import fr from './locales/fr.json'
import { browser } from '$app/environment';

export function initLang() {
    addMessages('en', en);
    addMessages('fr', fr);
    init({
        fallbackLocale: "en",
        initialLocale: browser && window.navigator.language.includes("fr") ? "fr" : "en", //getLocaleFromNavigator(),
    });
}
