<script>
    import { goto } from '$app/navigation';
    import { onMount } from 'svelte';
    import { browser } from '$app/environment';

    let userName = '';
    let deviceId = 'Loading...';
    let darkMode = false;
    let leaving = false;

    onMount(async () => {
        console.log('onMount triggered');

        if (!browser) {
            console.log('Not in browser, skipping Tauri call');
            return;
        }

        try {
            console.log('About to import Tauri API...');
            const tauri = await import('@tauri-apps/api/core');
            console.log('Tauri API imported:', tauri);

            console.log('Invoking get_device_id...');
            const id = await tauri.invoke('get_device_id');

            console.log('SUCCESS - Device ID received:', id);

            deviceId = id;
        } catch (err) {
            console.error('ERROR during device ID fetch:', err);

            // Print deeper error details
 

            deviceId = 'Error getting ID';
        }
    });

    function handleCreateAccount() {
        console.log('Creating account for:', userName);

        leaving = true;

        setTimeout(() => {
            goto('/hostJoin');
        }, 600);
    }

    function toggleDarkMode() {
        darkMode = !darkMode;
        document.body.classList.toggle('dark-mode', darkMode);
    }
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
                placeholder="user name"
                class="username-input"
            />
        </div>

        <div class="device-wrapper">
            <p class="device-id">device id: {deviceId}</p>
        </div>

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