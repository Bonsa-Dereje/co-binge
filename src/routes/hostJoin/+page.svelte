<script>
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';

    let entering = true;
    let darkMode = false;
    let deviceId = "01D4TH879";

    onMount(() => {
        requestAnimationFrame(() => {
            entering = false;
        });
    });

    function toggleDarkMode() {
        darkMode = !darkMode;
        document.body.classList.toggle("dark-mode", darkMode);
    }

    function handleHostClick() {
        // navigate to chooseApp page
        goto('/chooseApp');
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
        <div class="session-wrapper">

            <button class="session-button" on:click={handleHostClick}>
                Host
            </button>

            <div class="session-divider"></div>

            <button class="session-button">
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
                user name
            </div>

            <div class="profile-device">
                device id: {deviceId}
            </div>

        </div>

    </div>

</div>

<style src="../style.css"></style>