<script lang="ts">
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';

    let entering = true;
    let darkMode = false;
    let deviceId = "01D4TH879";

    type Group = {
        name: string;
        members: number;
    };

    const groups: Group[] = [
        { name: "blueParty05D", members: 2 },
        { name: "theDoodes", members: 5 },
        { name: "crocs", members: 5 }
    ];

    onMount(() => {
        requestAnimationFrame(() => {
            entering = false;
        });
    });

    function toggleDarkMode() {
        darkMode = !darkMode;
        document.body.classList.toggle("dark-mode", darkMode);
    }

    function openGroup(group: Group) {
        console.log("Open group:", group.name);
        goto('/groupsPage'); // Svelte way navigation
    }
</script>

<div class="page-wrapper groups-page" class:entering={entering}>

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

        <!-- Title -->
        <div class="groups-title">
            Groups
        </div>

        <!-- Group List -->
        <div class="groups-wrapper">

            {#each groups as group}
                <button class="group-card" on:click={() => openGroup(group)}>

                    <div class="group-name">
                        {group.name}
                    </div>

                    <div class="group-right">

                        <div class="group-avatars">
                            <img src="/avatars/girlAvatar.png" class="group-avatar"/>
                            <img src="/avatars/girlAvatar.png" class="group-avatar"/>
                            <img src="/avatars/girlAvatar.png" class="group-avatar"/>
                        </div>

                        <div class="group-members">
                            {group.members}
                            <span class="member-dot"></span>
                        </div>

                    </div>

                </button>
            {/each}

        </div>

        <!-- Create Button -->
        <button class="create-button">
            Create
        </button>

        <!-- Profile -->
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

<style src="./groupsList.css"></style>