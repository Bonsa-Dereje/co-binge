<script lang="ts">
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';
    import { globalUserName, globalDeviceId } from '$lib/stores/user';
    import { ytLink } from '$lib/stores/ytLink';

    let entering = true;
    let leaving = false;
    let darkMode = false;

    let username = '';
    let deviceId = '';

    let clipboardLink = '';
    let isValidLink = false;

    function validateYouTubeLink(link: string): boolean {
        try {
            const url = new URL(link);
            return (
                url.hostname.includes('youtube.com') ||
                url.hostname.includes('youtu.be')
            );
        } catch {
            return false;
        }
    }

    async function readClipboard() {
        try {
            const text = await navigator.clipboard.readText();
            clipboardLink = text.trim();
            isValidLink = validateYouTubeLink(clipboardLink);
        } catch (err) {
            console.error('Clipboard read failed:', err);
            isValidLink = false;
        }
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

        readClipboard();
        window.addEventListener('focus', readClipboard);

        return () => {
            unsubscribeUser();
            unsubscribeDevice();
            window.removeEventListener('focus', readClipboard);
        };
    });

    function toggleDarkMode() {
        darkMode = !darkMode;
        document.body.classList.toggle("dark-mode", darkMode);
    }

    function handlePasteClick() {
        if (!isValidLink) return;

        ytLink.set(clipboardLink);

        leaving = true;

        setTimeout(() => {
            goto('/youtubeCard');
        }, 400);
    }
</script>

<div class="page-wrapper host-join-page ytPaste" class:entering={entering}>
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
        <div class="session-wrapper ytPaste" class:leaving={leaving}>

            <button 
                class="pasteButton ytPaste"
                class:disabled={!isValidLink}
                disabled={!isValidLink}
                on:click={handlePasteClick}
            >
                Paste Link
            </button>

        </div>

        <!-- Bottom Profile -->
        <div class="profile-wrapper_pastePage ytPaste">

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

<style src="./pastePage.css"></style>

