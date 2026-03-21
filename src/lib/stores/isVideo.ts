import { writable } from 'svelte/store';

// Boolean store: true if the latest dragged file is a video
export const isVideoStore = writable<boolean>(false);

// Optional helper function
export function setIsVideo(value: boolean) {
    isVideoStore.set(value);
}