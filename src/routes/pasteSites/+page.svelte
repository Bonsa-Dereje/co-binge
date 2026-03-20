<script lang="ts">
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';
    import { globalUserName, globalDeviceId } from '$lib/stores/user';

    let entering = true;
    let leaving = false;
    let darkMode = false;
    let username = '';   // loaded from global store
    let deviceId = '';   // loaded from global store

    onMount(() => {
        requestAnimationFrame(() => {
            entering = false;
        });

        // Subscribe to global stores
        const unsubscribeUser = globalUserName.subscribe(value => {
            username = value || 'user name';
        });
        const unsubscribeDevice = globalDeviceId.subscribe(value => {
            deviceId = value || '01D4TH879';
        });

        // Cleanup subscriptions when component unmounts
        return () => {
            unsubscribeUser();
            unsubscribeDevice();
        };
    });

    function toggleDarkMode() {
        darkMode = !darkMode;
        document.body.classList.toggle("dark-mode", darkMode);
    }

    function handlePasteClick() {
        leaving = true;

        setTimeout(() => {
            goto('/youtubeCard');
        }, 400);
    }
</script>

<div class="page-wrapper host-join-page" class:entering={entering}>

    <div class="card">

        <!-- Toggle -->
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

        <!-- Center -->
        <div class="session-wrapper" class:leaving={leaving}>

            <button 
                class="pasteButton"
                on:click={handlePasteClick}
            >
                Paste Link
            </button>

        </div>

        <!-- Bottom Profile -->
        <div class="profile-wrapper_pastePage">

            <img 
                src="/avatars/girlAvatar.png" 
                alt="avatar" 
                class="profile-avatar_pastePage"
            />

            <div class="profile-name_pastePage">
                {username}
            </div>

            <div class="profile-device_pastePage">
                device id: {deviceId}
            </div>

        </div>

    </div>

</div>

<style src="../style.css"></style>