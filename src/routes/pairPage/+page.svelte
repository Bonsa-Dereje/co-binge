<script lang="ts">
    import { onMount } from 'svelte';
    import { globalUserName, globalDeviceId } from '$lib/stores/user';

    let entering = true;
    let darkMode = false;

    let username = '';
    let deviceId = '';

    let copied = false;

    async function copyDeviceId() {
        try {
            await navigator.clipboard.writeText(deviceId);
            copied = true;

            setTimeout(() => {
                copied = false;
            }, 1500);
        } catch (err) {
            console.error("Copy failed:", err);
        }
    }

    function toggleDarkMode() {
        darkMode = !darkMode;
        document.body.classList.toggle("dark-mode", darkMode);
    }

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

        return () => {
            unsubscribeUser();
            unsubscribeDevice();
        };
    });
</script>

<div class="page-wrapper pairPage" class:entering={entering}>
    <div class="card">

        <!-- Toggle -->
        <div class="toggle-wrapper">
            <button 
                class="toggle"
                class:active={darkMode}
                on:click={toggleDarkMode}
            >
                <div class="toggle-circle"></div>
            </button>
        </div>

        <!-- Center Content -->
        <div class="pair-wrapper">

            <div class="title">This is your pairing ID</div>
            <div class="subtitle">Send code to client</div>

            <!-- Device Box -->
            <div class="device-box">
                <span class="device-id">{deviceId}</span>

                <button class="copy-btn" on:click={copyDeviceId}>
                    {copied ? 'Copied' : 'Copy'}
                </button>
            </div>

            <!-- Waiting Text -->
            <div class="waiting-wrapper">
                <span>pairing</span>
                <div class="spinner"></div>
            </div>

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

<style src="./pairPage.css"></style>