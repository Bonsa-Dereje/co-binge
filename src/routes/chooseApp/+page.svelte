<script lang="ts">
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';

    let entering = true;
    let leaving = false;
    let darkMode = false;
    let deviceId = "01D4TH879";

    onMount(() => {
        requestAnimationFrame(() => {
            entering = false;
        });
    });

    function toggleDarkMode(): void {
        darkMode = !darkMode;
        document.body.classList.toggle("dark-mode", darkMode);
    }

    function handleAppClick(route: string): void {
        leaving = true;

        setTimeout(() => {
            goto(route);
        }, 500);
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

        <!-- 2x2 App Grid -->
        <div class="app-grid" class:leaving={leaving}>

            <button class="app-box" aria-label="Traffic Cone" on:click={() => handleAppClick('/dragDrop')}>
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <path d="M12 2L2 22h20L12 2z"/>
                    <path d="M12 6l-6 12h12l-6-12z"/>
                    <line x1="12" y1="10" x2="12" y2="14"/>
                    <line x1="10" y1="16" x2="14" y2="16"/>
                </svg>
            </button>

            <button class="app-box" aria-label="YouTube Card" on:click={() => handleAppClick('/youtubeCard')}>                <svg viewBox="0 0 24 24" fill="currentColor">
                    <path d="M23.498 6.186a3.016 3.016 0 0 0-2.122-2.136C19.505 3.545 12 3.545 12 3.545s-7.505 0-9.377.505A3.017 3.017 0 0 0 .502 6.186C0 8.07 0 12 0 12s0 3.93.502 5.814a3.016 3.016 0 0 0 2.122 2.136c1.871.505 9.376.505 9.376.505s7.505 0 9.377-.505a3.015 3.015 0 0 0 2.122-2.136C24 15.93 24 12 24 12s0-3.93-.502-5.814zM9.545 15.568V8.432L15.818 12l-6.273 3.568z"/>
                </svg>
            </button>

            <button class="app-box" aria-label="Globe" on:click={() => handleAppClick('/browser')}>
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <circle cx="12" cy="12" r="10"/>
                    <ellipse cx="12" cy="12" rx="4" ry="10"/>
                    <path d="M2 12h20"/>
                    <path d="M12 2c-2 3-3 7-3 10s1 7 3 10"/>
                    <path d="M12 2c2 3 3 7 3 10s-1 7-3 10"/>
                </svg>
            </button>

            <button class="app-box" aria-label="Netflix" on:click={() => handleAppClick('/netflix')}>
                <svg viewBox="0 0 24 24" fill="currentColor">
                    <path d="M5.398 0v.006c3.028 8.556 5.37 15.175 8.348 23.596 2.344.058 4.85.398 4.854.398-2.8-7.924-5.923-16.747-8.487-24zm8.489 0v9.63L18.6 22.951c-.043-7.86-.004-15.913.002-22.95zM5.398 1.05V24c1.873-.225 2.81-.312 4.715-.398v-9.22z"/>
                </svg>
            </button>

        </div>


        <!-- Bottom Profile -->
        <div class="profile-wrapper">
            <img src="/avatars/girlAvatar.png" alt="avatar" class="profile-avatar"/>
            <div class="profile-name">user name</div>
            <div class="profile-device">device id: {deviceId}</div>
        </div>

    </div>

</div>

<style src="../style.css"></style>