<script lang="ts">
    import { onMount } from 'svelte';
    import { globalUserName, globalDeviceId } from '$lib/stores/user';

    let entering = true;
    let darkMode = false;
    let username = '';   // loaded from global store
    let deviceId = '';   // loaded from global store
    let groupId = "";

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

    function joinGroup() {
        console.log("Joining group:", groupId);
    }
</script>

<div class="page-wrapper join-page" class:entering={entering}>

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

        <!-- Join Input -->
        <div class="join-wrapper">

            <input
                class="group-input"
                placeholder="group name"
                bind:value={groupId}
            />

            <button class="create-group" on:click={joinGroup}>
                Create Group
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

<style src="./createGroup.css"></style>