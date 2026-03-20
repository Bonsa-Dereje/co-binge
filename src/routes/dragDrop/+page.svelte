<script lang="ts">
    import { onMount } from "svelte";
    import { globalUserName, globalDeviceId } from '$lib/stores/user';

    let entering = true;
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
            deviceId = value || '01D4TH879';
        });
    });

    function toggleDarkMode() {
        darkMode = !darkMode;
        document.body.classList.toggle("dark-mode", darkMode);
    }
</script>

<div class="page-wrapper" class:entering={entering}>

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

        <!-- Drag Drop Box -->
        <div class="drop-zone">
            <svg viewBox="0 0 24 24" class="folder-icon">
                <path d="M3 7h6l2 2h10v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"
                fill="none" stroke="currentColor" stroke-width="1.5"/>
                <path d="M3 7V5a2 2 0 0 1 2-2h4l2 2h8a2 2 0 0 1 2 2"
                fill="none" stroke="currentColor" stroke-width="1.5"/>
            </svg>

            <div class="drop-text">drag and drop</div>
        </div>

        <!-- Host Button -->
        <div class="host-wrapper">
            <button class="host-button">Host</button>
        </div>

        <!-- Profile -->
        <div class="profile-wrapperDrag">
            <img src="/avatars/girlAvatar.png" alt="avatar" class="profile-avatar"/>
            <div class="profile-nameDrag">{username}</div>
            <div class="profile-deviceDrag">device id: {deviceId}</div>
        </div>

    </div>

</div>

<style src="../style.css"></style>