<script lang="ts">
import { onMount } from 'svelte';
import { goto } from '$app/navigation';
import { globalUserName, globalDeviceId } from '$lib/stores/user';
import { invoke } from '@tauri-apps/api/core'; // ✅ ADD

let entering = true;
let leaving = false;
let darkMode = false;
let username = '';
let deviceId = '';

onMount(() => {
    requestAnimationFrame(() => {
        entering = false;
    });

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

// ✅ UPDATED: navigate first, write to DB in background
function handleHostClick() {
    leaving = true;

    // navigate immediately
    goto('/pairPage');

    // run DB write in background (non-blocking)
    setTimeout(async () => {
        try {
            await invoke("set_hosting_true", {
                deviceId: deviceId
            });
            console.log("Hosting set to TRUE");
        } catch (e) {
            console.error("Failed to set hosting:", e);
        }
    }, 0);
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

        <div class="session-wrapper" class:leaving={leaving}>

            <button class="session-button" on:click={handleHostClick}>
                Host
            </button>

            <div class="session-divider"></div>

            <button class="session-button" on:click={handleJoinClick}> 
                Join
            </button>

        </div>

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