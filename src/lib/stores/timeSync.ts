import { writable } from "svelte/store";

export type TimeSync = {
    rawTime: string;
    plusFiveTime: string;
    isoTime: string;
};

function createTimeSyncStore() {
    const stored =
        typeof localStorage !== "undefined"
            ? localStorage.getItem("timeSync")
            : null;

    const initial: TimeSync = stored
        ? JSON.parse(stored)
        : {
              rawTime: "",
              plusFiveTime: "",
              isoTime: ""
          };

    const { subscribe, set, update } = writable<TimeSync>(initial);

    function persist(value: TimeSync) {
        if (typeof localStorage !== "undefined") {
            localStorage.setItem("timeSync", JSON.stringify(value));
        }
    }

    return {
        subscribe,

        setTime(date: Date) {
            const raw = date;

            const hh = String(raw.getHours()).padStart(2, "0");
            const mm = String(raw.getMinutes()).padStart(2, "0");
            const ss = String(raw.getSeconds()).padStart(2, "0");

            const rawTime = `${hh}:${mm}:${ss}`;

            const plusFive = new Date(raw.getTime() + 5000);

            const hh2 = String(plusFive.getHours()).padStart(2, "0");
            const mm2 = String(plusFive.getMinutes()).padStart(2, "0");
            const ss2 = String(plusFive.getSeconds()).padStart(2, "0");

            const plusFiveTime = `${hh2}:${mm2}:${ss2}`;

            const value: TimeSync = {
                rawTime,
                plusFiveTime,
                isoTime: plusFive.toISOString()
            };

            set(value);
            persist(value);
        },

        clear() {
            const empty: TimeSync = {
                rawTime: "",
                plusFiveTime: "",
                isoTime: ""
            };

            set(empty);
            persist(empty);
        }
    };
}

export const timeSyncStore = createTimeSyncStore();