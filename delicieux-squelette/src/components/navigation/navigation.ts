import { _, locale } from 'svelte-i18n'

import { unwrapFunctionStore } from 'svelte-i18n';
import { derived } from 'svelte/store';

const $_ = unwrapFunctionStore(_);

export const navigationValues = derived(locale, (_) => {
    return [
        {
            name: $_('accueil'),
            route: "/"
        },
        {
            name: $_('parcourir'),
            route: "/voir"
        },
        {
            name: $_('draw'),
            route: "/dessiner"
        },
    ]
}) 