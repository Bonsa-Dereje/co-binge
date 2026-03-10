<script>
    import { goto } from '$app/navigation';

    let userName = '';
    let deviceId = '01D4TH879';
    let darkMode = false;

    function handleCreateAccount() {
        console.log('Creating account for:', userName);

        // navigate to hostJoin page
        goto('/hostJoin');
    }

    function toggleDarkMode() {
        darkMode = !darkMode;
        document.body.classList.toggle('dark-mode', darkMode);
    }
</script>

<div class="page-wrapper">

    <!-- Card -->
    <div class="card">

        <!-- Toggle container -->
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

        <!-- Avatar container -->
        <div class="avatar-wrapper">
            <img src="/avatars/girlAvatar.png" alt="Avatar" class="avatar" />
        </div>

        <!-- Username container -->
        <div class="username-wrapper">
            <input 
                type="text"
                bind:value={userName}
                placeholder="user name"
                class="username-input"
            />
        </div>

        <!-- Device ID container -->
        <div class="device-wrapper">
            <p class="device-id">device id: {deviceId}</p>
        </div>

        <!-- Create Account container -->
        <div class="create-wrapper">
            <button 
                on:click={handleCreateAccount}
                class="create-button"
            >
                Create Account
            </button>
        </div>

    </div>

</div>

<style src="../style.css"></style>