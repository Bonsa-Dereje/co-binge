import { writable } from 'svelte/store';

export const globalTime = writable<string | null>(null);

let interval: ReturnType<typeof setInterval> | null = null;

export function startGlobalTimeSync() {
    if (interval) return;

    const fetchTime = async () => {
        try {
            const res = await fetch('https://time.now/developer/api/ip');
            const data = await res.json();

            // using UTC for consistency across devices
            globalTime.set(data.utc_datetime);
        } catch (err) {
            console.error("Time fetch failed:", err);
        }
    };

    fetchTime(); // initial call
    interval = setInterval(fetchTime, 3000);
}

export function stopGlobalTimeSync() {
    if (interval) {
        clearInterval(interval);
        interval = null;
    }
}