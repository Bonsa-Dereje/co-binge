<script lang="ts">
    import { onMount } from 'svelte';
    import { globalUserName, globalDeviceId } from '$lib/stores/user';
    import { invoke } from '@tauri-apps/api/core';

    let entering = true;
    let darkMode = false;
    let username = '';
    let deviceId = '';
    let groupId = "";

    const REQUIRED_LENGTH = 12;

    onMount(() => {
        requestAnimationFrame(() => {
            entering = false;
        });

        const unsubscribeUser = globalUserName.subscribe(value => {
            username = value || 'user name';
        });

        const unsubscribeDevice = globalDeviceId.subscribe(value => {
            deviceId = value || '01D4TH879';
        });

        checkClipboard();

        window.addEventListener('focus', checkClipboard);

        return () => {
            unsubscribeUser();
            unsubscribeDevice();
            window.removeEventListener('focus', checkClipboard);
        };
    });

    async function checkClipboard() {
        try {
            const text = (await navigator.clipboard.readText()).trim();

            if (text.length === REQUIRED_LENGTH) {
                groupId = text;
            }
        } catch (err) {
            console.warn("Clipboard access denied");
        }
    }

    function toggleDarkMode() {
        darkMode = !darkMode;
        document.body.classList.toggle("dark-mode", darkMode);
    }

    // ✅ FIXED JOIN FUNCTION (CALLS BACKEND join_pairing)
    async function joinGroup() {
        if (groupId.length !== REQUIRED_LENGTH) return;

        try {
            const result = await invoke("join_pairing", {
                deviceId: deviceId,
                clipboard: groupId
            });

            console.log("✅ Pairing success:", result);
        } catch (err) {
            console.error("❌ Join failed:", err);
        }
    }

    $: isValid = groupId.length === REQUIRED_LENGTH;
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
                placeholder="pairing id"
                bind:value={groupId}
            />

            <button
                class="join-button"
                class:active={isValid}
                on:click={joinGroup}
            >
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

<style src="./pastePR_ID.css"></style>