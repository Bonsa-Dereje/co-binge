<script lang="ts">
import { goto } from '$app/navigation';
import { onMount } from 'svelte';
import { browser } from '$app/environment';
import { supabase } from '$lib/supabase';
import { writable } from 'svelte/store';

// --- Persistent global username store ---
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

// Global username and device ID stores
export const globalUserName = createPersistentStore('username', '');
export const globalDeviceId = createPersistentStore('deviceId', '');

let userName = '';
let deviceId = 'Loading...';
let darkMode = false;
let leaving = false;
let errorMessage = '';

/** Sanitize username: allow letters, numbers, _ and -, lowercase */
function sanitizeUsername(name: string) {
    return name.trim().replace(/[^a-zA-Z0-9_-]/g, '').toLowerCase();
}

/** Create user in Supabase */
async function createUserInSupabase(username: string, deviceId: string) {
    const sanitized = sanitizeUsername(username);

    const { data, error } = await supabase.from('users').insert([{
        userName: sanitized,
        device_id: deviceId,   // store as string (varchar)
        last_login: new Date().toISOString(),
        is_active: true
    }]);

    if (error) {
        console.error('Supabase insert error:', error);
        return { success: false, error };
    }

    console.log('Supabase insert success:', data);
    return { success: true, data };
}

/** Handle account creation */
async function handleCreateAccount() {
    if (!userName.trim()) {
        errorMessage = 'Please enter a username.';
        return;
    }

    errorMessage = '';
    console.log('Creating account for:', userName);

    const result = await createUserInSupabase(userName, deviceId);

    if (!result.success) {
        errorMessage = 'Failed to create user. Try a different username.';
        return;
    }

    // Save globally (and persist)
    globalUserName.set(userName);
    globalDeviceId.set(deviceId);

    leaving = true;

    setTimeout(() => {
        goto('/hostJoin');
    }, 600);
}

/** Toggle dark mode */
function toggleDarkMode() {
    darkMode = !darkMode;
    document.body.classList.toggle('dark-mode', darkMode);
}

/** Fetch device ID from Tauri */
onMount(async () => {
    console.log('onMount triggered');

    if (!browser) {
        console.log('Not in browser, skipping Tauri call');
        return;
    }

    try {
        const tauri = await import('@tauri-apps/api/core');
        const id: string = await tauri.invoke('get_device_id');
        console.log('Device ID received:', id);
        deviceId = id;

        // Save globally
        globalDeviceId.set(id);
    } catch (err) {
        console.error('Error fetching device ID:', err);
        deviceId = 'Error getting ID';
    }

    // Pre-fill username if already stored
    globalUserName.subscribe(value => {
        if (value) userName = value;
    });
});
</script>

<div class="page-wrapper" class:leaving={leaving}>
    <div class="card">

        <div class="toggle-wrapper">
            <button 
                class="toggle"
                class:active={darkMode}
                on:click={toggleDarkMode}
                aria-label="Toggle dark mode"
            >
                <div class="toggle-circle"></div>
            </button>
        </div>

        <div class="avatar-wrapper">
            <img src="/avatars/girlAvatar.png" alt="Avatar" class="avatar" />
        </div>

        <div class="username-wrapper">
            <input 
                type="text"
                bind:value={userName}
                placeholder="User name"
                class="username-input"
            />
        </div>

        <div class="device-wrapper">
            <p class="device-id">Device ID: {deviceId}</p>
        </div>

        {#if errorMessage}
            <p class="error-message">{errorMessage}</p>
        {/if}

        <div class="create-wrapper">
            <button 
                on:click={handleCreateAccount}
                class="create-button"
            >
                Create Account
            </button>
        </div>

    </div>
</div>

<style src="../style.css"></style>