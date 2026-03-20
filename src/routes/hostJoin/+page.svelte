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
    globalUserName.subscribe(value => {
        username = value || 'user name';
    });
    globalDeviceId.subscribe(value => {
        deviceId = value || 'Loading...';
    });
});

function toggleDarkMode() {
    darkMode = !darkMode;
    document.body.classList.toggle("dark-mode", darkMode);
}

function handleHostClick() {
    leaving = true;
    setTimeout(() => {
        goto('/chooseApp');
    }, 400);
}

function handleJoinClick() {
    leaving = true;
    setTimeout(() => {
        goto('/pastePR_ID');
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

        <!-- Center Host / Join -->
        <div class="session-wrapper" class:leaving={leaving}>

            <button class="session-button" on:click={handleHostClick}>
                Host
            </button>

            <div class="session-divider"></div>

            <button class="session-button" on:click={handleJoinClick}> 
                Join
            </button>

        </div>

        <!-- Bottom Profile -->
        <div class="profile-wrapper">

            <img 
                src="/avatars/girlAvatar.png" 
                alt="avatar" 
                class="profile-avatar"
            />

            <div class="profile-name">
                {username}
            </div>

            <div class="profile-device">
                device id: {deviceId}
            </div>

        </div>

    </div>

</div>

<style src="../style.css"></style>