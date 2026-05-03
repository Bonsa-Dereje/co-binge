import { writable } from 'svelte/store';

export const initialGlobalTime = writable<string | null>(null);
export const playbackStartTime = writable<string | null>(null);
export const dropTime = writable<string | null>(null);

let hasInitialized = false;

// ✅ FORMAT HH:MM:SS
function formatTime(date: Date) {
    return date.toISOString().split('T')[1].split('.')[0];
}

// ✅ INITIAL SYNC (RUN ONCE)
export function setSyncTimes(globalTimeValue: string) {
    if (!globalTimeValue || hasInitialized) return;

    hasInitialized = true;

    const baseTime = new Date(globalTimeValue);
    const syncStart = new Date(baseTime.getTime() + 5000);

    initialGlobalTime.set(globalTimeValue);
    playbackStartTime.set(formatTime(syncStart));

    console.log("✅ Sync initialized");
    console.log("Initial UTC:", globalTimeValue);
    console.log("Playback Start:", formatTime(syncStart));
}

// ✅ DROP SYNC (RUN EVERY DROP)
export function setDropTime(globalTimeValue: string) {
    if (!globalTimeValue) return;

    const baseTime = new Date(globalTimeValue);
    const syncStart = new Date(baseTime.getTime() + 5000);

    dropTime.set(globalTimeValue);

    console.log("📦 Drop Time (API):", globalTimeValue);
    console.log("📦 Drop Playback Start:", formatTime(syncStart));
}