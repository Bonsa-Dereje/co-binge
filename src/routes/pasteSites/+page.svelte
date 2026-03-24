<script lang="ts">
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';
    import { globalUserName, globalDeviceId } from '$lib/stores/user';
    import { searchKeyword } from '$lib/stores/searchKeyword';

    let entering = true;
    let leaving = false;
    let darkMode = false;

    let username = '';
    let deviceId = '';

    let keyword = '';

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

    function toggleDarkMode() {
        darkMode = !darkMode;
        document.body.classList.toggle("dark-mode", darkMode);
    }

    function handleSearch() {
        if (!keyword.trim()) return;

        searchKeyword.set(keyword);

        leaving = true;

        setTimeout(() => {
            goto('/searchResults');
        }, 400);
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

        <!-- Center -->
        <div class="session-wrapper" class:leaving={leaving}>

            <div class="search-container">
                <input
                    type="text"
                    placeholder="Search movies..."
                    bind:value={keyword}
                    class="search-input"
                />

                <button
                    class="search-button"
                    on:click={handleSearch}
                    disabled={!keyword.trim()}
                >
                    Search
                </button>
            </div>

        </div>

        <!-- Bottom Profile -->
        <div class="profile-wrapper_pastePage">

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

<style src="./pasteSites.css"></style>