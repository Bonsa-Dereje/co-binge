import { writable } from 'svelte/store';
import { browser } from '$app/environment';

/** Utility to create a persistent store using localStorage */
function createPersistentStore(key: string, initial: string) {
    const store = writable(initial);

    if (browser) {
        const stored = localStorage.getItem(key);
        if (stored) store.set(stored);
    }

    store.subscribe(value => {
        if (browser) localStorage.setItem(key, value);
    });

    return store;
}

// Global stores
export const globalUserName = createPersistentStore('username', '');
export const globalDeviceId = createPersistentStore('deviceId', '');