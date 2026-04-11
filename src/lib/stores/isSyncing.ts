import { writable } from 'svelte/store';

// Boolean store: true when syncing should start
export const isSyncingStore = writable<boolean>(false);

// Optional helper
export function setIsSyncing(value: boolean) {
    isSyncingStore.set(value);
}