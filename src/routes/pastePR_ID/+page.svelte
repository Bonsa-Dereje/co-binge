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

    // NEW STATES
    let loading = false;
    let paired = false;

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

            const text =
                (await navigator.clipboard.readText()).trim();

            if (text.length === REQUIRED_LENGTH) {

                groupId = text;
            }

        } catch (err) {

            console.warn("Clipboard access denied");
        }
    }

    function toggleDarkMode() {

        darkMode = !darkMode;

        document.body.classList.toggle(
            "dark-mode",
            darkMode
        );
    }

    async function joinGroup() {

        // prevent invalid click
        if (!isValid || loading || paired) {
            return;
        }

        loading = true;

        try {

            const result = await invoke("join_pairing");

            console.log(
                "✅ Pairing success:",
                result
            );

            loading = false;

            paired = true;

        } catch (err) {

            loading = false;

            console.error(
                "❌ Join failed:",
                err
            );
        }
    }

    $: isValid =
        groupId.length === REQUIRED_LENGTH;
</script>

<div
    class="page-wrapper join-page"
    class:entering={entering}
>

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

            <!-- INPUT + STATUS -->
            <div class="input-status-wrapper">

                <input
                    class="group-input"
                    placeholder="pairing id"
                    bind:value={groupId}
                />

                {#if loading}

                    <div class="spinner small-spinner"></div>

                {/if}

                {#if paired}

                    <div class="tick-wrapper">
                        ✓
                    </div>

                {/if}

            </div>

            <!-- JOIN BUTTON -->
            <button
                class="join-button"
                class:active={isValid}
                class:success={paired}
                on:click={joinGroup}
            >

                {#if paired}

                    PAIRED

                {:else}

                    Join

                {/if}

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