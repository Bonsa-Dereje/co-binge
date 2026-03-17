<script>
    import { onMount } from 'svelte';

    let entering = true;
    let darkMode = false;
    let deviceId = "01D4TH879";
    let groupId = "";

    onMount(() => {
        requestAnimationFrame(() => {
            entering = false;
        });
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
                user name
            </div>

            <div class="profile-device">
                device id: {deviceId}
            </div>

        </div>

    </div>

</div>


<style src="./pastePR_ID.css"></style>