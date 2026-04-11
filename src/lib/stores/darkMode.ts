import { writable } from 'svelte/store';

// Helper to detect initial theme
function getInitialTheme(): boolean {
    if (typeof localStorage !== 'undefined') {
        const stored = localStorage.getItem('darkMode');
        if (stored !== null) {
            return stored === 'true';
        }
    }

    // fallback to system preference
    if (typeof window !== 'undefined') {
        return window.matchMedia('(prefers-color-scheme: dark)').matches;
    }

    return false;
}

// Create store
function createDarkModeStore() {
    const { subscribe, set, update } = writable<boolean>(getInitialTheme());

    return {
        subscribe,

        toggle: () => update(value => {
            const newValue = !value;
            applyTheme(newValue);
            persist(newValue);
            return newValue;
        }),

        set: (value: boolean) => {
            applyTheme(value);
            persist(value);
            set(value);
        }
    };
}

// Apply theme to DOM
function applyTheme(isDark: boolean) {
    if (typeof document === 'undefined') return;

    const root = document.documentElement;

    if (isDark) {
        root.classList.add('dark');
    } else {
        root.classList.remove('dark');
    }
}

// Save to localStorage
function persist(value: boolean) {
    if (typeof localStorage !== 'undefined') {
        localStorage.setItem('darkMode', String(value));
    }
}

// Initialize immediately so refresh applies theme
const darkMode = createDarkModeStore();

// Apply on first load
if (typeof window !== 'undefined') {
    darkMode.subscribe(value => {
        applyTheme(value);
    });
}

export default darkMode;