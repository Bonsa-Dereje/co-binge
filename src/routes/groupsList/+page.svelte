<script lang="ts">
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';
    import { globalUserName, globalDeviceId } from '$lib/stores/user';

    let entering = true;
    let darkMode = false;
    let username = '';   // loaded from global store
    let deviceId = '';   // loaded from global store

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

        // Subscribe to global stores
        const unsubscribeUser = globalUserName.subscribe(value => {
            username = value || 'user name';
        });
        const unsubscribeDevice = globalDeviceId.subscribe(value => {
            deviceId = value || '01D4TH879';
        });

        // Cleanup subscriptions when component unmounts
        return () => {
            unsubscribeUser();
            unsubscribeDevice();
        };
    });

    function toggleDarkMode() {
        darkMode = !darkMode;
        document.body.classList.toggle("dark-mode", darkMode);
    }

    function openGroup(group: Group) {
        goto('/groupsPage');
    }

    function goToCreateGroup() {
        goto('/createGroup');
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
        <button class="create-button" on:click={goToCreateGroup}>
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
                {username}
            </div>

            <div class="profile-device">
                device id: {deviceId}
            </div>

        </div>

    </div>

</div>

<style src="./groupsList.css"></style>