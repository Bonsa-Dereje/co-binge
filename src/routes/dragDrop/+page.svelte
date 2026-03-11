<script>
    import { onMount } from "svelte";

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

    function handleDrop(e) {
        e.preventDefault();
        const files = e.dataTransfer.files;
        console.log("Dropped files:", files);
    }

    function allowDrop(e) {
        e.preventDefault();
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
        <div
            class="drop-zone"
            on:drop={handleDrop}
            on:dragover={allowDrop}
        >

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
        <div class="profile-wrapper">
            <img src="/avatars/girlAvatar.png" alt="avatar" class="profile-avatar"/>
            <div class="profile-name">user name</div>
            <div class="profile-device">device id: {deviceId}</div>
        </div>

    </div>

</div>

<style src="./dragDrop.css"></style>