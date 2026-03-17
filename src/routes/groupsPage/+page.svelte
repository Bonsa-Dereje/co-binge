<script>
    import { onMount } from "svelte";

    let entering = true;
    let darkMode = false;
    let deviceId = "01D4TH879";

    let thumbnail = "/thumbnails/maxresdefault.jpg";

    onMount(() => {
        requestAnimationFrame(() => {
            entering = false;
        });
    });

    function toggleDarkMode() {
        darkMode = !darkMode;
        document.body.classList.toggle("dark-mode", darkMode);
    }
</script>

<div class="page-wrapper ytCard" class:entering={entering}>

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

        <!-- Party Profiles -->
        <div class="party-profiles">

            <div class="profile-stack">
                <img src="/avatars/girlAvatar.png" class="party-avatar" alt="user">
                <img src="/avatars/girlAvatar.png" class="party-avatar" alt="user">
                <img src="/avatars/girlAvatar.png" class="party-avatar" alt="user">
            </div>

            <div class="party-name">blueParty05D</div>

        </div>

        <!-- Thumbnail Card -->
        <div class="thumbnail-card">
            <img src={thumbnail} alt="video thumbnail" class="thumbnail-img">
        </div>

        <!-- Youtube Title -->
        <div class="yt-title">
            Unhinged Top Gear Moments #45
        </div>

        <!-- Views + Date -->
        <div class="yt-meta">

            <div class="yt-meta-item">
                <svg viewBox="0 0 24 24" class="meta-icon">
                    <path d="M2 12s4-6 10-6 10 6 10 6-4 6-10 6S2 12 2 12z"
                    fill="none" stroke="currentColor" stroke-width="1.5"/>
                    <circle cx="12" cy="12" r="3"
                    fill="none" stroke="currentColor" stroke-width="1.5"/>
                </svg>
                2.5k Views
            </div>

            <div class="yt-meta-item">
                <svg viewBox="0 0 24 24" class="meta-icon">
                    <rect x="3" y="4" width="18" height="18"
                    fill="none" stroke="currentColor" stroke-width="1.5"/>
                    <path d="M8 2v4M16 2v4M3 10h18"
                    fill="none" stroke="currentColor" stroke-width="1.5"/>
                </svg>
                3 days ago
            </div>

        </div>

        <!-- Host Button -->
        <div class="host-wrapper">
            <button class="host-button">Host</button>
        </div>

        <!-- Profile -->
        <div class="profile-wrapperYT">
            <img src="/avatars/girlAvatar.png" alt="avatar" class="profile-avatarDrag"/>
            <div class="profile-nameYT">user name</div>
            <div class="profile-deviceYT">device id: {deviceId}</div>
        </div>

    </div>

</div>

<style src="./groupsPage.css"></style>