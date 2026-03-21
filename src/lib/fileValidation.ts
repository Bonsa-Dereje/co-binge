import { writable } from 'svelte/store';
import { browser } from '$app/environment';

/** Persistent boolean store */
function createPersistentBool(key: string, initial: boolean) {
    const store = writable(initial);

    if (browser) {
        const stored = localStorage.getItem(key);
        if (stored !== null) store.set(stored === "true");
    }

    store.subscribe(value => {
        if (browser) localStorage.setItem(key, String(value));
    });

    return store;
}

// ✅ GLOBAL VIDEO VALIDATION STATE
export const isVideoFileStore = createPersistentBool('isVideoFile', true);

/** Helper validation (for browser drag) */
export function validateVideo(name: string) {
    const allowed = ["mp4", "mkv", "avi", "mov", "webm"];
    const ext = name.split('.').pop()?.toLowerCase();

    const isValid = !!ext && allowed.includes(ext);

    isVideoFileStore.set(isValid);

    return isValid;
}